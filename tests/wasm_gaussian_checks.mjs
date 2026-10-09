function assert(value, message) { if (!value) throw new Error(message); }
function equal(a, b, message) { assert(Object.is(a, b), `${message}: ${a} != ${b}`); }
function array(a, b, message) { equal(a.length, b.length, message + ' length'); a.forEach((v, i) => equal(v, b[i], message + '[' + i + ']')); }
function rejects(run, message) { let failed = false; try { run(); } catch { failed = true; } assert(failed, message); }
const text = new TextDecoder();
export async function verifyGaussianBindings(api, load, serialization = true) {
    let precisionValues = 0, updateValues = 0, emissionValues = 0, wireBytes = 0, maximumError = 0;
    for (const suffix of ['F32', 'F64']) {
        const single = suffix === 'F32', cast = single ? Math.fround : v => v, A = single ? Float32Array : Float64Array;
        const G = api['Gaussian' + suffix], S = api['GaussianStatistics' + suffix], U = api['UpdateOptions' + suffix], C = api['GaussianCollection' + suffix], DC = api['DurationCollection' + suffix], D = api['Duration' + suffix];
        const flat = new G(2, 3); array(flat.weights(), [.5, .5], 'uniform default'); array(flat.means(), Array(6).fill(0), 'zero means'); rejects(() => flat.prepare(), 'uninitialized distribution'); flat.free();
        for (const shape of [[0, 1], [1, 0], [0xffffffff, 1]]) rejects(() => new G(...shape), 'invalid constructor dimensions');
        const distribution = new G(1, 2);
        if (single) {
            const observations = [[0, 0], [1.25, -2.5], [-3, 4], [100, -100]];
            for (const line of text.decode(await load('c-emissions.txt')).trim().split(/\r?\n/)) {
                const [mixtures, index, temperature, ...expected] = line.split(/\s+/).map(Number), g = new G(mixtures, 2);
                g.replace(2, new A(Array.from({length: mixtures}, (_, k) => (k + 1) / (mixtures * (mixtures + 1) / 2))), new A(Array.from({length: mixtures * 2}, (_, i) => i % 2 ? -2.5 - Math.floor(i / 2) : 1.25 + Math.floor(i / 2))), new A(Array.from({length: mixtures * 2}, (_, i) => (i % 2 ? 2.5 : .75) + Math.floor(i / 2))), new A(Array(mixtures * 2).fill(0)));
                const p = g.prepare(), observation = new A(observations[index]);
                const actual = [p.log_probability(observation, temperature), p.probability(observation, temperature), ...Array.from({length: mixtures}, (_, k) => [p.component_log_probability(observation, k), p.component_probability(observation, k)]).flat()];
                array(actual, expected.map(Math.fround), line); emissionValues += actual.length; p.free(); g.free();
            }
        }
        distribution.replace(2, new A([1]), new A([1.25, -2.5]), new A([2.5, .75]), new A([.1, .2])); distribution.validate(); distribution.increase_mixtures(5, .2);
        const prepared = distribution.prepare();
        for (const line of text.decode(await load(single ? 'c-precision-float.txt' : 'c-precision-double.txt')).trim().split(/\r?\n/)) {
            const [operation, indexString, secondaryString, ...values] = line.split(/\s+/), index = Number(indexString), secondary = Number(secondaryString), expected = values.map(Number);
            if (operation === 'duration') continue;
            const observation = new A([index * .3 - .5, index * -.2 + .75]);
            let actual;
            if (operation === 'mixture') actual = [distribution.weights()[index], ...[0, 1].flatMap(d => [distribution.means()[index * 2 + d], distribution.variances()[index * 2 + d], distribution.variance_floors()[index * 2 + d]])];
            else if (operation === 'component') actual = [prepared.component_log_probability(observation, secondary), prepared.component_probability(observation, secondary)];
            else if (operation === 'output') actual = [prepared.log_probability(observation, [.5, 1, 1.75][secondary]), prepared.probability(observation, [.5, 1, 1.75][secondary])];
            else throw new Error('Unknown precision operation: ' + operation);
            equal(actual.length, expected.length, 'precision shape');
            actual.forEach((value, i) => { const error = Math.abs(value - expected[i]) / Math.max(1, Math.abs(expected[i])); maximumError = Math.max(maximumError, error); assert(error <= 2e-14, line + ' normalized error ' + error); precisionValues++; });
        }
        const observation = new A([.125, -.25]), reference = prepared.log_probability(observation, 1), normalizers = prepared.cached_log_normalizers();
        equal(normalizers.length, 5, 'normalizer count'); normalizers.fill(123); assert(prepared.cached_log_normalizers()[0] !== 123, 'normalizers copied');
        for (const bad of [new A([]), new A([1]), new A([NaN, 1])]) rejects(() => prepared.log_probability(bad, 1), 'invalid observation');
        rejects(() => prepared.component_probability(observation, 5), 'component range'); rejects(() => prepared.log_probability(observation, -1), 'temperature range');
        const cloned = prepared.cloned(), snapshot = prepared.parameters(), recovered = cloned.into_parameters(), consumed = recovered.prepare_owned();
        distribution.replace(1, new A([1]), new A([99]), new A([1]), new A([0])); distribution.free();
        equal(prepared.log_probability(observation, 1), reference, 'prepared source independence'); prepared.free(); equal(consumed.log_probability(observation, 1), reference, 'consumed lifetime'); consumed.free(); snapshot.free();

        const updates = text.decode(await load(single ? 'c-updates.txt' : 'c-double-update.txt'));
        for (const line of updates.trim().split(/\r?\n/).filter(line => line.startsWith('mixture '))) {
            const [, scenarioString, componentString, ...values] = line.split(/\s+/), scenario = Number(scenarioString), component = Number(componentString), mixtures = scenario === 0 ? 1 : 2;
            const g = new G(mixtures, 2), stats = S.from_gaussian(g), options = new U();
            g.replace(2, new A(Array(mixtures).fill(.5)), new A(Array(mixtures * 2).fill(-7)), new A(Array(mixtures * 2).fill(8)), new A(Array.from({length: mixtures * 2}, (_, i) => i % 2 ? .75 : 0)));
            stats.replace(2, new A(Array.from({length: mixtures * 2}, (_, i) => i % 2 ? 6 : -3.5)), new A(Array.from({length: mixtures * 2}, (_, i) => i % 2 ? 12 : 9.25)), new A(Array.from({length: mixtures}, (_, i) => scenario === 2 && i === 1 ? 1e-8 : 2 + i)), new A([scenario === 3 ? 1e-8 : 4]));
            equal(g.update(stats, options), scenario !== 3, 'occupancy update flag');
            const offset = component * 2, actual = [g.weights()[component], g.means()[offset], g.variances()[offset], g.means()[offset + 1], g.variances()[offset + 1]];
            array(actual, values.map(v => cast(Number(v))), line); updateValues += actual.length;
            const statCopy = stats.cloned(); stats.clear(); array(stats.weighted_sums(), Array(mixtures * 2).fill(0), 'clear sums'); array(stats.weighted_squared_sums(), Array(mixtures * 2).fill(0), 'clear squares'); array(stats.component_occupancies(), Array(mixtures).fill(0), 'clear occupancy'); equal(stats.state_occupancy()[0], 0, 'clear state'); assert(statCopy.state_occupancy()[0] !== 0, 'stat clone independence');
            statCopy.free(); stats.free(); options.free(); g.free();
        }
        const owner = new G(1, 1), stats = new S(1, 1), options = new U();
        owner.replace(1, new A([1]), new A([2]), new A([3]), new A([.25])); stats.replace(1, new A([4]), new A([10]), new A([2]), new A([2]));
        const nativeSingle = owner.prepare(), singleScore = nativeSingle.log_probability(new A([0]), 1); nativeSingle.free();
        owner.replace(1, new A([.25]), new A([2]), new A([3]), new A([.25])); const weightedSingle = owner.prepare(); equal(weightedSingle.log_probability(new A([0]), 1), singleScore, 'original single-component weight convention'); equal(weightedSingle.log_probability(new A([single ? 3.4028234663852886e38 : Number.MAX_VALUE]), 0), 0, 'zero-temperature overflow behavior'); weightedSingle.free();
        rejects(() => owner.increase_mixtures(2, NaN), 'invalid split displacement'); equal(owner.mixtures, 1, 'atomic split error'); rejects(() => owner.increase_mixtures(0x80000000, 1), 'invalid split count'); equal(owner.mixtures, 1, 'atomic split count');
        if (serialization) rejects(() => owner.write_with_encoding(1), 'historical schema rejects nonzero floors');
        const before = owner.means();
        rejects(() => owner.replace(2, new A([1]), new A([1]), new A([1]), new A([1])), 'parameter shape'); array(owner.means(), before, 'atomic replace');
        rejects(() => stats.replace(2, new A([1]), new A([1]), new A([1]), new A([1])), 'stat shape'); array(stats.weighted_sums(), [4], 'atomic statistic replace');
        stats.replace(1, new A([NaN]), new A([10]), new A([2]), new A([2])); rejects(() => owner.update(stats, options), 'invalid update statistic'); array(owner.means(), before, 'atomic update');
        const rawBits = single ? new Uint32Array([0xffc00123]) : new BigUint64Array([0xfff8000000000123n]), raw = new A(rawBits.buffer), B = single ? Uint32Array : BigUint64Array;
        owner.replace(1, raw, raw, raw, raw);
        for (const name of ['weights', 'means', 'variances', 'variance_floors']) array(new B(owner[name]().buffer), rawBits, 'IEEE parameter ' + name);
        stats.replace(1, raw, raw, raw, raw);
        for (const name of ['weighted_sums', 'weighted_squared_sums', 'component_occupancies', 'state_occupancy']) array(new B(stats[name]().buffer), rawBits, 'IEEE statistic ' + name);
        rejects(() => owner.prepare(), 'NaN preparation'); rejects(() => owner.increase_mixtures(2, 1), 'NaN splitting');
        owner.free(); stats.free(); options.free();

        const list = new C(), durations = new DC(), source = new G(1, 1), d = new D();
        equal(list.is_empty(), true, 'empty collection'); rejects(() => G.combine(list), 'empty combination'); equal(list.get(0), undefined, 'absent item');
        source.replace(1, new A([1]), new A([3]), new A([4]), new A([.5])); list.push(source); list.push(source);
        d.set_values(new A([5, 6, 7])); durations.push(d); durations.push(d); source.free(); d.free();
        const listCopy = list.cloned(), durationCopy = durations.cloned(); list.clear(); durations.clear(); equal(list.length, 0, 'collection clear'); equal(listCopy.length, 2, 'independent collection');
        const joined = G.combine(listCopy); array(joined.means(), [3, 3], 'combination'); array(joined.variances(), [4, 4], 'combination variances'); array(joined.variance_floors(), [.5, .5], 'combination floors');
        const fromDuration = G.from_durations(durationCopy); array(fromDuration.means(), [5, 5], 'duration combination'); array(fromDuration.variances(), [6, 6], 'duration variances'); array(fromDuration.variance_floors(), [0, 0], 'native duration-combination floors');
        const item = listCopy.get(0), durationItem = durationCopy.get(0); listCopy.replace(1, item); durationCopy.replace(1, durationItem); rejects(() => listCopy.replace(2, item), 'collection range'); rejects(() => durationCopy.replace(2, durationItem), 'duration collection range');
        const multi = new G(2, 1); list.push(multi); rejects(() => G.combine(list), 'combination requires single components'); multi.free(); rejects(() => G.from_durations(durations), 'empty duration combination');
        listCopy.free(); durationCopy.free(); array(item.means(), [3], 'copied item lifetime'); array(durationItem.values(), [5, 6, 7], 'copied duration lifetime');
        for (const object of [item, durationItem, joined, fromDuration, list, durations]) object.free();

        if (serialization) {
            for (const [name, encoding, budget] of [['c-standalone-gmm.bin', 0, 18], ['c-historical-standalone-gmm.bin', 1, 29]]) {
                const bytes = new Uint8Array(await load(name)), value = G.read(bytes); array(value.write_with_encoding(encoding), bytes, 'original GMM wire'); if (!encoding) array(value.write(), bytes, 'default encoding'); wireBytes += bytes.length;
                const joined = new Uint8Array(bytes.length * 2); joined.set(bytes); joined.set(bytes, bytes.length); rejects(() => G.read(joined), 'trailing wire');
                const prefix = G.read_prefix(joined); equal(prefix.position, BigInt(bytes.length), 'wire cursor'); const copy = prefix.parameters(); array(copy.write_with_encoding(encoding), bytes, 'prefix values'); copy.free(); const consumed = prefix.into_parameters(); consumed.free();
                const limited = G.read_prefix_with_limits(joined, budget); limited.free(); rejects(() => G.read_prefix_with_limits(joined, budget - 1), 'prefix budget'); const strict = G.read_with_limits(bytes, budget); strict.free(); rejects(() => G.read_with_limits(bytes, budget - 1), 'strict budget');
                for (let end = 0; end < bytes.length; end++) rejects(() => G.read_prefix(bytes.subarray(0, end)), 'truncated wire'); rejects(() => value.write_with_encoding(2), 'encoding range'); value.free();
            }
        } else { for (const name of ['read', 'read_with_limits', 'read_prefix', 'read_prefix_with_limits']) assert(!(name in G), 'disabled ' + name); assert(!('write' in G.prototype) && !('write_with_encoding' in G.prototype), 'disabled writes'); assert(!('DecodedGaussian' + suffix in api), 'disabled decoded owner'); }
    }
    equal(precisionValues, 230, 'complete Gaussian precision corpora'); equal(updateValues, 70, 'complete Gaussian update corpora'); equal(emissionValues, 144, 'complete original single/multiple component corpus');
    return { precisionValues, updateValues, emissionValues, wireBytes, maximumError };
}
