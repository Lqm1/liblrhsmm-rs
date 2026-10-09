function assert(value, message) { if (!value) throw new Error(message); }
function equal(a, b, message) { assert(Object.is(a, b), `${message}: ${a} != ${b}`); }
function array(a, b, message) { equal(a.length, b.length, message + ' length'); a.forEach((v, i) => equal(v, b[i], message + '[' + i + ']')); }
function rejects(run, message) { let failed = false; try { run(); } catch { failed = true; } assert(failed, message); }
export async function verifyContainerBindings(api, load, serialization = true) {
    let wireBytes = 0, preparedHistoricalDurations = 0, constructedModels = 0;
    for (const suffix of ['F32', 'F64']) {
        const single = suffix === 'F32', A = single ? Float32Array : Float64Array, B = single ? Uint32Array : BigUint64Array;
        const G = api['Gaussian' + suffix], GC = api['GaussianCollection' + suffix], D = api['Duration' + suffix], DC = api['DurationCollection' + suffix];
        const S = api['Stream' + suffix], SB = api['StreamBuilder' + suffix], SC = api['StreamCollection' + suffix], M = api['Model' + suffix], MB = api['ModelBuilder' + suffix], P = api['PreparationOptions' + suffix];
        for (const object of [S.empty(), new S(0, 0, 0), new SB(0).build()]) { equal(object.emissions, 0, 'empty stream'); array(object.weight(), [1], 'empty weight'); const p = object.prepare(); equal(p.emission(0), undefined, 'empty prepared emission'); p.free(); object.free(); }
        for (const object of [M.empty(), new M(new Uint32Array(), 0), new MB(0, 0).build()]) { equal(object.streams, 0, 'empty streams'); equal(object.durations, 0, 'empty durations'); object.validate(); object.free(); }
        rejects(() => new M(new Uint32Array([1]), 0), 'layout triples'); rejects(() => new S(1, 0, 1), 'invalid flat shape'); rejects(() => new MB(0x80000000, 0), 'model builder range'); rejects(() => new MB(0, 0x80000000), 'duration builder range'); rejects(() => new SB(0x80000000), 'stream builder range');
        const flat = new M(new Uint32Array([2, 3, 2, 1, 2, 1]), 3);
        equal(flat.streams, 2, 'flat model stream count'); equal(flat.durations, 3, 'flat duration count');
        const flatStream = flat.stream(0), flatEmission = flatStream.emission(0); equal(flatStream.emissions, 2, 'flat emissions'); array(flatEmission.weights(), Array(3).fill(single ? Math.fround(1 / 3) : 1 / 3), 'uniform flat weights'); flatEmission.free(); flatStream.free();
        if (serialization && single) { const expected = new Uint8Array(await load('c-construction-model.hsmm')); array(flat.write(), expected, 'exact original construction'); wireBytes += expected.length; }
        flat.free();
        const g = new G(1, 1); g.replace(1, new A([1]), new A([2]), new A([3]), new A([.25]));
        const d = new D(); d.set_values(new A([4.25, 2.5, .1])); d.set_constraints(new Int32Array([2, 6, -1]));
        const streamBuilder = new SB(2); equal(streamBuilder.has_emission(0), false, 'unfilled emission'); equal(streamBuilder.emission(0), undefined, 'absent slot'); rejects(() => streamBuilder.cloned().build(), 'unfilled stream build');
        streamBuilder.set_emission(0, g); streamBuilder.set_emission(1, g); streamBuilder.set_weight(new A([.75])); streamBuilder.clear_emission(1); equal(streamBuilder.has_emission(1), false, 'clear emission'); streamBuilder.set_emission(1, g);
        rejects(() => streamBuilder.set_emission(2, g), 'slot range'); rejects(() => streamBuilder.clear_emission(2), 'clear range'); rejects(() => streamBuilder.set_weight(new A([])), 'weight shape'); array(streamBuilder.weight(), [.75], 'atomic weight error');
        const s = streamBuilder.build(), modelBuilder = new MB(1, 2); equal(modelBuilder.has_stream(0), false, 'unfilled stream'); equal(modelBuilder.has_duration(0), false, 'unfilled duration'); rejects(() => modelBuilder.cloned().build(), 'unfilled model build');
        modelBuilder.set_stream(0, s); modelBuilder.set_duration(0, d); modelBuilder.set_duration(1, d); modelBuilder.clear_stream(0); modelBuilder.clear_duration(1); equal(modelBuilder.stream(0), undefined, 'cleared stream'); equal(modelBuilder.duration(1), undefined, 'cleared duration'); modelBuilder.set_stream(0, s); modelBuilder.set_duration(1, d);
        rejects(() => modelBuilder.set_stream(1, s), 'stream slot range'); rejects(() => modelBuilder.set_duration(2, d), 'duration slot range'); rejects(() => modelBuilder.clear_stream(1), 'stream clear range'); rejects(() => modelBuilder.clear_duration(2), 'duration clear range');
        const builderCopy = modelBuilder.cloned(), model = modelBuilder.build(); equal(builderCopy.has_stream(0), true, 'independent builder clone'); builderCopy.free(); constructedModels++;
        const options = new P(); options.duration_cache_length = 3; options.duration_weight = .75;
        const prepared = model.prepare(options), preparedStream = s.prepare(), child = prepared.emission(0, 0), durationChild = prepared.duration(0), streamChild = preparedStream.emission(1), settings = prepared.options();
        equal(settings.duration_cache_length, 3, 'prepared settings'); equal(prepared.duration(2), undefined, 'duration range'); equal(prepared.emission(1, 0), undefined, 'stream range'); equal(prepared.emission(0, 2), undefined, 'emission range');
        const standaloneGaussian = g.prepare(), standaloneDuration = d.prepare(options);
        const gaussianCache = standaloneGaussian.cached_log_normalizers(), durationCache = standaloneDuration.cached_log_probabilities();
        array(child.cached_log_normalizers(), gaussianCache, 'model child complete Gaussian cache');
        array(streamChild.cached_log_normalizers(), gaussianCache, 'stream child complete Gaussian cache');
        array(durationChild.cached_log_probabilities(), durationCache, 'model child complete duration cache');
        equal(durationChild.log_normalizer(), standaloneDuration.log_normalizer(), 'model child duration normalizer');
        const childSettings = durationChild.options();
        for (const field of ['duration_cache_length', 'duration_weight', 'temperature']) equal(childSettings[field], options[field], 'child setting ' + field);
        childSettings.duration_cache_length = 0; childSettings.duration_weight = 0; childSettings.temperature = 0;
        settings.duration_cache_length = 0; settings.duration_weight = 0; settings.temperature = 0;
        const retainedSettings = prepared.options(), retainedChildSettings = durationChild.options();
        for (const field of ['duration_cache_length', 'duration_weight', 'temperature']) {
            equal(retainedSettings[field], options[field], 'detached model setting ' + field);
            equal(retainedChildSettings[field], options[field], 'detached child setting ' + field);
        }
        retainedSettings.free(); retainedChildSettings.free(); childSettings.free(); standaloneGaussian.free(); standaloneDuration.free();
        const detachedGaussianCache = child.cached_log_normalizers(), detachedDurationCache = durationChild.cached_log_probabilities();
        detachedGaussianCache.fill(123); detachedDurationCache.fill(123);
        array(child.cached_log_normalizers(), gaussianCache, 'detached model Gaussian cache');
        array(durationChild.cached_log_probabilities(), durationCache, 'detached model duration cache');
        const probability = child.log_probability(new A([0]), 1), durationScore = durationChild.log_probability(4);
        const copy = model.cloned(), streamCopy = s.cloned(), preparedCopy = prepared.cloned(), preparedStreamCopy = preparedStream.cloned();
        g.replace(1, new A([1]), new A([99]), new A([1]), new A([0])); const emptyStream = S.empty(), unsetDuration = new D(); model.set_stream(0, emptyStream); model.set_duration(0, unsetDuration); emptyStream.free(); unsetDuration.free(); model.free(); s.free(); g.free(); d.free();
        const copiedStream = copy.stream(0), copiedEmission = copiedStream.emission(0), copiedDuration = copy.duration(0); array(copiedEmission.means(), [2], 'deep source independence'); array(copiedDuration.constraints(), [2, 6, -1], 'duration constraints');
        copiedStream.free(); copiedEmission.free(); copiedDuration.free(); copy.free(); streamCopy.free(); prepared.free(); preparedStream.free();
        equal(child.log_probability(new A([0]), 1), probability, 'prepared child survives parent'); equal(streamChild.log_probability(new A([0]), 1), probability, 'stream child survives parent'); equal(durationChild.log_probability(4), durationScore, 'duration child survives parent');
        const recovered = preparedCopy.into_parameters(), recoveredStream = preparedStreamCopy.into_parameters(); const owned = recovered.prepare_owned(options), ownedStream = recoveredStream.prepare_owned(); owned.free(); ownedStream.free(); child.free(); streamChild.free(); durationChild.free(); settings.free();
        const model2 = M.empty(), streams = new SC(), durations = new DC(), emissions = new GC(), source = new S(0, 0, 0), gaussian = new G(1, 1), duration = new D(); gaussian.replace(1, new A([1]), new A([5]), new A([2]), new A([0])); emissions.push(gaussian); source.replace(new A([.5]), emissions); duration.set_values(new A([4, 1, 0])); streams.push(source); durations.push(duration); model2.replace(streams, durations); model2.validate();
        const rawBits = single ? new Uint32Array([0xffc00123]) : new BigUint64Array([0xfff8000000000123n]); source.set_weight(new A(rawBits.buffer)); array(new B(source.weight().buffer), rawBits, 'stream IEEE bits'); const streamClone = source.cloned(); array(new B(streamClone.weight().buffer), rawBits, 'cloned weight bits'); streamClone.free();
        equal(model2.stream(1), undefined, 'absent stream'); rejects(() => model2.set_stream(1, source), 'model stream range'); rejects(() => model2.set_duration(1, duration), 'model duration range');
        const emptyDuration = new D(); model2.set_duration(0, emptyDuration); rejects(() => model2.prepare(options), 'unused uninitialized duration'); const selected = model2.prepare_selected_durations(options, new Uint32Array()); equal(selected.duration(0), undefined, 'unselected duration'); selected.free(); rejects(() => model2.prepare_selected_durations(options, new Uint32Array([1])), 'selected state range'); model2.set_duration(0, duration);
        const selectedOwned = model2.prepare_owned_selected_durations(options, new Uint32Array([0, 0])); const selectedChild = selectedOwned.duration(0); assert(selectedChild !== undefined, 'duplicate selected state'); selectedChild.free(); selectedOwned.free();
        for (const object of [streams, durations, emissions, source, gaussian, duration, emptyDuration, options]) object.free();
        if (serialization) {
            for (const [Owner, name, encoding, budget] of [[S, 'c-standalone-stream.bin', 0, 33], [S, 'c-historical-standalone-stream.bin', 1, 6452], [M, 'current-c-model.hsmm', 0, 28606], [M, 'cmu-arctic-all-speakers.hsmm', 1, 20866]]) {
                const bytes = new Uint8Array(await load(name)), value = Owner.read(bytes); array(value.write_with_encoding(encoding), bytes, 'original container bytes ' + name); wireBytes += bytes.length;
                if (!encoding) array(value.write(), bytes, 'default current encoding'); rejects(() => value.write_with_encoding(2), 'encoding code');
                const joined = new Uint8Array(bytes.length + 1); joined.set(bytes); joined[joined.length - 1] = 42; rejects(() => Owner.read(joined), 'strict trailing'); const prefix = Owner.read_prefix(joined); equal(prefix.position, BigInt(bytes.length), 'prefix cursor'); const snapshot = prefix.parameters(); array(snapshot.write_with_encoding(encoding), bytes, 'prefix bytes'); snapshot.free(); const recovered = prefix.into_parameters(); recovered.free();
                const limited = Owner.read_with_limits(bytes, budget); limited.free(); rejects(() => Owner.read_with_limits(bytes, budget - 1), 'strict budget'); const limitedPrefix = Owner.read_prefix_with_limits(joined, budget); limitedPrefix.free(); rejects(() => Owner.read_prefix_with_limits(joined, budget - 1), 'prefix budget');
                const ends = bytes.length < 500 ? Array.from({length: bytes.length}, (_, i) => i) : [0, 1, 2, Math.floor(bytes.length / 2), bytes.length - 1]; for (const end of ends) rejects(() => Owner.read_prefix(bytes.subarray(0, end)), 'truncated container');
                if (Owner === M && encoding === 1) {
                    equal(value.durations, 215, 'historical total durations'); const states = [];
                    for (let i = 0; i < value.durations; i++) { const d = value.duration(i); if (d.values()[1] > 0) states.push(i); d.free(); }
                    equal(states.length, 205, 'historical initialized durations'); const p = new P(); p.duration_cache_length = 3; rejects(() => value.prepare(p), 'historical uninitialized states'); const prepared = value.prepare_selected_durations(p, new Uint32Array(states));
                    for (let i = 0; i < value.durations; i++) { const child = prepared.duration(i); equal(child !== undefined, states.includes(i), 'selected historical state'); if (child) { assert(Number.isFinite(child.log_probability(4)), 'historical density finite'); child.free(); } }
                    preparedHistoricalDurations += states.length; p.free(); prepared.free();
                }
                value.free();
            }
        } else { for (const Owner of [S, M]) { for (const name of ['read', 'read_with_limits', 'read_prefix', 'read_prefix_with_limits']) assert(!(name in Owner), 'disabled codec'); assert(!('write' in Owner.prototype), 'disabled output'); } for (const name of ['DecodedStream', 'DecodedModel']) assert(!(name + suffix in api), 'disabled decoded owner'); }
    }
    equal(constructedModels, 2, 'both precision constructions'); equal(preparedHistoricalDurations, serialization ? 410 : 0, 'both historical preparations');
    return { constructedModels, wireBytes, preparedHistoricalDurations };
}
