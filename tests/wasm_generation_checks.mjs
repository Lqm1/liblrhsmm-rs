function assert(condition, message) {
    if (!condition) throw new Error(message);
}
function throws(action, message) {
    let failed = false;
    try { action(); } catch { failed = true; }
    assert(failed, message);
}
export async function verifyGenerationBindings(api, readFixture) {
    let comparisons = 0;
    let discrete = 0;
    let maximumError = 0;
    for (const [precision, filename, maximum] of [
        [32, 'c-generation.txt', 2147483647],
        [64, 'c-double-generation-32767.txt', 32767],
        [64, 'c-double-generation-2147483647.txt', 2147483647],
    ]) {
        const T = precision === 32 ? Float32Array : Float64Array;
        const scalar = precision === 32 ? Math.fround : value => value;
        const draw = index => scalar(scalar([
            Math.trunc(maximum / 8), Math.trunc(maximum / 4),
            Math.trunc(maximum / 2), Math.trunc(maximum / 4) * 3,
            Math.trunc(maximum / 8) * 7, maximum,
        ][index % 6]) / scalar(maximum));
        const text = new TextDecoder().decode(await readFixture(filename));
        const records = text.trim().split(/\r?\n/).map(line => line.split(/\s+/));
        const compare = (actual, expected, message) => {
            expected = scalar(expected);
            assert(Number.isFinite(actual) && Number.isFinite(expected), message);
            const difference = Math.abs(actual - expected) / Math.max(1, Math.abs(expected));
            maximumError = Math.max(maximumError, difference);
            assert(precision === 32 ? actual === expected : difference <= 2e-14, message);
            comparisons++;
        };
        for (let scenario = 0; scenario < 2; scenario++) {
            const model = new api[`ModelF${precision}`](new Uint32Array([2, 2, 2, 2, 1, 1]), 2);
            for (let stream = 0; stream < 2; stream++) {
                const parameters = model.stream(stream);
                for (let state = 0; state < 2; state++) {
                    const gaussian = parameters.emission(state);
                    const dimensions = stream === 0 ? 2 : 1;
                    const count = stream === 0 ? 4 : 1;
                    gaussian.replace(dimensions,
                        new T(stream === 0 ? [0.25, 0.75] : [1]),
                        new T(Array.from({length: count}, (_, index) => state + index * 0.5 + stream * 2)),
                        new T(Array.from({length: count}, (_, index) => scenario === 0 ? 0.25 + index * 0.25 : 0)),
                        gaussian.variance_floors());
                    parameters.set_emission(state, gaussian);
                    gaussian.free();
                }
                model.set_stream(stream, parameters);
                parameters.free();
            }
            for (let state = 0; state < 2; state++) {
                const duration = model.duration(state);
                duration.set_values(new T([scenario === 0 ? 2.5 - state : 0, scenario === 0 ? 0.75 - state * 0.5 : 0, duration.values()[2]]));
                const constraints = duration.constraints();
                constraints[0] = 9;
                duration.set_constraints(constraints);
                model.set_duration(state, duration);
                duration.free();
            }
            const sequence = new api[`SegmentationF${precision}`](2, 2);
            sequence.set_duration_states(new Int32Array([0, 1]));
            sequence.set_output_states(0, new Int32Array([0, 1]));
            sequence.set_output_states(1, new Int32Array([1, 0]));
            const values = new T(Array.from({length: 40}, (_, index) => draw(index)));
            values[39] = NaN; // Unused trailing draws are not inspected.
            const generated = model.generate_with_uniform_values(sequence, values);
            const copied = generated.cloned();
            const observation = generated.observation();
            generated.free();
            for (const fields of records) {
                const index = Number(fields[1]);
                if (fields[0] === 'gaussian' && scenario === 0) {
                    compare(api[`gaussian_sample_f${precision}`](-0.75, 1.25, draw(index), draw(index + 1)), Number(fields[2]), fields.join(' '));
                } else if (fields[0] === 'choice' && scenario === 0) {
                    assert(api[`choose_component_f${precision}`](new T([0.25, 0.5, 0.1]), draw(index)) === Number(fields[2]), 'component selection');
                    discrete++;
                } else if (fields[0] === 'boundaries' && index === scenario) {
                    assert(JSON.stringify([...sequence.boundaries()]) === JSON.stringify(fields.slice(2).map(Number)), 'sampled boundaries');
                    discrete++;
                } else if (fields[0] === 'draws' && index === scenario) {
                    assert(copied.draws_used === Number(fields[2]), 'native random draw consumption order');
                    discrete++;
                } else if (fields[0] === 'frame' && index === scenario) {
                    const actual = [...observation.frame(0, Number(fields[2])), ...observation.frame(1, Number(fields[2]))];
                    actual.forEach((value, offset) => compare(value, Number(fields[offset + 3]), fields.join(' ')));
                }
            }
            // Every possible exhaustion point preserves all existing boundaries.
            sequence.set_boundaries(new Int32Array([999, 999]));
            for (let length = 0; length < copied.draws_used; length++) {
                throws(() => model.generate_with_uniform_values(sequence, values.subarray(0, length)), 'exhaustion must fail');
                assert([...sequence.boundaries()].every(value => value === 999), 'exhaustion changed segmentation');
            }
            const invalid = values.slice();
            invalid[5] = NaN;
            throws(() => model.generate_with_uniform_values(sequence, invalid), 'late invalid draw must fail');
            assert([...sequence.boundaries()].every(value => value === 999), 'invalid draw changed segmentation');
            const transferred = copied.into_observation();
            model.free(); sequence.free(); observation.free();
            assert(transferred.frames === (scenario === 0 ? 4 : 2), 'consumed result retains observation');
            transferred.free();
        }
        throws(() => api[`choose_component_f${precision}`](new T([]), 0.5), 'empty weights');
        throws(() => api[`gaussian_sample_f${precision}`](0, 1, 0, 0.5), 'zero radial');
        assert(api[`choose_component_f${precision}`](new T([0, 0]), 1) === 1, 'last component fallback');
        assert(api[`choose_component_f${precision}`](new T([2, 3]), 1) === 0, 'weights are not normalized');
        assert(api[`choose_component_f${precision}`](new T([0.5, 0.5]), 0.5) === 1, 'strict cumulative threshold');
        for (const invalid of [-1, NaN, Infinity]) {
            throws(() => api[`choose_component_f${precision}`](new T([invalid, 1]), 0.5), 'invalid mixture weight');
            throws(() => api[`gaussian_sample_f${precision}`](0, 1, invalid, 0.5), 'invalid radial draw');
        }
        if (precision === 64) {
            assert(api.choose_component_f64(new T([0.5 + Number.EPSILON, 0.5]), 0.5) === 0, 'double cumulative precision');
            assert(api.gaussian_sample_f64(1 + Number.EPSILON, 0, 0.5, 0.5) === 1 + Number.EPSILON, 'double scalar precision');
        }
    }
    assert(comparisons === 72 && discrete === 30, 'complete C generation corpora');
    return {comparisons, discrete, maximumError};
}
