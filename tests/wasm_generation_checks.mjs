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
    let callbackRuns = 0;
    let wordComparisons = 0;
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
            const request = new api[`GenerationF${precision}`](model, sequence);
            let callbackDraws = 0;
            const live = request.generate_with_uniform(() => draw(callbackDraws++));
            assert(live.draws_used === copied.draws_used && callbackDraws === copied.draws_used, 'live callback consumption order');
            assert(live.random_words_used === 0 && copied.random_words_used === 0, 'uniform paths use no raw words');
            const liveObservation = live.into_observation();
            for (let stream = 0; stream < 2; stream++) {
                const original = observation.stream(stream);
                const sampled = liveObservation.stream(stream);
                assert(JSON.stringify([...original.values()]) === JSON.stringify([...sampled.values()]), 'live callback observation matches C-controlled draws');
                original.free(); sampled.free();
            }
            const liveSequence = request.sequence();
            assert(JSON.stringify([...liveSequence.boundaries()]) === JSON.stringify([...sequence.boundaries()]), 'live callback boundaries');
            liveSequence.free(); liveObservation.free();
            callbackRuns++;
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
            request.set_sequence(sequence);
            const invalidModel = model.cloned();
            const invalidDuration = invalidModel.duration(0);
            invalidDuration.set_values(new T([NaN, 1, 0]));
            invalidModel.set_duration(0, invalidDuration);
            invalidDuration.free();
            request.set_model(invalidModel);
            invalidModel.free();
            let validationCalls = 0;
            throws(() => request.generate_with_uniform(() => {validationCalls++; return 0.5;}), 'model prevalidation');
            throws(() => request.generate_with_random_words(() => {validationCalls++; return 0;}), 'raw model prevalidation');
            assert(validationCalls === 0, 'invalid model consumed external randomness');
            request.set_model(model);
            for (const failureIndex of [0, 1, 5, copied.draws_used - 1]) {
                const marker = {failureIndex};
                let calls = 0;
                let failure;
                try {
                    request.generate_with_uniform(() => {
                        const index = calls++;
                        if (index === failureIndex) throw marker;
                        return draw(index);
                    });
                } catch (error) { failure = error; }
                assert(failure === marker && calls === failureIndex + 1, 'callback preserves exception identity and stops calling');
                const unchanged = request.sequence();
                assert([...unchanged.boundaries()].every(value => value === 999), 'throw changed owned segmentation');
                unchanged.free();
            }
            for (const value of [undefined, null, '0.5', {}, NaN, Infinity, -1]) {
                throws(() => request.generate_with_uniform(() => value), 'invalid callback value');
            }
            let reentrantCalls = 0;
            const protectedResult = request.generate_with_uniform(() => {
                throws(() => request.sequence(), 'reentrant sequence access');
                throws(() => request.model(), 'reentrant model access');
                throws(() => request.cloned(), 'reentrant clone');
                throws(() => request.set_model(model), 'reentrant model replacement');
                throws(() => request.set_sequence(sequence), 'reentrant sequence replacement');
                throws(() => request.generate_with_uniform(() => 0.5), 'reentrant uniform generation');
                throws(() => request.generate_with_random_words(() => 0), 'reentrant raw generation');
                return draw(reentrantCalls++);
            });
            assert(protectedResult.draws_used === copied.draws_used, 'caught reentrancy retains generator usability');
            protectedResult.free();
            throws(() => request.generate_with_uniform(() => request.sequence()), 'uncaught reentrancy must fail');
            // A caught nested borrow error must not poison the generated ABI guard.
            let recoveredIndex = 0;
            const recovered = request.generate_with_uniform(() => draw(recoveredIndex++));
            recovered.free();
            const independent = request.cloned();
            let independentIndex = 0;
            const nestedResult = request.generate_with_uniform(() => {
                if (independentIndex === 0) {
                    const child = independent.generate_with_uniform(() => 0.5);
                    child.free();
                }
                return draw(independentIndex++);
            });
            nestedResult.free(); independent.free();
            // Independently convert arbitrary raw words using the documented grids.
            let state = 0x12345678;
            const words = Array.from({length: 256}, () => {
                state = (Math.imul(state, 1664525) + 1013904223) >>> 0;
                return state;
            });
            const converted = [];
            for (let index = 0; index < words.length; index += precision === 32 ? 1 : 2) {
                converted.push(precision === 32 ? ((words[index] >>> 8) + 1) / 2 ** 24
                    : Number(((BigInt(words[index + 1]) << 32n | BigInt(words[index])) >> 11n) + 1n) / 2 ** 53);
            }
            const expected = model.generate_with_uniform_values(sequence, new T(converted));
            let wordsUsed = 0;
            const raw = request.generate_with_random_words(() => {
                throws(() => request.sequence(), 'raw callback reentrant access');
                throws(() => request.generate_with_random_words(() => 0), 'raw callback reentrant generation');
                return words[wordsUsed++];
            });
            assert(raw.draws_used === expected.draws_used && raw.random_words_used === wordsUsed
                && wordsUsed === raw.draws_used * (precision === 32 ? 1 : 2), 'raw word consumption and conversion');
            const expectedObservation = expected.into_observation();
            const rawObservation = raw.into_observation();
            assert(rawObservation.frames === expectedObservation.frames, 'raw generated frame count');
            for (let stream = 0; stream < 2; stream++) {
                const left = expectedObservation.stream(stream);
                const right = rawObservation.stream(stream);
                assert(JSON.stringify([...left.values()]) === JSON.stringify([...right.values()]), 'raw generation matches native uniform grid');
                left.free(); right.free();
            }
            expectedObservation.free(); rawObservation.free();
            const rawMarker = {raw: true};
            let rawCalls = 0;
            let rawFailure;
            const prior = request.sequence();
            try {
                request.generate_with_random_words(() => {
                    if (rawCalls++ === 5) throw rawMarker;
                    return 0xffffffff;
                });
            } catch (error) { rawFailure = error; }
            assert(rawFailure === rawMarker && rawCalls === 6, 'raw exception identity and consumption');
            const after = request.sequence();
            assert(JSON.stringify([...prior.boundaries()]) === JSON.stringify([...after.boundaries()]), 'raw error atomicity');
            prior.free(); after.free();
            for (const word of [-1, 0.5, 2 ** 32, NaN, Infinity, undefined, '1']) {
                throws(() => request.generate_with_random_words(() => word), 'invalid raw word');
            }
            const detached = request.cloned();
            const requestModel = request.model();
            requestModel.free();
            request.free();
            const transferred = copied.into_observation();
            model.free(); sequence.free(); observation.free();
            assert(transferred.frames === (scenario === 0 ? 4 : 2), 'consumed result retains observation');
            transferred.free();
            let detachedIndex = 0;
            const detachedResult = detached.generate_with_uniform(() => draw(detachedIndex++));
            detachedResult.free(); detached.free();
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
        let wordState = 0x87654321;
        for (let index = 0; index < 4096; index++) {
            const words = index === 0 ? [0, 0] : index === 1 ? [0xffffffff, 0xffffffff] : Array.from({length: 2}, () => {
                wordState = (Math.imul(wordState, 1664525) + 1013904223) >>> 0;
                return wordState;
            });
            let count = 0;
            const actual = api[`uniform_sample_f${precision}`](() => words[count++]);
            const expected = precision === 32 ? ((words[0] >>> 8) + 1) / 2 ** 24
                : Number(((BigInt(words[1]) << 32n | BigInt(words[0])) >> 11n) + 1n) / 2 ** 53;
            assert(actual === expected && count === (precision === 32 ? 1 : 2), 'standalone native uniform grid');
            wordComparisons++;
        }
        const marker = {standalone: precision};
        let caught;
        try { api[`uniform_sample_f${precision}`](() => {throw marker;}); } catch (error) {caught = error;}
        assert(caught === marker, 'standalone uniform exception identity');
    }
    assert(comparisons === 72 && discrete === 30, 'complete C generation corpora');
    assert(callbackRuns === 6 && wordComparisons === 12288, 'complete live generation and raw grid checks');
    return {comparisons, discrete, callbackRuns, wordComparisons, maximumError};
}
