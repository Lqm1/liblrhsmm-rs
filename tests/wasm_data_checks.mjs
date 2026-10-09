function assert(value, message) { if (!value) throw new Error(message); }
function equal(a, b, message) { assert(Object.is(a, b), `${message}: ${a} != ${b}`); }
function array(a, b, message) { equal(a.length, b.length, message + ' length'); a.forEach((v, i) => equal(v, b[i], message + '[' + i + ']')); }
function rejects(run, message) { let failed = false; try { run(); } catch { failed = true; } assert(failed, message); }
export async function verifyDataBindings(api, load, serialization = true) {
    let wireBytes = 0, transitionRecords = 0, remappedStates = 0;
    for (const suffix of ['F32', 'F64']) {
        const single = suffix === 'F32', A = single ? Float32Array : Float64Array, B = single ? Uint32Array : BigUint64Array;
        const O = api['Observation' + suffix], OS = api['ObservationStream' + suffix], OSC = api['ObservationStreamCollection' + suffix], OC = api['ObservationCollection' + suffix], S = api['Segmentation' + suffix], SC = api['SegmentationCollection' + suffix], J = api['JumpGroup' + suffix];
        const observation = new O(3, new Uint32Array([2, 1])), source = new OS(2, new A([1, 2, 3, 4, 5, 6])); observation.set_stream(0, source); source.replace(1, new A([99])); array(observation.frame(0, 1), [3, 4], 'native frame order'); equal(observation.frame(0, 3), undefined, 'frame range'); equal(observation.frame(2, 0), undefined, 'stream range');
        const copy = observation.cloned(), child = copy.stream(0), values = child.values(); values.fill(99); array(child.values(), [1, 2, 3, 4, 5, 6], 'copied values'); observation.free(); copy.free(); array(child.values(), [1, 2, 3, 4, 5, 6], 'child lifetime'); child.free(); source.free();
        for (const dimensions of [new Uint32Array(), new Uint32Array([0]), new Uint32Array([0x80000000])]) rejects(() => new O(1, dimensions), 'observation dimensions'); rejects(() => new O(0x80000000, new Uint32Array([1])), 'frame count');
        const zero = new O(0, new Uint32Array([1])); equal(zero.frames, 0, 'zero frames'); zero.validate(); zero.free();
        const rawBits = single ? new Uint32Array([0xffc00123, 0x80000000, 0x7f800000]) : new BigUint64Array([0xfff8000000000123n, 0x8000000000000000n, 0x7ff0000000000000n]); const raw = new OS(1, new A(rawBits.buffer)), streams = new OSC(), owner = new O(3, new Uint32Array([1])); streams.push(raw); owner.replace(3, streams); raw.free(); streams.free(); const rawChild = owner.stream(0); array(new B(rawChild.values().buffer), rawBits, 'observation IEEE bits'); rawChild.free(); const bad = new OS(2, new A([1])); rejects(() => owner.set_stream(0, bad), 'invalid child shape'); const retained = owner.stream(0); array(new B(retained.values().buffer), rawBits, 'atomic child error'); retained.free(); bad.free();
        const observations = new OC(); observations.push(owner); observations.push(owner); const collectionCopy = observations.cloned(); observations.clear(); owner.free(); observations.free(); const collected = collectionCopy.get(0); collectionCopy.free(); const collectedChild = collected.stream(0); array(new B(collectedChild.values().buffer), rawBits, 'nested observation copies'); collectedChild.free(); collected.free();
        const sourceSeg = new S(2, 5); sourceSeg.set_boundaries(new Int32Array([1, 3, 3, 6, 8])); sourceSeg.set_duration_states(new Int32Array([10, 11, 12, 13, 14])); sourceSeg.set_output_states(0, new Int32Array([20, 21, 22, 23, 24])); sourceSeg.set_output_states(1, new Int32Array([40, 39, 38, 37, 36])); sourceSeg.add_jump(0, 3, .25);
        const remapped = sourceSeg.resegment(new Uint32Array([0, 2, 3, 4, 3, 5, 1, 8])); array(remapped.boundaries(), [2, 4, 5, 8], 'path right boundaries'); array(remapped.duration_states(), [10, 13, 13, 11], 'repeated/skipped/backward states'); array(remapped.output_states(0), [20, 23, 23, 21], 'output state remap'); array(remapped.output_states(1), [40, 37, 37, 39], 'second output remap'); remappedStates += remapped.segments;
        for (let i = 0; i < remapped.segments; i++) { const jumps = remapped.outgoing(i); array(jumps.deltas(), [1], 'remapping resets jumps'); array(jumps.probabilities(), [1], 'remapping resets probability'); jumps.free(); }
        if (serialization) { const bytes = new Uint8Array(await load('c-resegmentation.bin')); array(remapped.write(), bytes, 'original C resegmentation'); wireBytes += bytes.length; }
        rejects(() => sourceSeg.resegment(new Uint32Array([1])), 'occurrence shape'); rejects(() => sourceSeg.resegment(new Uint32Array([5, 1])), 'occurrence state range'); rejects(() => sourceSeg.resegment(new Uint32Array([0, 4, 1, 3])), 'occurrence boundary order');
        rejects(() => sourceSeg.set_boundaries(new Int32Array([1])), 'boundary shape'); array(sourceSeg.boundaries(), [1, 3, 3, 6, 8], 'atomic boundaries'); rejects(() => sourceSeg.set_duration_states(new Int32Array([1])), 'state shape'); rejects(() => sourceSeg.set_output_states(2, new Int32Array(5)), 'output stream range');
        const invalidJumps = new J(new Int32Array([2]), new A([.5])); rejects(() => sourceSeg.set_outgoing(0, invalidJumps), 'missing final ordinary transition'); invalidJumps.free(); rejects(() => new J(new Int32Array([1]), new A([])), 'jump shape');
        const arbitraryJumps = new J(new Int32Array([0, 2, 1]), new A(rawBits.buffer)), jumpCopy = arbitraryJumps.cloned(); sourceSeg.set_outgoing(0, arbitraryJumps); const storedJumps = sourceSeg.outgoing(0); array(new B(storedJumps.probabilities().buffer), rawBits, 'jump IEEE bits'); storedJumps.free(); arbitraryJumps.free(); array(new B(jumpCopy.probabilities().buffer), rawBits, 'jump clone bits'); jumpCopy.free(); rejects(() => sourceSeg.build_transitions(), 'invalid stored transition probabilities');
        const segments = new SC(); segments.push(remapped); const segmentCopy = segments.cloned(); segments.clear(); segments.free(); remapped.free(); const retainedSegment = segmentCopy.get(0); segmentCopy.free(); array(retainedSegment.boundaries(), [2, 4, 5, 8], 'segmentation collection lifetime'); sourceSeg.replace(retainedSegment); retainedSegment.free(); sourceSeg.validate(); sourceSeg.free();
        // Replace every observation field, including frame/stream shape, and retain
        // the previous value when the replacement violates native invariants.
        const reshaped = new O(1, new Uint32Array([1])), replacementStreams = new OSC();
        for (const [dimensions, values] of [[2, [1, 2, 3, 4]], [1, [5, 6]]]) {
            const replacementStream = new OS(dimensions, new A(values));
            replacementStreams.push(replacementStream); replacementStream.free();
        }
        reshaped.replace(2, replacementStreams); replacementStreams.clear();
        equal(reshaped.frames, 2, 'replacement frame count'); equal(reshaped.streams, 2, 'replacement stream count');
        array(reshaped.frame(0, 1), [3, 4], 'replacement first stream'); array(reshaped.frame(1, 1), [6], 'replacement second stream');
        rejects(() => reshaped.replace(2, replacementStreams), 'empty replacement streams');
        const shortStream = new OS(1, new A([7])); replacementStreams.push(shortStream); shortStream.free();
        rejects(() => reshaped.replace(2, replacementStreams), 'replacement storage mismatch');
        equal(reshaped.frames, 2, 'atomic replacement frames'); equal(reshaped.streams, 2, 'atomic replacement streams');
        array(reshaped.frame(0, 1), [3, 4], 'atomic replacement contents');
        reshaped.replace(1, replacementStreams); replacementStreams.free();
        equal(reshaped.frames, 1, 'shrunk frame count'); equal(reshaped.streams, 1, 'shrunk stream count');
        array(reshaped.frame(0, 0), [7], 'shrunk contents after source release'); reshaped.free();

        // Graph owners retain all segmentation fields and their derived jumps.
        // Changing either the editable source or a returned copy cannot change
        // the cached graph, including after its source has been released.
        const editable = new S(1, 2);
        editable.set_boundaries(new Int32Array([2, 5])); editable.set_duration_states(new Int32Array([9, 4]));
        editable.set_output_states(0, new Int32Array([6, 3])); editable.add_jump(0, 0, .5);
        const preservedGraph = editable.build_transitions(), preservedClone = preservedGraph.cloned();
        const returnedSegmentation = preservedGraph.segmentation();
        returnedSegmentation.set_boundaries(new Int32Array([8, 9])); returnedSegmentation.add_jump(0, 1, .25);
        const smaller = new S(0, 1); editable.replace(smaller); smaller.free();
        equal(editable.streams, 0, 'replacement output stream shape'); equal(editable.segments, 1, 'replacement state shape');
        editable.free(); returnedSegmentation.free(); preservedGraph.free();
        const preserved = preservedClone.segmentation(), incoming = preservedClone.incoming(0), outgoing = preservedClone.outgoing(0);
        array(preserved.boundaries(), [2, 5], 'graph boundary snapshot'); array(preserved.duration_states(), [9, 4], 'graph duration snapshot');
        array(preserved.output_states(0), [6, 3], 'graph output snapshot'); array(incoming.deltas(), [0, -1], 'graph cached deltas');
        array(incoming.probabilities(), [.5, 1], 'graph cached probabilities'); array(outgoing.deltas(), [0, 1], 'graph outgoing snapshot');
        array(outgoing.probabilities(), [.5, 1], 'graph outgoing probabilities'); incoming.free(); outgoing.free(); preserved.free();
        const consumedSegmentation = preservedClone.into_segmentation();
        array(consumedSegmentation.boundaries(), [2, 5], 'consumed graph parameters'); consumedSegmentation.free();

        const graphSource = new S(0, 4);
        for (const [state, delta, probability] of [[0, 2, .2], [1, 0, .3], [2, -2, .4], [3, -2, .5], [0, -100, .1], [3, 100, .1]]) graphSource.add_jump(state, delta, Math.fround(probability));
        // Both precisions deliberately receive the exact original binary32 inputs.
        for (let state = 0; state < 4; state++) graphSource.add_jump(state, 1, Math.fround(Math.fround(.6) + Math.fround(state * Math.fround(.1))));
        const reference = new TextDecoder().decode(await load('c-transitions.txt')).trim().split(/\r?\n/);
        for (let pass = 0; pass < 2; pass++) { const graph = graphSource.build_transitions(), clone = graph.cloned(); for (const line of reference) { const [, state, index, delta, probability] = line.split(/\s+/).map(Number), incoming = graph.incoming(state); equal(incoming.deltas()[index], delta, line); equal(incoming.probabilities()[index], Math.fround(probability), line); incoming.free(); transitionRecords++; } equal(graph.incoming(4), undefined, 'incoming range'); equal(graph.outgoing(4), undefined, 'outgoing range'); graph.free(); const recovered = clone.into_segmentation(); const repeated = recovered.build_transitions_owned(); const snapshot = repeated.segmentation(); snapshot.free(); repeated.free(); }
        const graph = graphSource.cloned().build_transitions_owned(), childGraph = graph.incoming(0); graphSource.free(); graph.free(); assert(childGraph.length > 0, 'copied graph group lifetime'); childGraph.free();
        const extremes = new S(0, 2), precise = single ? Math.fround(1 + 2 ** -23) : 1 + Number.EPSILON, extremeJumps = new J(new Int32Array([-2147483648, 0, 1]), new A([.5, precise, -0])); extremes.set_outgoing(0, extremeJumps); extremeJumps.free();
        for (const value of [-1, NaN, Infinity]) rejects(() => extremes.add_jump(0, 0, value), 'invalid added probability'); rejects(() => extremes.add_jump(2, 0, 1), 'jump source range'); const extremeGraph = extremes.build_transitions_owned(), exactIncoming = extremeGraph.incoming(0), ordinaryIncoming = extremeGraph.incoming(1); array(exactIncoming.deltas(), [0, -1], 'out-of-range extreme excluded'); equal(exactIncoming.probabilities()[0], precise, 'selected-precision transition retained'); equal(ordinaryIncoming.probabilities()[ordinaryIncoming.length - 1], -0, 'signed-zero transition retained'); exactIncoming.free(); ordinaryIncoming.free(); extremeGraph.free();
        const preciseObservation = new O(1, new Uint32Array([1])), preciseStream = new OS(1, new A([precise])); preciseObservation.set_stream(0, preciseStream); preciseStream.free(); equal(preciseObservation.frame(0, 0)[0], precise, 'selected-precision feature retained'); preciseObservation.free();
        for (const shape of [[0x80000000, 0], [0, 0x80000000]]) rejects(() => new S(...shape), 'segmentation shape'); const empty = new S(0, 0); empty.validate(); const emptyGraph = empty.build_transitions_owned(); equal(emptyGraph.incoming(0), undefined, 'empty graph'); emptyGraph.free();
        if (serialization) {
            const fixtures = [[O, 'c-observation.bin'], ...[0, 1, 2].map(i => [O, `corrected-c-regroup-${i}-observation.bin`]), ...['c-segmentation.bin', 'c-copy-segmentation.bin', 'c-resegmentation.bin', ...[0, 1, 2].map(i => `corrected-c-regroup-${i}-segmentation.bin`)].map(name => [S, name])];
            for (const [Owner, name] of fixtures) {
                const bytes = new Uint8Array(await load(name)), value = Owner.read(bytes); array(value.write(), bytes, name + ' original wire'); wireBytes += bytes.length;
                let budget;
                if (Owner === O) { budget = 3 + value.streams * 2; for (let i = 0; i < value.streams; i++) { const stream = value.stream(i); budget += stream.values().length; stream.free(); } }
                else { budget = 3 + 3 * value.segments + value.streams; for (let i = 0; i < value.streams; i++) budget += value.output_states(i).length; for (let i = 0; i < value.segments; i++) { const jumps = value.outgoing(i); budget += 2 * jumps.length; jumps.free(); } }
                const limited = Owner.read_with_limits(bytes, budget); limited.free(); rejects(() => Owner.read_with_limits(bytes, budget - 1), 'strict budget');
                const joined = new Uint8Array(bytes.length + 1); joined.set(bytes); joined[joined.length - 1] = 42; rejects(() => Owner.read(joined), 'trailing object'); const prefix = Owner.read_prefix(joined); equal(prefix.position, BigInt(bytes.length), 'cursor'); const copied = prefix.parameters(); array(copied.write(), bytes, 'prefix values'); copied.free(); const consumed = prefix.into_parameters(); consumed.free(); const boundedPrefix = Owner.read_prefix_with_limits(joined, budget); boundedPrefix.free(); rejects(() => Owner.read_prefix_with_limits(joined, budget - 1), 'prefix budget'); for (let end = 0; end < bytes.length; end++) rejects(() => Owner.read_prefix(bytes.subarray(0, end)), 'truncated object'); value.free();
            }
        } else { for (const Owner of [O, S]) { for (const name of ['read', 'read_with_limits', 'read_prefix', 'read_prefix_with_limits']) assert(!(name in Owner), 'disabled codec'); assert(!('write' in Owner.prototype), 'disabled output'); } for (const name of ['DecodedObservation', 'DecodedSegmentation']) assert(!(name + suffix in api), 'disabled decoded owner'); }
    }
    equal(transitionRecords, 64, 'complete repeated transition corpus'); equal(remappedStates, 8, 'both precision resegmentation'); return { transitionRecords, remappedStates, wireBytes };
}
