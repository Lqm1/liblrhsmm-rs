function assert(value, message) { if (!value) throw new Error(message); }
function equal(actual, expected, message) { assert(Object.is(actual, expected), message + ': ' + actual + ' != ' + expected); }
function rejects(action, message) { let caught = false; try { action(); } catch { caught = true; } assert(caught, message); }
function array(actual, expected, message) { equal(actual.length, expected.length, message + ' length'); for (let i = 0; i < actual.length; i++) equal(actual[i], expected[i], message + '[' + i + ']'); }

export async function verifyEmissionBindings(api, load) {
    let logRecords = 0, densityRecords = 0, modeChecks = 0, maximumNormalizedError = 0;
    for (const single of [true, false]) {
        const suffix = single ? 'F32' : 'F64', A = single ? Float32Array : Float64Array, B = single ? Uint32Array : BigUint64Array;
        const M = api['Model' + suffix], O = api['Observation' + suffix], S = api['Segmentation' + suffix], P = api['EmissionPruning' + suffix], L = api['LogEmissions' + suffix];
        const model = new M(new Uint32Array([3, 2, 2, 3, 1, 1]), 3), observation = new O(12, new Uint32Array([2, 1])), segmentation = new S(2, 3);
        for (let stream = 0; stream < 2; stream++) {
            const value = model.stream(stream); value.set_weight(new A([stream === 0 ? .5 : 1.25]));
            for (let state = 0; state < 3; state++) {
                const g = value.emission(state), count = stream === 0 ? 4 : 1;
                g.replace(stream === 0 ? 2 : 1, g.weights(), new A(Array.from({length:count}, (_, i) => state * .5 + i * .25)), new A(Array.from({length:count}, (_, i) => .75 + i * .25)), g.variance_floors());
                value.set_emission(state, g); g.free();
            }
            model.set_stream(stream, value); value.free();
            const features = observation.stream(stream), dimensions = stream === 0 ? 2 : 1;
            features.replace(dimensions, new A(Array.from({length:12 * dimensions}, (_, i) => Math.floor(i / dimensions) * .125 + (i % dimensions) * .5)));
            observation.set_stream(stream, features); features.free();
        }
        for (let state = 0; state < 3; state++) { const d = model.duration(state); d.set_values(new A([4, 1, 0])); model.set_duration(state, d); d.free(); }
        segmentation.set_boundaries(new Int32Array([4, 8, 12])); segmentation.set_duration_states(new Int32Array([0, 1, 2])); segmentation.set_output_states(0, new Int32Array([0, 1, 2])); segmentation.set_output_states(1, new Int32Array([2, 1, 0]));
        const options = new api['PreparationOptions' + suffix](); options.duration_cache_length = 2; const prepared = model.prepare(options); options.free();
        const modes = [[0, NaN], [1, .25], [1, .5], [1, 1.75], [1, 2.5], [1, 1], [2, .34]];
        for (const temperature of [0, .75, 1.75, single ? 1 + 2 ** -23 : 1 + Number.EPSILON]) for (const [mode, parameter] of modes) {
            const pruning = new P(mode, parameter), clone = pruning.cloned(); equal(clone.mode, mode, 'copied pruning mode'); equal(clone.parameter, new A([parameter])[0], 'selected pruning parameter'); clone.free();
            const log = model.emission_log_probabilities(observation, segmentation, temperature, pruning), cached = prepared.emission_log_probabilities(observation, segmentation, temperature, pruning), density = model.emission_probabilities(observation, segmentation, temperature, pruning), cachedDensity = prepared.emission_probabilities(observation, segmentation, temperature, pruning), converted = log.probabilities();
            array(new B(log.values().buffer), new B(cached.values().buffer), 'prepared exact log bits'); array(new B(density.values().buffer), new B(cachedDensity.values().buffer), 'prepared exact density bits'); array(new B(density.values().buffer), new B(converted.values().buffer), 'converted exact density bits');
            for (const owner of [log, density]) { equal(owner.frames, 12, 'table frames'); equal(owner.states, 3, 'table states'); equal(owner.get(12, 0), undefined, 'frame range'); equal(owner.get(0, 3), undefined, 'state range'); equal(owner.get(0xffffffff, 0xffffffff), undefined, 'large range'); const values = owner.values(), copy = owner.cloned(); values.fill(42); assert(owner.get(0, 0) !== 42, 'returned array detached'); array(new B(copy.values().buffer), new B(owner.values().buffer), 'copied table storage'); copy.free(); }
            for (const owner of [log, cached, density, cachedDensity, converted, pruning]) owner.free(); modeChecks++;
        }
        for (const [filename, fractional] of [[single ? 'c-emission-tables.txt' : 'c-double-emissions.txt', false], [single ? 'c-fractional-emissions.txt' : 'c-double-emissions-fractional.txt', true]]) {
            const lines = new TextDecoder().decode(await load(filename)).trim().split(/\r?\n/), tables = new Map();
            for (const line of lines) {
                const [scenario, index, expected] = line.trim().split(/\s+/).map(Number);
                if (!tables.has(scenario)) { const pruning = new P(fractional || scenario === 0 ? 1 : 2, fractional ? [.25, .5, 1.75, 2.5][scenario] : scenario === 0 ? 1 : .34); tables.set(scenario, prepared.emission_log_probabilities(observation, segmentation, .75, pruning)); pruning.free(); }
                const actual = tables.get(scenario).get(Math.floor(index / 3), index % 3), reference = new A([expected])[0], error = Math.abs(actual - reference) / Math.max(1, Math.abs(reference));
                if (single) equal(actual, reference, 'original float C log'); else assert(error <= 2e-14, 'original double C log bound'); maximumNormalizedError = Math.max(maximumNormalizedError, error); logRecords++;
            }
            for (const owner of tables.values()) owner.free();
        }
        const pruning = new P(1, 1), density = model.emission_probabilities(observation, segmentation, .75, pruning);
        if (single) for (const line of new TextDecoder().decode(await load('c-probability-forms.txt')).trim().split(/\r?\n/).filter(line => line.startsWith('emission '))) { const [, index, expected] = line.split(/\s+/); equal(density.values()[Number(index)], Math.fround(Number(expected)), 'original C density'); densityRecords++; }
        assert(density.get(0, 2) > 0, 'original approximate pruned underflow'); density.free();
        pruning.mode = 99; rejects(() => model.emission_log_probabilities(observation, segmentation, 1, pruning), 'invalid mode'); pruning.mode = 2;
        for (const parameter of [-1, Infinity, NaN]) { pruning.parameter = parameter; rejects(() => prepared.emission_log_probabilities(observation, segmentation, 1, pruning), 'invalid slope'); }
        pruning.mode = 1;
        for (const parameter of [-1, Infinity, NaN]) { pruning.parameter = parameter; rejects(() => model.emission_log_probabilities(observation, segmentation, 1, pruning), 'invalid radius'); }
        pruning.parameter = 1;
        for (const temperature of [-1, Infinity, NaN]) rejects(() => prepared.emission_probabilities(observation, segmentation, temperature, pruning), 'invalid temperature');
        segmentation.set_boundaries(new Int32Array([4, 8, 11])); rejects(() => model.emission_log_probabilities(observation, segmentation, 1, pruning), 'incomplete boundaries'); segmentation.set_boundaries(new Int32Array([4, 8, 12]));
        segmentation.set_output_states(0, new Int32Array([-1, 1, 2])); rejects(() => prepared.emission_probabilities(observation, segmentation, 1, pruning), 'negative output state'); segmentation.set_output_states(0, new Int32Array([0, 1, 2]));
        const result = model.emission_log_probabilities(observation, segmentation, .75, pruning), retained = result.cloned(), saved = result.values(); result.free(); model.free(); prepared.free(); observation.free(); segmentation.free(); pruning.free(); array(new B(retained.values().buffer), new B(saved.buffer), 'table outlives sources'); retained.free();
        for (const [frames, states, values] of [[0, 1, [0]], [1, 0, []], [2, 2, [0]], [0xffffffff, 0xffffffff, []], [1, 1, [NaN]], [1, 1, [Infinity]]]) rejects(() => new L(frames, states, new A(values)), 'invalid external log table');
        const externalValues = new A([-0, single ? -1e15 : -1e300, single ? 2 ** -23 : Number.EPSILON, 1]), external = new L(2, 2, externalValues); array(new B(external.values().buffer), new B(externalValues.buffer), 'external precision bits'); externalValues.fill(7); equal(external.get(0, 0), -0, 'signed zero retained'); const externalDensity = external.probabilities(); equal(externalDensity.frames, 2, 'external density shape'); external.free(); assert(externalDensity.get(0, 1) > 0, 'external original clamp'); externalDensity.free();
    }
    equal(logRecords, 432, 'complete C log corpus'); equal(densityRecords, 36, 'complete C density corpus'); equal(modeChecks, 56, 'complete modes/temperatures/precisions');
    return {logRecords, densityRecords, modeChecks, maximumNormalizedError};
}
