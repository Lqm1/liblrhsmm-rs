import {verifyPosteriorStorage} from './native_posterior_storage_checks.mjs';
import {verifyDurationAndMixturePosteriors} from './native_posterior_numeric_checks.mjs';
function assert(value, message) { if (!value) throw new Error(message); }
function equal(actual, expected, message) { assert(Object.is(actual, expected), message + ': ' + actual + ' != ' + expected); }
function array(actual, expected, message) { equal(actual.length, expected.length, message + ' length'); for (let i = 0; i < actual.length; i++) equal(actual[i], expected[i], message + '[' + i + ']'); }
function rejects(action, message) { let caught = false; try { action(); } catch { caught = true; } assert(caught, message); }

export async function verifyPosteriorBindings(api, load) {
    const storage = verifyPosteriorStorage(api);
    let geometricRecords = 0, transitionValues = 0, maximumNormalizedError = 0;
    for (const single of [true, false]) {
        const suffix = single ? 'F32' : 'F64', A = single ? Float32Array : Float64Array, B = single ? Uint32Array : BigUint64Array, cases = new Map();
        for (const line of new TextDecoder().decode(await load(single ? 'c-geometric.txt' : 'c-double-geometric-posteriors.txt')).trim().split(/\r?\n/).filter(line => line.trim().startsWith('occupancy '))) {
            const fields = line.trim().split(/\s+/), [states, frames, temperature, slope, index] = fields.slice(1, 6).map(Number), key = fields.slice(1, 5).join(' ');
            if (!cases.has(key)) {
                const model = new api['Model' + suffix](new Uint32Array(), states), segmentation = new api['Segmentation' + suffix](0, states), emissions = new api['LogEmissions' + suffix](frames, states, new A(Array.from({length:frames * states}, (_, i) => { const residual = Math.floor(i / states) - i % states * 4; return -residual * residual * .25; }))), options = new api['GeometricOptions' + suffix]();
                for (let state = 0; state < states; state++) { const duration = model.duration(state); duration.set_values(new A([[.5, 2.75, 5][state], 1, 0])); model.set_duration(state, duration); duration.free(); }
                segmentation.set_duration_states(new Int32Array(Array.from({length:states}, (_, i) => i))); options.temperature = temperature; options.pruning_slope = slope;
                const forward = model.forward_geometric(segmentation, emissions, options), backward = model.backward_geometric(segmentation, emissions, options), total = backward.backward_log_probability(emissions), occupancy = forward.geometric_occupancies(backward, total, slope), transitions = model.geometric_transition_occupancies(segmentation, emissions, forward, backward, total, options);
                cases.set(key, {model, segmentation, emissions, options, forward, backward, occupancy, transitions, total});
            }
            const value = cases.get(key), frame = Math.floor(index / states), state = index % states, expected = new A([Number(fields[6])])[0], actual = value.occupancy.get(frame, state), error = Math.abs(actual - expected) / Math.max(1, Math.abs(expected)); if (single) assert(actual === expected, 'original C state occupancy'); else assert(error <= 2e-14, 'original double state occupancy bound'); maximumNormalizedError = Math.max(maximumNormalizedError, error); geometricRecords++;
            const child = value.transitions.get(frame, state), present = Number(fields[7]) !== 0; equal(child !== null, present, 'original C transition presence'); if (child) { const values = child.values(); for (let i = 0; i < 2; i++) { assert(values[i] === Math.fround(Number(fields[8 + i])), 'original C transition weight'); transitionValues++; } child.free(); }
        }
        for (const value of cases.values()) {
            const {occupancy, transitions, forward, backward, emissions, model, segmentation, options, total} = value;
            equal(occupancy.frames, emissions.frames, 'state occupancy frames'); equal(occupancy.states, emissions.states, 'state occupancy states'); equal(transitions.frames, emissions.frames, 'transition frames'); equal(transitions.states, emissions.states, 'transition states');
            const stateCopy = occupancy.cloned(), transitionCopy = transitions.cloned(), importedState = new api['StateOccupancies' + suffix](occupancy.frames, occupancy.states, occupancy.values()), importedTransition = new api.TransitionPosteriors(transitions.frames, transitions.states, transitions.presence(), transitions.values()); array(new B(importedState.values().buffer), new B(occupancy.values().buffer), 'external computed state bits'); array(importedTransition.presence(), transitions.presence(), 'external computed transition presence'); array(new Uint32Array(importedTransition.values().buffer), new Uint32Array(transitions.values().buffer), 'external computed transition bits'); importedState.free(); importedTransition.free();
            rejects(() => forward.geometric_occupancies(backward, NaN, options.pruning_slope), 'invalid state normalizer'); rejects(() => forward.geometric_occupancies(backward, total, -1), 'invalid state slope'); rejects(() => model.geometric_transition_occupancies(segmentation, emissions, forward, backward, Infinity, options), 'invalid transition normalizer');
            const stateValues = occupancy.values(), transitionPresence = transitions.presence(), transitionArray = transitions.values(); for (const key of ['model', 'segmentation', 'emissions', 'options', 'forward', 'backward', 'occupancy', 'transitions']) value[key].free(); array(new B(stateCopy.values().buffer), new B(stateValues.buffer), 'state posterior survives input release'); array(transitionCopy.presence(), transitionPresence, 'transition presence survives release'); array(new Uint32Array(transitionCopy.values().buffer), new Uint32Array(transitionArray.buffer), 'transition values survive release'); stateCopy.free(); transitionCopy.free();
        }
    }
    equal(geometricRecords, 292, 'complete original geometric posterior corpus');
    const durationAndMixture = await verifyDurationAndMixturePosteriors(api, load);
    return {storage, geometricRecords, transitionValues, maximumNormalizedError, durationAndMixture};
}
