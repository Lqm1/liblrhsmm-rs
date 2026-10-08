"""Actual ctypes initialization, external collection and full training pipelines."""
import ctypes as c
import math
import struct
import sys
lib = c.CDLL(sys.argv[1])
P, N, U, I = c.c_void_p, c.c_size_t, c.c_uint32, c.c_int32
serialization = '--no-serialization' not in sys.argv
class Layout(c.Structure):
    _fields_ = [('frames', N), ('states', N), ('values', N)]
class Specification(c.Structure):
    _fields_ = [('emissions', N), ('components', N), ('dimensions', N)]
class Limits(c.Structure):
    _fields_ = [('max_array_entries', N)]
def common(name, arguments):
    function = getattr(lib, 'liblrhsmm_rs_' + name)
    function.argtypes, function.restype = arguments, U
    return function
common_releases = {family: common(family + '_release', [c.POINTER(P)]) for family in ['duration_occupancies', 'transition_occupancies', 'mixture_occupancies']}
if serialization:
    byte_length = common('bytes_length', [P, c.POINTER(N)])
    byte_copy = common('bytes_copy', [P, N, c.POINTER(c.c_uint8), N])
    byte_release = common('bytes_release', [c.POINTER(P)])
    default_limits = common('decode_limits_default', [c.POINTER(Limits)])

