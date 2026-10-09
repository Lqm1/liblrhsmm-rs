function assert(value, message) { if (!value) throw new Error(message); }
function equal(actual, expected, message) { assert(Object.is(actual, expected), message + ': ' + actual + ' != ' + expected); }
function array(actual, expected, message) { equal(actual.length, expected.length, message + ' length'); for (let i = 0; i < actual.length; i++) equal(actual[i], expected[i], message + '[' + i + ']'); }
function rejects(action, message) { let caught = false; try { action(); } catch { caught = true; } assert(caught, message); }

export async function verifyGeometricBindings(api, load) {
    let numericValues = 0, boundaries = 0, maximumNormalizedError = 0;
    for (const single of [true, false]) {
        const suffix = single ? 'F32' : 'F64', A = single ? Float32Array : Float64Array, B = single ? Uint32Array : BigUint64Array;
        const M = api['Model' + suffix], S = api['Segmentation' + suffix], L = api['LogEmissions' + suffix], O = api['GeometricOptions' + suffix], T = api['StateScores' + suffix], P = api['GeometricPath' + suffix];
        const cases = new Map();
        function sample(states, frames, temperature, slope) {
            const model = new M(new Uint32Array(), states), segmentation = new S(0, states), emissions = new L(frames, states, new A(Array.from({length:frames * states}, (_, i) => { const residual = Math.floor(i / states) - i % states * 4; return -residual * residual * .25; }))), options = new O();
            for (let state = 0; state < states; state++) { const d = model.duration(state); d.set_values(new A([[.5, 2.75, 5][state], 1, 0])); model.set_duration(state, d); d.free(); }
            segmentation.set_duration_states(new Int32Array(Array.from({length:states}, (_, i) => i))); options.temperature = temperature; options.pruning_slope = slope;
            return {model, segmentation, emissions, options};
        }
        for (const line of new TextDecoder().decode(await load(single ? 'c-geometric.txt' : 'c-double-geometric.txt')).trim().split(/\r?\n/)) {
            const fields = line.trim().split(/\s+/), kind = fields[0];
            if (kind === 'math' || kind === 'occupancy') continue;
            assert(['scores', 'total', 'boundary'].includes(kind), 'known inference record');
            const [states, frames, temperature, slope, index] = fields.slice(1, 6).map(Number), key = fields.slice(1, 5).join(' ');
            if (!cases.has(key)) {
                const value = sample(states, frames, temperature, slope);
                value.forward = value.model.forward_geometric(value.segmentation, value.emissions, value.options); value.backward = value.model.backward_geometric(value.segmentation, value.emissions, value.options); value.path = value.model.viterbi_geometric(value.segmentation, value.emissions, value.options); value.pathScores = value.path.scores();
                const clone = value.options.cloned(); equal(clone.temperature, new A([temperature])[0], 'copied temperature'); equal(clone.pruning_slope, new A([slope])[0], 'copied slope'); clone.free();
                cases.set(key, value);
            }
            const value = cases.get(key);
            if (kind === 'boundary') { equal(value.path.boundaries()[index], Number(fields[6]), 'original C discrete boundary'); boundaries++; }
            else {
                const actual = kind === 'scores' ? [value.forward.values()[index], value.backward.values()[index], value.pathScores.values()[index]] : [value.forward.total_through_state(value.backward, index), value.forward.total_log_probability(), value.backward.backward_log_probability(value.emissions)];
                for (let i = 0; i < 3; i++) { const expected = new A([Number(fields[6 + i])])[0], error = Math.abs(actual[i] - expected) / Math.max(1, Math.abs(expected)); if (single) assert(actual[i] === expected, 'original C float score'); else assert(error <= 2e-14, 'original C double bound'); maximumNormalizedError = Math.max(maximumNormalizedError, error); numericValues++; }
            }
        }
        for (const value of cases.values()) {
            const {model, segmentation, emissions, options, forward, backward, path, pathScores} = value;
            for (const table of [forward, backward, pathScores]) { equal(table.frames, emissions.frames, 'complete frames'); equal(table.states, emissions.states, 'complete states'); equal(table.get(table.frames, 0), undefined, 'out of frame'); equal(table.get(0, table.states), undefined, 'out of state'); const values = table.values(), clone = table.cloned(); values.fill(42); array(new B(clone.values().buffer), new B(table.values().buffer), 'independent score array'); clone.free(); }
            const states = path.states(), ends = path.boundaries(); equal(states.length, emissions.frames, 'complete path length'); equal(states[0], 0, 'path begins at zero'); equal(states.at(-1), emissions.states - 1, 'path final state'); equal(path.log_probability()[0], pathScores.values().at(-1), 'path terminal score');
            for (let state = 0; state < ends.length; state++) equal(ends[state], states.findLastIndex(value => value === state) + 1, 'frame path boundary consistency');
            const pathCopy = path.cloned(), retainedScores = pathCopy.scores(); for (const object of [model, segmentation, emissions, options, forward, backward, path, pathScores]) object.free(); array(pathCopy.states(), states, 'path outlives input'); array(pathCopy.boundaries(), ends, 'boundaries outlive input'); const anotherScores = pathCopy.scores(); array(new B(anotherScores.values().buffer), new B(retainedScores.values().buffer), 'path copied score snapshot'); anotherScores.free(); retainedScores.free(); pathCopy.free();
        }
        const bits = single ? new Uint32Array([0xffc00123, 0x80000000, 0x7f800000, 0xff800000]) : new BigUint64Array([0xfff8000000000123n, 0x8000000000000000n, 0x7ff0000000000000n, 0xfff0000000000000n]), raw = new A(bits.buffer), table = new T(2, 2, raw);
        array(new B(table.values().buffer), bits, 'import all raw IEEE score bits'); const tableClone = table.cloned(); table.free(); array(new B(tableClone.values().buffer), bits, 'cloned IEEE score bits');
        const probability = new A(bits.buffer.slice(0, A.BYTES_PER_ELEMENT)), imported = new P(new Uint32Array([0xffffffff, 0]), new Uint32Array([0xffffffff]), probability, tableClone); array(new B(imported.log_probability().buffer), new B(probability.buffer), 'path raw probability bits'); imported.set_states(new Uint32Array([2, 2, 1])); imported.set_boundaries(new Uint32Array([0, 0xffffffff])); array(imported.states(), [2, 2, 1], 'complete editable states'); array(imported.boundaries(), [0, 0xffffffff], 'complete editable boundaries'); rejects(() => imported.set_log_probability(new A()), 'invalid probability shape'); array(new B(imported.log_probability().buffer), new B(probability.buffer), 'probability failed edit atomic'); imported.set_log_probability(new A([-0])); equal(imported.log_probability()[0], -0, 'editable signed zero'); imported.set_scores(tableClone); tableClone.free(); const importedScores = imported.scores(); array(new B(importedScores.values().buffer), bits, 'copied editable scores'); importedScores.free(); imported.free();
        for (const [frames, states, values] of [[0, 1, []], [1, 0, []], [2, 2, [0]], [0xffffffff, 0xffffffff, []]]) rejects(() => new T(frames, states, new A(values)), 'invalid score shape');
        const input = sample(3, 12, 1, 1), forward = input.model.forward_geometric(input.segmentation, input.emissions, input.options), wrong = new T(1, 1, new A([0])), wrongEmissions = new L(1, 1, new A([0]));
        rejects(() => forward.total_through_state(wrong, 0), 'mismatched backward table'); rejects(() => forward.total_through_state(forward, 3), 'total state index'); rejects(() => forward.backward_log_probability(wrongEmissions), 'mismatched emission dimensions'); wrong.free(); wrongEmissions.free();
        for (const bad of [-1, NaN, Infinity]) { input.options.temperature = bad; rejects(() => input.model.forward_geometric(input.segmentation, input.emissions, input.options), 'invalid temperature'); input.options.temperature = 1; input.options.pruning_slope = bad; rejects(() => input.model.backward_geometric(input.segmentation, input.emissions, input.options), 'invalid slope'); input.options.pruning_slope = 1; }
        input.segmentation.set_duration_states(new Int32Array([-1, 1, 2])); rejects(() => input.model.viterbi_geometric(input.segmentation, input.emissions, input.options), 'invalid duration state'); for (const owner of [...Object.values(input), forward]) owner.free();
        const impossible = sample(3, 1, 1, 1); rejects(() => impossible.model.viterbi_geometric(impossible.segmentation, impossible.emissions, impossible.options), 'impossible complete path'); for (const owner of Object.values(impossible)) owner.free();
        const defaults = new O(); equal(defaults.temperature, 1, 'default temperature'); equal(defaults.pruning_slope, new A([.3])[0], 'default slope'); defaults.free();
    }
    equal(numericValues, 960, 'complete inference C numeric corpus'); equal(boundaries, 28, 'complete inference C boundaries');
    return {numericValues, boundaries, maximumNormalizedError};
}
