function assert(condition, message) {
    if (!condition) throw new Error(message);
}

/** Explicit native field list; defaults come from the four Rust Default impls. */
export function verifyConfigurationBindings(api) {
    let fields = 0;
    let alternatives = 0;
    for (const suffix of ['F32', 'F64']) {
        const cast = suffix === 'F32' ? Math.fround : value => value;
        for (const [name, defaults, integers] of [
            ['PreparationOptions', {duration_cache_length: 300, duration_weight: 1, temperature: 1}, ['duration_cache_length']],
            ['HsmmOptions', {temperature: 1, duration_weight: 1, state_radius: 5, duration_extra: 30, duration_extra_factor: 1}, ['duration_extra']],
            ['GeometricOptions', {temperature: 1, pruning_slope: 0.3}, []],
            ['UpdateOptions', {duration_mode: 0, duration_variance_floor: 0.05, emission_variance_floor: 0.01}, ['duration_mode']],
        ]) {
            const options = new api[name + suffix]();
            for (const [field, expected] of Object.entries(defaults)) {
                assert(options[field] === cast(expected), name + '.' + field + ' native default');
                const value = integers.includes(field) ? 1 : cast(1 + Number.EPSILON);
                options[field] = value;
                assert(options[field] === value, name + '.' + field + ' selected precision');
                fields++;
            }
            const copy = options.cloned();
            for (const field of Object.keys(defaults)) {
                options[field] = integers.includes(field) ? 0 : -0;
                assert(copy[field] === (integers.includes(field) ? 1 : cast(1 + Number.EPSILON)), 'configuration clone independence');
                if (!integers.includes(field)) assert(Object.is(options[field], -0), 'configuration signed zero');
            }
            options.free();
            for (const field of Object.keys(defaults)) {
                assert(copy[field] === (integers.includes(field) ? 1 : cast(1 + Number.EPSILON)), 'configuration clone after release');
            }
            copy.free();
        }
        for (const [name, modes] of [['EmissionPruning', [0, 1, 2]], ['MixtureOccupancyMode', [0, 1]]]) {
            for (const mode of modes) {
                const options = new api[name + suffix](mode, 0.5);
                assert(options.mode === mode && options.parameter === 0.5, 'all enum alternatives represented');
                options.parameter = cast(1 + Number.EPSILON);
                const copy = options.cloned();
                options.mode = 0xffffffff;
                options.parameter = NaN;
                assert(copy.mode === mode && copy.parameter === cast(1 + Number.EPSILON), 'enum clone retains selected precision');
                options.free(); copy.free(); alternatives++;
            }
        }
    }
    const constants = [...api.numerical_constants()];
    assert(JSON.stringify(constants) === JSON.stringify([7000, 0.001, 7000, 0.002, 1e-7, Math.fround(-1e15), -1e15]), 'all seven native numerical constants');
    assert(fields === 26 && alternatives === 10, 'complete audited configuration fields');
    return {fields, alternatives, numericalConstants: constants.length};
}
