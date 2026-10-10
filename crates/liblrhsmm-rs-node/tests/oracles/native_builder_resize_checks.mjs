function assert(condition, message) {
    if (!condition) throw new Error(message);
}
function throws(action, message) {
    let failed = false;
    try {action();} catch {failed = true;}
    assert(failed, message);
}
export function verifyBuilderResize(api) {
    for (const suffix of ['F32', 'F64']) {
        const T = suffix === 'F32' ? Float32Array : Float64Array;
        const gaussian = new api['Gaussian' + suffix](1, 1);
        gaussian.replace(1, new T([1]), new T([3]), new T([2]), new T([0.5]));
        const streamBuilder = new api['StreamBuilder' + suffix](2);
        streamBuilder.set_weight(new T([0.75]));
        streamBuilder.set_emission(0, gaussian);
        streamBuilder.resize(4);
        assert(streamBuilder.emissions === 4 && streamBuilder.has_emission(0)
            && !streamBuilder.has_emission(1) && !streamBuilder.has_emission(3), 'grow retains populated and empty slots');
        assert(streamBuilder.weight()[0] === 0.75, 'resize retains stream weight');
        streamBuilder.set_emission(3, gaussian);
        streamBuilder.resize(1);
        streamBuilder.resize(4);
        assert(!streamBuilder.has_emission(3), 'removed slots do not return');
        throws(() => streamBuilder.resize(0x80000000), 'invalid stream resize');
        assert(streamBuilder.emissions === 4 && streamBuilder.has_emission(0), 'failed stream resize atomic');
        const streamCopy = streamBuilder.cloned();
        streamBuilder.resize(0);
        assert(streamBuilder.emissions === 0 && streamCopy.emissions === 4 && streamCopy.has_emission(0), 'resize clone independence');
        streamCopy.resize(1);
        const stream = streamCopy.build();
        streamBuilder.free(); gaussian.free();
        const duration = new api['Duration' + suffix]();
        duration.set_values(new T([3, 2, 0.5]));
        const modelBuilder = new api['ModelBuilder' + suffix](2, 2);
        modelBuilder.set_stream(0, stream);
        modelBuilder.set_duration(0, duration);
        modelBuilder.resize(4, 3);
        assert(modelBuilder.streams === 4 && modelBuilder.durations === 3
            && modelBuilder.has_stream(0) && modelBuilder.has_duration(0)
            && !modelBuilder.has_stream(3) && !modelBuilder.has_duration(2), 'model resize retains optional slots');
        modelBuilder.set_stream(3, stream);
        modelBuilder.set_duration(2, duration);
        for (const sizes of [[0x80000000, 1], [1, 0x80000000]]) {
            throws(() => modelBuilder.resize(...sizes), 'invalid model resize');
            assert(modelBuilder.streams === 4 && modelBuilder.durations === 3
                && modelBuilder.has_stream(3) && modelBuilder.has_duration(2), 'both model arrays unchanged on failure');
        }
        modelBuilder.resize(1, 1);
        modelBuilder.resize(4, 3);
        assert(!modelBuilder.has_stream(3) && !modelBuilder.has_duration(2), 'model removed slots stay absent');
        const modelCopy = modelBuilder.cloned();
        modelBuilder.resize(0, 0);
        assert(modelCopy.streams === 4 && modelCopy.durations === 3, 'independent model builder lengths');
        stream.free(); duration.free(); modelBuilder.free();
        modelCopy.resize(1, 1);
        const model = modelCopy.build();
        model.validate();
        const retained = model.stream(0);
        const emission = retained.emission(0);
        assert(retained.weight()[0] === 0.75 && emission.means()[0] === 3, 'resized build retains all parameters');
        emission.free(); retained.free(); model.free();
    }
    return {precisions: 2, builderKinds: 2};
}
