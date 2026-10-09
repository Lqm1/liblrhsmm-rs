function assert(condition, message) {
    if (!condition) throw new Error(message);
}
function equal(actual, expected, message) {
    assert(JSON.stringify(actual) === JSON.stringify(expected), message);
}
export function verifyOwnershipBindings(api) {
    let cycles = 0;
    for (const suffix of ['F32', 'F64']) {
        const T = suffix === 'F32' ? Float32Array : Float64Array;
        for (let cycle = 0; cycle < 64; cycle++) {
            const duration = new api['Duration' + suffix]();
            duration.set_values(new T([3, 2, 0.25]));
            duration.set_constraints(new Int32Array([1, 8, -1]));
            const options = new api['PreparationOptions' + suffix]();
            options.duration_cache_length = 12;
            options.duration_weight = 0.75;
            options.temperature = 0.5;
            const prepared = duration.prepare(options);
            const reference = prepared.log_probability(3);
            const cache = prepared.cached_log_probabilities();
            const duplicate = prepared.cloned();
            const snapshot = prepared.parameters();
            const copiedOptions = prepared.options();
            const ordinary = duration.cloned();
            duration.free(); options.free();
            assert(prepared.log_probability(3) === reference, 'prepared duration after source release');
            prepared.free();
            assert(duplicate.log_probability(3) === reference, 'prepared duration after original cache release');
            equal([...duplicate.cached_log_probabilities()], [...cache], 'all duration cache values retained');
            const recovered = duplicate.into_parameters();
            const again = recovered.prepare_owned(copiedOptions);
            copiedOptions.free();
            assert(again.log_probability(3) === reference, 'consumed duration preparation independence');
            again.free();
            equal([...snapshot.values()], [3, 2, 0.25], 'duration parameter snapshot lifetime');
            equal([...ordinary.constraints()], [1, 8, -1], 'ordinary duration clone lifetime');
            snapshot.free(); ordinary.free();

            const durationStats = new api['DurationStatistics' + suffix]();
            durationStats.set_values(new T([6, 20, 2]));
            const durationStatsCopy = durationStats.cloned();
            durationStats.free();
            equal([...durationStatsCopy.values()], [6, 20, 2], 'duration statistics clone after release');
            durationStatsCopy.clear();
            equal([...durationStatsCopy.values()], [0, 0, 0], 'retained duration statistics mutable');
            durationStatsCopy.free();

            const gaussian = new api['Gaussian' + suffix](2, 2);
            gaussian.replace(2, new T([0.25, 0.75]), new T([1, 2, 3, 4]),
                new T([2, 3, 4, 5]), new T([0.125, 0.25, 0.375, 0.5]));
            const gaussianPrepared = gaussian.prepare();
            const observation = new T([0.25, 0.5]);
            const score = gaussianPrepared.log_probability(observation, 0.75);
            const normalizers = gaussianPrepared.cached_log_normalizers();
            const gaussianPreparedCopy = gaussianPrepared.cloned();
            const gaussianSnapshot = gaussianPrepared.parameters();
            const gaussianOrdinary = gaussian.cloned();
            const stats = api['GaussianStatistics' + suffix].from_gaussian(gaussian);
            stats.replace(2, new T([1, 2, 3, 4]), new T([5, 6, 7, 8]), new T([2, 3]), new T([5]));
            const statsCopy = stats.cloned();
            gaussian.free(); stats.free();
            assert(gaussianPrepared.log_probability(observation, 0.75) === score, 'prepared Gaussian source release');
            gaussianPrepared.free();
            assert(gaussianPreparedCopy.log_probability(observation, 0.75) === score, 'prepared Gaussian cache clone release');
            equal([...gaussianPreparedCopy.cached_log_normalizers()], [...normalizers], 'all normalizers retained');
            const gaussianRecovered = gaussianPreparedCopy.into_parameters();
            const gaussianAgain = gaussianRecovered.prepare_owned();
            assert(gaussianAgain.log_probability(observation, 0.75) === score, 'consumed Gaussian parameters retain full model');
            gaussianAgain.free();
            for (const [field, expected] of [
                ['weights', [0.25, 0.75]], ['means', [1, 2, 3, 4]],
                ['variances', [2, 3, 4, 5]], ['variance_floors', [0.125, 0.25, 0.375, 0.5]],
            ]) {
                equal([...gaussianSnapshot[field]()], expected, 'prepared Gaussian copied field ' + field);
                equal([...gaussianOrdinary[field]()], expected, 'ordinary Gaussian copied field ' + field);
            }
            gaussianSnapshot.free(); gaussianOrdinary.free();
            equal([...statsCopy.weighted_sums()], [1, 2, 3, 4], 'statistics complete sums after source release');
            equal([...statsCopy.weighted_squared_sums()], [5, 6, 7, 8], 'statistics complete squares after source release');
            equal([...statsCopy.component_occupancies()], [2, 3], 'statistics component occupancies after source release');
            equal([...statsCopy.state_occupancy()], [5], 'statistics state occupancy after source release');
            statsCopy.clear();
            equal([...statsCopy.weighted_sums()], [0, 0, 0, 0], 'retained Gaussian statistics mutable');
            statsCopy.free();
            cycles++;
        }
    }
    assert(cycles === 128, 'complete both-precision lifetime cycles');
    return {cycles, mappedReleaseFamilies: 4, precisions: 2};
}