for suffix, scalar, code, files in [
    ('f32', c.c_float, 'f', ['c-emission-tables.txt', 'c-fractional-emissions.txt']),
    ('f64', c.c_double, 'd', ['c-double-emissions.txt', 'c-double-emissions-fractional.txt']),
]:
    class Settings(c.Structure):
        _fields_ = [('temperature', scalar), ('pruning', U), ('radius', scalar), ('slope', scalar)]

    class Gaussian(c.Structure):
        _fields_ = [('mixtures', N), ('dimensions', N), ('weights', c.POINTER(scalar)), ('means', c.POINTER(scalar)), ('variances', c.POINTER(scalar)), ('variance_floors', c.POINTER(scalar))]

    class Duration(c.Structure):
        _fields_ = [('mean', scalar), ('variance', scalar), ('minimum', I), ('maximum', I), ('fixed_mean', I), ('variance_floor', scalar)]

    class Preparation(c.Structure):
        _fields_ = [('duration_cache_length', N), ('duration_weight', scalar), ('temperature', scalar)]

    class Jump(c.Structure):
        _fields_ = [('delta', I), ('probability', scalar)]

    class JumpSlice(c.Structure):
        _fields_ = [('jumps', c.POINTER(Jump)), ('count', N)]

    functions, tracked, called = {}, set(), set()

    def register(name, arguments, new=False):
        function = getattr(lib, f'liblrhsmm_rs_{name}_{suffix}')
        function.argtypes, function.restype = arguments, U
        functions[name] = function
        if new:
            tracked.add(name)

    def status(name, expected, *arguments):
        actual = functions[name](*arguments)
        assert actual == expected, (suffix, name, actual, expected)
        if name in tracked:
            called.add(name)

    def call(name, *arguments):
        status(name, 0, *arguments)

    def release(family, handle):
        call(f'{family}_release', c.byref(handle))
        assert handle.value is None

    register('emission_settings_default', [c.POINTER(Settings)])
    for owner in ['model', 'prepared_model']:
        for method in ['emission_log_probabilities', 'emission_probabilities']:
            register(f'{owner}_{method}', [P, P, P, c.POINTER(Settings), c.POINTER(P)])
    assert not tracked
    register('gaussian_create', [c.POINTER(Gaussian), c.POINTER(P)])
    register('stream_create', [scalar, c.POINTER(P), N, c.POINTER(P)])
    register('duration_default', [c.POINTER(Duration)])
    register('duration_create', [c.POINTER(Duration), c.POINTER(P)])
    register('model_create', [c.POINTER(P), N, c.POINTER(P), N, c.POINTER(P)])
    register('model_prepare', [P, c.POINTER(Preparation), c.POINTER(P)])
    register('observation_stream_create', [N, c.POINTER(scalar), N, c.POINTER(P)])
    register('observation_create', [N, c.POINTER(P), N, c.POINTER(P)])
    register('segmentation_create', [N, N, c.POINTER(I), c.POINTER(I), c.POINTER(c.POINTER(I)), c.POINTER(JumpSlice), c.POINTER(P)])
    for family in ['log_emissions', 'emission_densities']:
        register(f'{family}_layout', [P, c.POINTER(Layout)])
        register(f'{family}_copy_values', [P, N, c.POINTER(scalar), N])
    for family in ['gaussian', 'stream', 'duration', 'model', 'prepared_model', 'observation_stream', 'observation', 'segmentation', 'log_emissions', 'emission_densities']:
        register(f'{family}_release', [c.POINTER(P)])


    class Hsmm(c.Structure):
        _fields_ = [('temperature', scalar), ('duration_weight', scalar), ('state_radius', scalar), ('duration_extra', N), ('duration_extra_factor', scalar)]
    class Geometric(c.Structure):
        _fields_ = [('temperature', scalar), ('pruning_slope', scalar)]
    class Update(c.Structure):
        _fields_ = [('duration_mode', U), ('duration_variance_floor', scalar), ('emission_variance_floor', scalar)]
    class ModelLayout(c.Structure):
        _fields_ = [('streams', N), ('durations', N)]
    class GaussianLayout(c.Structure):
        _fields_ = [('mixtures', N), ('dimensions', N), ('state_occupancy', scalar)]
    class DurationValues(c.Structure):
        _fields_ = [('value_sum', scalar), ('squared_sum', scalar), ('occupancy', scalar)]
    register('model_statistics_collect_initialization', [P, P, P, P], True)
    for method in ['hsmm', 'geometric']:
        register('model_statistics_collect_' + method, [P, P, P, P, P, P, c.POINTER(P), N, scalar], True)
    register('model_statistics_estimate_hsmm', [P, P, P, P, c.POINTER(Hsmm), c.POINTER(scalar)], True)
    register('model_statistics_estimate_geometric', [P, P, P, P, c.POINTER(Geometric), c.POINTER(scalar)], True)
    assert len(tracked) == 5
    register('model_new', [c.POINTER(Specification), N, N, c.POINTER(P)])
    register('model_clone', [P, c.POINTER(P)])
    register('model_statistics_from_model', [P, c.POINTER(P)])
    register('model_statistics_layout', [P, c.POINTER(ModelLayout)])
    register('model_statistics_get_stream', [P, N, c.POINTER(P)])
    register('model_statistics_get_duration', [P, N, c.POINTER(P)])
    register('stream_statistics_count', [P, c.POINTER(N)])
    register('stream_statistics_get_emission', [P, N, c.POINTER(P)])
    register('gaussian_statistics_layout', [P, c.POINTER(GaussianLayout)])
    register('gaussian_statistics_copy_field', [P, U, N, c.POINTER(scalar), N])
    register('duration_statistics_get', [P, c.POINTER(DurationValues)])
    register('update_options_default', [c.POINTER(Update)])
    register('model_update', [P, P, c.POINTER(Update)])
    register('hsmm_settings_default', [c.POINTER(Hsmm)])
    register('model_viterbi_hsmm', [P, P, P, c.POINTER(Hsmm), c.POINTER(P)])
    register('hsmm_path_log_probability', [P, c.POINTER(scalar)])
    for method in ['forward', 'backward']:
        register('model_' + method + '_hsmm', [P, P, P, c.POINTER(Hsmm), c.POINTER(P)])
        register('model_' + method + '_geometric', [P, P, P, c.POINTER(Geometric), c.POINTER(P)])
    register('hsmm_scores_get_end', [P, c.POINTER(P)])
    register('state_scores_total_through_state', [P, P, N, c.POINTER(scalar)])
    register('state_scores_backward_total', [P, P, c.POINTER(scalar)])
    register('model_duration_occupancies', [P, P, P, P, P, scalar, c.POINTER(Hsmm), c.POINTER(P)])
    register('model_hsmm_state_occupancies', [P, P, P, P, c.POINTER(Hsmm), c.POINTER(P)])
    register('state_scores_geometric_occupancies', [P, P, scalar, scalar, c.POINTER(P)])
    register('model_geometric_transition_occupancies', [P, P, P, P, P, scalar, c.POINTER(Geometric), c.POINTER(P)])
    for method in ['hsmm', 'geometric']:
        register('model_mixture_occupancies_' + method, [P, P, P, P, N, scalar, scalar, c.POINTER(P)])
    for family in ['model_statistics', 'stream_statistics', 'gaussian_statistics', 'duration_statistics', 'hsmm_path', 'hsmm_scores', 'state_scores', 'state_occupancies']:
        register(family + '_release', [c.POINTER(P)])
    if serialization:
        register('model_write_bytes', [P, U, c.POINTER(P)])
        register('model_read_bytes', [c.POINTER(c.c_uint8), N, c.POINTER(Limits), c.POINTER(P)])

    def snapshot(owner):
        layout = ModelLayout()
        call('model_statistics_layout', owner, c.byref(layout))
        assert (layout.streams, layout.durations) == (2, 3)
        result = bytearray()
        for stream in range(2):
            s, count = P(), N()
            call('model_statistics_get_stream', owner, stream, c.byref(s))
            call('stream_statistics_count', s, c.byref(count))
            assert count.value == 3
            for state in range(3):
                g, shape = P(), GaussianLayout()
                call('stream_statistics_get_emission', s, state, c.byref(g))
                call('gaussian_statistics_layout', g, c.byref(shape))
                result += struct.pack('=QQ' + code, shape.mixtures, shape.dimensions, shape.state_occupancy)
                for field in range(3):
                    length = shape.mixtures if field == 2 else shape.mixtures * shape.dimensions
                    values = (scalar * length)()
                    call('gaussian_statistics_copy_field', g, field, 0, values, length)
                    result += bytes(values)
                release('gaussian_statistics', g)
            release('stream_statistics', s)
        for state in range(3):
            d, values = P(), DurationValues()
            call('model_statistics_get_duration', owner, state, c.byref(d))
            call('duration_statistics_get', d, c.byref(values))
            result += bytes(values)
            release('duration_statistics', d)
        return bytes(result)

    def infer(model, observation, segmentation):
        settings, hsmm = Settings(1, 0, 0, 0), Hsmm()
        call('hsmm_settings_default', c.byref(hsmm))
        log, path, probability = P(), P(), scalar()
        call('model_emission_log_probabilities', model, observation, segmentation, c.byref(settings), c.byref(log))
        call('model_viterbi_hsmm', model, segmentation, log, c.byref(hsmm), c.byref(path))
        call('hsmm_path_log_probability', path, c.byref(probability))
        assert math.isfinite(probability.value)
        release('hsmm_path', path)
        release('log_emissions', log)
        return bytes(probability)

    def encode(model):
        owner, length = P(), N()
        call('model_write_bytes', model, 0, c.byref(owner))
        assert byte_length(owner, c.byref(length)) == 0
        values = (c.c_uint8 * length.value)()
        assert byte_copy(owner, 0, values, len(values)) == 0
        assert byte_release(c.byref(owner)) == 0
        return values

    streams, features = [], []
    for stream in range(2):
        components = dimensions = 2 if stream == 0 else 1
        emissions = []
        for state in range(3):
            weights = (scalar * components)(*[1 / components] * components)
            means = (scalar * (components * dimensions))(*[state * 0.5 + index * 0.25 for index in range(components * dimensions)])
            variances = (scalar * (components * dimensions))(*[0.75 + index * 0.25 for index in range(components * dimensions)])
            floors = (scalar * (components * dimensions))()
            descriptor = Gaussian(components, dimensions, weights, means, variances, floors)
            handle = P()
            call('gaussian_create', c.byref(descriptor), c.byref(handle))
            emissions.append(handle)
        borrowed = (P * 3)(*[handle.value for handle in emissions])
        handle = P()
        call('stream_create', 0.5 if stream == 0 else 1.25, borrowed, 3, c.byref(handle))
        streams.append(handle)
        for child in emissions:
            release('gaussian', child)
        samples = (scalar * (12 * dimensions))(*[frame * 0.125 + dimension * 0.5 for frame in range(12) for dimension in range(dimensions)])
        handle = P()
        call('observation_stream_create', dimensions, samples, len(samples), c.byref(handle))
        features.append(handle)
    durations = []
    for _ in range(3):
        descriptor = Duration()
        call('duration_default', c.byref(descriptor))
        descriptor.mean, descriptor.variance = 4, 1
        handle = P()
        call('duration_create', c.byref(descriptor), c.byref(handle))
        durations.append(handle)
    model, observation, segmentation, prepared = P(), P(), P(), P()
    call('model_create', (P * 2)(*[v.value for v in streams]), 2, (P * 3)(*[v.value for v in durations]), 3, c.byref(model))
    call('observation_create', 12, (P * 2)(*[v.value for v in features]), 2, c.byref(observation))
    for family, handles in [('stream', streams), ('duration', durations), ('observation_stream', features)]:
        for handle in handles:
            release(family, handle)
    labels = [(I * 3)(0, 1, 2), (I * 3)(2, 1, 0)]
    rows = (c.POINTER(I) * 2)(*labels)
    jump = (Jump * 1)(Jump(1, 1))
    slices = (JumpSlice * 3)(*[JumpSlice(jump, 1) for _ in range(3)])
    call('segmentation_create', 2, 3, (I * 3)(4, 8, 12), (I * 3)(0, 1, 2), rows, slices, c.byref(segmentation))

    initial, initial_statistics = P(), P()
    specs = (Specification * 2)(Specification(3, 1, 2), Specification(3, 1, 1))
    call('model_new', specs, 2, 3, c.byref(initial))
    call('model_statistics_from_model', initial, c.byref(initial_statistics))
    for repeat in range(2):
        call('model_statistics_collect_initialization', initial_statistics, initial, observation, segmentation)
    values = snapshot(initial_statistics)
    assert values[-9 * c.sizeof(scalar):] == struct.pack('=' + code * 9, *([8, 32, 2] * 3))
    update = Update()
    call('update_options_default', c.byref(update))
    call('model_update', initial, initial_statistics, c.byref(update))
    infer(initial, observation, segmentation)
    release('model_statistics', initial_statistics)
    release('model', initial)
    for mode in range(2):
        manual, automatic = P(), P()
        call('model_statistics_from_model', model, c.byref(manual))
        call('model_statistics_from_model', model, c.byref(automatic))
        hsmm, geo = Hsmm(), Geometric(0.75, 0.5)
        call('hsmm_settings_default', c.byref(hsmm))
        hsmm.temperature, hsmm.state_radius = 0.75, 1
        emission = Settings(0.75, 1 if mode == 0 else 2, 1, 0.5)
        log, state, durations, transitions, hf, hb, gf, gb = [P() for _ in range(8)]
        call('model_emission_log_probabilities', model, observation, segmentation, c.byref(emission), c.byref(log))
        total = scalar()
        if mode == 0:
            call('model_forward_hsmm', model, segmentation, log, c.byref(hsmm), c.byref(hf))
            call('model_backward_hsmm', model, segmentation, log, c.byref(hsmm), c.byref(hb))
            call('hsmm_scores_get_end', hf, c.byref(gf))
            call('hsmm_scores_get_end', hb, c.byref(gb))
            call('state_scores_total_through_state', gf, gb, 0, c.byref(total))
            call('model_duration_occupancies', model, segmentation, log, hf, hb, total, c.byref(hsmm), c.byref(durations))
            call('model_hsmm_state_occupancies', model, segmentation, log, durations, c.byref(hsmm), c.byref(state))
        else:
            call('model_forward_geometric', model, segmentation, log, c.byref(geo), c.byref(gf))
            call('model_backward_geometric', model, segmentation, log, c.byref(geo), c.byref(gb))
            call('state_scores_backward_total', gb, log, c.byref(total))
            call('state_scores_geometric_occupancies', gf, gb, total, geo.pruning_slope, c.byref(state))
            call('model_geometric_transition_occupancies', model, segmentation, log, gf, gb, total, c.byref(geo), c.byref(transitions))
        mixtures = [P(), P()]
        for stream in range(2):
            call('model_mixture_occupancies_' + ('hsmm' if mode == 0 else 'geometric'), model, observation, segmentation, state, stream, 0.75, 1 if mode == 0 else 0.5, c.byref(mixtures[stream]))
        borrowed = (P * 2)(*[v.value for v in mixtures])
        method, posterior, options, pruning = ('hsmm', durations, hsmm, 1) if mode == 0 else ('geometric', transitions, geo, 0.5)
        for repeat in range(2):
            call('model_statistics_collect_' + method, manual, model, observation, segmentation, posterior, state, borrowed, 2, pruning)
            actual = scalar(99)
            call('model_statistics_estimate_' + method, automatic, model, observation, segmentation, c.byref(options), c.byref(actual))
            assert bytes(actual) == bytes(total)
            assert snapshot(manual) == snapshot(automatic)
        before = snapshot(automatic)
        status('model_statistics_estimate_hsmm', 1, automatic, model, observation, segmentation, c.byref(hsmm), None)
        status('model_statistics_estimate_geometric', 1, automatic, model, observation, segmentation, c.byref(geo), None)
        hsmm.temperature = geo.temperature = math.nan
        sentinel = scalar(99)
        status('model_statistics_estimate_hsmm', 3, automatic, model, observation, segmentation, c.byref(hsmm), c.byref(sentinel))
        status('model_statistics_estimate_geometric', 3, automatic, model, observation, segmentation, c.byref(geo), c.byref(sentinel))
        assert sentinel.value == 99 and snapshot(automatic) == before
        before = snapshot(manual)
        bad = (P * 2)(borrowed[0], None)
        status('model_statistics_collect_' + method, 1, manual, model, observation, segmentation, posterior, state, bad, 2, pruning)
        status('model_statistics_collect_' + method, 3, manual, model, observation, segmentation, posterior, state, borrowed, 1, pruning)
        assert snapshot(manual) == before
        trained = P()
        call('model_clone', model, c.byref(trained))
        call('update_options_default', c.byref(update))
        update.duration_mode = mode
        call('model_update', trained, automatic, c.byref(update))
        likelihood = infer(trained, observation, segmentation)
        if serialization:
            data = encode(trained)
            limits, decoded = Limits(), P()
            assert default_limits(c.byref(limits)) == 0
            call('model_read_bytes', data, len(data), c.byref(limits), c.byref(decoded))
            release('model', trained)
            trained = decoded
            roundtrip = infer(trained, observation, segmentation)
            if suffix == 'f32':
                assert likelihood == roundtrip
            assert bytes(encode(trained)) == bytes(data)
            assert infer(trained, observation, segmentation) == roundtrip
        release('model', trained)
        for family, handle in [('duration_occupancies', durations), ('transition_occupancies', transitions), *[('mixture_occupancies', v) for v in mixtures]]:
            assert common_releases[family](c.byref(handle)) == 0 and handle.value is None
        for family, handle in [('state_occupancies', state), ('state_scores', gf), ('state_scores', gb), ('hsmm_scores', hf), ('hsmm_scores', hb), ('log_emissions', log), ('model_statistics', manual), ('model_statistics', automatic)]:
            release(family, handle)
    for family, handle in [('model', model), ('observation', observation), ('segmentation', segmentation)]:
        release(family, handle)
    assert called == tracked, (suffix, tracked - called)
print('Training Python: all10 exports, complete statistics and update/codec/inference pipelines passed')
