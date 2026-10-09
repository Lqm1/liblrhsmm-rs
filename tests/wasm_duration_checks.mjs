function assert(value, message) { if (!value) throw new Error(message); }
function equal(actual, expected, message) { assert(Object.is(actual, expected), `${message}: ${actual} != ${expected}`); }
function array(actual, expected, message) { assert(actual.length === expected.length, message + ' length'); actual.forEach((v, i) => equal(v, expected[i], message + '[' + i + ']')); }
function rejects(run, message) { let failed = false; try { run(); } catch { failed = true; } assert(failed, message); }
const decoder = new TextDecoder();
export async function verifyDurationBindings(api, load, serialization = true) {
    let preparationCases = 0, updateCases = 0, wireBytes = 0, maxDoubleError = 0;
    for (const suffix of ['F32', 'F64']) {
        const single = suffix === 'F32', cast = single ? Math.fround : v => v;
        const D = api['Duration' + suffix], P = api['PreparationOptions' + suffix], S = api['DurationStatistics' + suffix], U = api['UpdateOptions' + suffix];
        const duration = new D(), options = new P();
        array(duration.values(), [0, 0, 0], 'flat start'); array(duration.constraints(), [-1, -1, -1], 'unset constraints');
        rejects(() => duration.prepare(options), 'uninitialized preparation');
        equal(options.duration_cache_length, 300, 'default cache'); equal(options.duration_weight, 1, 'default weight'); equal(options.temperature, 1, 'default temperature');
        duration.set_values(new (single ? Float32Array : Float64Array)([4.25, 2.5, 0])); duration.set_constraints(new Int32Array([2, 6, -1]));
        const reference = decoder.decode(await load(single ? 'c-preparation.txt' : 'c-double-preparation.txt'));
        for (const line of reference.trim().split(/\r?\n/).filter(line => line.startsWith('D '))) {
            const [, length, temperature, frame, expectedLog, expectedProbability] = line.split(/\s+/).map((v, i) => i ? Number(v) : v);
            options.duration_cache_length = length; options.duration_weight = .75; options.temperature = temperature * .5;
            const prepared = duration.prepare(options), clone = prepared.cloned(), snapshot = prepared.parameters(), settings = prepared.options();
            equal(settings.duration_cache_length, length, 'settings cache'); equal(settings.temperature, temperature * .5, 'settings temperature'); equal(settings.duration_weight, .75, 'settings weight');
            const log = prepared.log_probability(frame), probability = prepared.probability(frame);
            if (single) equal(log, Math.fround(expectedLog), line);
            else { const error = Math.abs(log - expectedLog) / Math.max(1, Math.abs(expectedLog)); maxDoubleError = Math.max(maxDoubleError, error); assert(error <= 2e-14, line); }
            equal(probability, single ? Math.fround(expectedProbability) : expectedProbability, line + ' probability');
            equal(log, duration.annealed_log_probability(frame, .75, temperature * .5), 'ordinary/prepared equality'); equal(probability, duration.probability(frame, .75, temperature * .5), 'ordinary probability');
            equal(prepared.cached_log_probabilities().length, length, 'cache length');
            if (frame < length) equal(prepared.cached_log_probabilities()[frame], log, 'cached score');
            prepared.free(); equal(clone.log_probability(frame), log, 'independent prepared clone');
            array(snapshot.values(), duration.values(), 'prepared parameter snapshot');
            const recovered = clone.into_parameters(); array(recovered.constraints(), [2, 6, -1], 'consumed preparation'); recovered.free(); snapshot.free(); settings.free(); preparationCases++;
        }
        options.duration_cache_length = 3; options.temperature = 0;
        const zero = duration.prepare(options); equal(zero.log_probability(1), cast(-1e15), 'minimum at temperature zero'); equal(zero.log_probability(2), 0, 'included zero temperature');
        rejects(() => zero.log_probability(0x80000000), 'frame range'); zero.free();
        options.temperature = 1;
        for (const name of ['temperature', 'duration_weight']) { const previous = options[name]; for (const value of [-1, NaN, Infinity]) { options[name] = value; rejects(() => duration.prepare(options), name); } options[name] = previous; }
        options.duration_cache_length = 0x80000000; rejects(() => duration.prepare(options), 'cache range'); options.duration_cache_length = 3;
        const consumed = duration.cloned(), prepared = consumed.prepare_owned(options), copied = duration.cloned();
        const expected = prepared.log_probability(4); duration.set_values(new (single ? Float32Array : Float64Array)([30, 4, 0])); duration.free();
        equal(prepared.log_probability(4), expected, 'snapshot survives source mutation/release');
        const cache = prepared.cached_log_probabilities(); cache.fill(123); assert(prepared.cached_log_probabilities()[0] !== 123, 'cache independent copy');
        prepared.free(); copied.free(); options.free();

        const bits = single ? new Uint32Array([0xffc00123, 0x80000000, 0x7f800000]) : new BigUint64Array([0xfff8000000000123n, 0x8000000000000000n, 0x7ff0000000000000n]);
        const raw = new (single ? Float32Array : Float64Array)(bits.buffer), bitOwner = new D();
        bitOwner.set_values(raw);
        const restored = bitOwner.values(); array(new (single ? Uint32Array : BigUint64Array)(restored.buffer), bits, 'arbitrary scalar IEEE bits');
        const bitCopy = bitOwner.cloned(); bitOwner.free(); array(new (single ? Uint32Array : BigUint64Array)(bitCopy.values().buffer), bits, 'cloned scalar IEEE bits'); bitCopy.free();
        const stats = new S(), update = new U(), editable = new D();
        array(stats.values(), [0, 0, 0], 'default statistics'); equal(update.duration_mode, 0, 'default mode'); equal(update.duration_variance_floor, cast(.05), 'default duration floor'); equal(update.emission_variance_floor, cast(.01), 'default emission floor');
        const updates = decoder.decode(await load(single ? 'c-updates.txt' : 'c-double-update.txt'));
        for (const line of updates.trim().split(/\r?\n/).filter(line => line.startsWith('duration '))) {
            const [, scenario, mean, variance] = line.split(/\s+/).map((v, i) => i ? Number(v) : v);
            editable.set_values(new (single ? Float32Array : Float64Array)([7, 3, scenario === 2 ? 2.5 : 0]));
            stats.set_values(new (single ? Float32Array : Float64Array)([scenario === 3 ? 1.5 : 9, 29, scenario === 4 ? 1e-8 : 3]));
            update.duration_mode = scenario === 3 ? 1 : 0;
            equal(editable.update(stats, update), scenario !== 4, 'update occupancy'); array(editable.values().slice(0, 2), [cast(mean), cast(variance)], line); updateCases++;
        }
        editable.set_constraints(new Int32Array([-1, -1, 4])); stats.set_values(new (single ? Float32Array : Float64Array)([6, 20, 2])); update.duration_mode = 0;
        equal(editable.update(stats, update), true, 'fixed mean update'); array(editable.values().slice(0, 2), [4, 2], 'correct residual variance');
        const before = editable.values();
        for (const mode of [2, 0xffffffff]) { update.duration_mode = mode; rejects(() => editable.update(stats, update), 'invalid mode'); array(editable.values(), before, 'atomic mode error'); } update.duration_mode = 0;
        for (const field of ['value_sum', 'squared_sum', 'occupancy']) { const previous = stats[field]; stats[field] = NaN; rejects(() => editable.update(stats, update), 'invalid statistics'); array(editable.values(), before, 'atomic statistics error'); stats[field] = previous; }
        for (const field of ['duration_variance_floor', 'emission_variance_floor']) { const previous = update[field]; update[field] = -1; rejects(() => editable.update(stats, update), 'invalid floor'); array(editable.values(), before, 'atomic floor error'); update[field] = previous; }
        rejects(() => editable.set_values(new (single ? Float32Array : Float64Array)([1, 2])), 'scalar shape'); array(editable.values(), before, 'atomic scalar shape');
        rejects(() => editable.set_constraints(new Int32Array([1])), 'constraint shape'); array(editable.constraints(), [-1, -1, 4], 'atomic constraint shape');
        const statCopy = stats.cloned(), updateCopy = update.cloned(); stats.set_values(raw); array(new (single ? Uint32Array : BigUint64Array)(stats.values().buffer), bits, 'statistics IEEE bits'); stats.clear(); array(stats.values(), [0, 0, 0], 'clear'); assert(statCopy.occupancy !== 0, 'independent statistics clone'); updateCopy.duration_mode = 1; equal(update.duration_mode, 0, 'independent update clone');
        statCopy.free(); updateCopy.free(); stats.free(); update.free(); editable.free();

        if (serialization) {
            for (const name of ['c-standalone-duration.bin', 'c-double-wire-duration.bin']) {
                const bytes = new Uint8Array(await load(name)), value = D.read(bytes);
                array(value.write(), bytes, 'C wire round trip'); wireBytes += bytes.length;
                const joined = new Uint8Array(bytes.length * 2 + 1); joined.set(bytes); joined.set(bytes, bytes.length); joined[joined.length - 1] = 42;
                rejects(() => D.read(joined), 'strict trailing input');
                const prefix = D.read_prefix(joined); equal(prefix.position, BigInt(bytes.length), 'prefix cursor');
                const first = prefix.parameters(); array(first.write(), bytes, 'prefix parameters'); first.free(); const recovered = prefix.into_parameters(); recovered.free();
                const bounded = D.read_prefix_with_limits(joined, 6); bounded.free(); rejects(() => D.read_prefix_with_limits(joined, 5), 'decode budget');
                const strict = D.read_with_limits(bytes, 6); strict.free(); rejects(() => D.read_with_limits(bytes, 5), 'strict budget');
                for (let end = 0; end < bytes.length; end++) rejects(() => D.read_prefix(bytes.subarray(0, end)), 'truncated prefix');
                value.free();
            }
        } else {
            for (const name of ['read', 'read_with_limits', 'read_prefix', 'read_prefix_with_limits']) assert(!(name in D), 'serialization-disabled ' + name);
            assert(!('write' in D.prototype), 'serialization-disabled write'); assert(!('DecodedDuration' + suffix in api), 'serialization-disabled decoded owner');
        }
    }
    equal(preparationCases, 144, 'complete preparation corpora'); equal(updateCases, 10, 'complete update corpora');
    return { preparationCases, updateCases, wireBytes, maxDoubleError };
}
