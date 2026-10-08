"""Actual ctypes calls to all prepared stream/model exports in both precisions."""
import ctypes as c
import sys

lib = c.CDLL(sys.argv[1])
P, N, U = c.c_void_p, c.c_size_t, c.c_uint32
class ModelLayout(c.Structure):
    _fields_ = [('streams', N), ('durations', N)]

for suffix, scalar in [('f32', c.c_float), ('f64', c.c_double)]:
    class GaussianParameters(c.Structure):
        _fields_ = [('mixtures', N), ('dimensions', N)] + [(name, c.POINTER(scalar)) for name in ['weights', 'means', 'variances', 'variance_floors']]
    class StreamLayout(c.Structure):
        _fields_ = [('weight', scalar), ('emissions', N)]
    class DurationParameters(c.Structure):
        _fields_ = [('mean', scalar), ('variance', scalar), ('minimum', c.c_int32), ('maximum', c.c_int32), ('fixed_mean', c.c_int32), ('variance_floor', scalar)]
    class Options(c.Structure):
        _fields_ = [('duration_cache_length', N), ('duration_weight', scalar), ('temperature', scalar)]
    signatures = {
        'stream_prepare': [P, c.POINTER(P)],
        'prepared_stream_parameters': [P, c.POINTER(P)],
        'prepared_stream_layout': [P, c.POINTER(StreamLayout)],
        'prepared_stream_get_emission': [P, N, c.POINTER(P)],
        'prepared_stream_clone': [P, c.POINTER(P)],
        'prepared_stream_release': [c.POINTER(P)],
        'model_prepare': [P, c.POINTER(Options), c.POINTER(P)],
        'model_prepare_selected_durations': [P, c.POINTER(Options), c.POINTER(N), N, c.POINTER(P)],
        'prepared_model_parameters': [P, c.POINTER(P)],
        'prepared_model_layout': [P, c.POINTER(ModelLayout)],
        'prepared_model_options': [P, c.POINTER(Options)],
        'prepared_model_stream_layout': [P, N, c.POINTER(StreamLayout)],
        'prepared_model_stream_parameters': [P, N, c.POINTER(P)],
        'prepared_model_has_duration': [P, N, c.POINTER(U)],
        'prepared_model_get_duration': [P, N, c.POINTER(P)],
        'prepared_model_get_emission': [P, N, N, c.POINTER(P)],
        'prepared_model_clone': [P, c.POINTER(P)],
        'prepared_model_release': [c.POINTER(P)],
        'gaussian_create': [c.POINTER(GaussianParameters), c.POINTER(P)],
        'gaussian_release': [c.POINTER(P)],
        'stream_create': [scalar, c.POINTER(P), N, c.POINTER(P)],
        'stream_set_weight': [P, scalar],
        'stream_release': [c.POINTER(P)],
        'duration_create': [c.POINTER(DurationParameters), c.POINTER(P)],
        'duration_release': [c.POINTER(P)],
        'model_create': [c.POINTER(P), N, c.POINTER(P), N, c.POINTER(P)],
        'model_release': [c.POINTER(P)],
        'prepared_gaussian_cache_length': [P, c.POINTER(N)],
        'prepared_gaussian_release': [c.POINTER(P)],
        'prepared_duration_cache_length': [P, c.POINTER(N)],
        'prepared_duration_release': [c.POINTER(P)],
    }
    functions, called = {}, set()
    for name, args in signatures.items():
        fn = getattr(lib, f'liblrhsmm_rs_{name}_{suffix}')
        fn.argtypes, fn.restype = args, U
        functions[name] = fn
    def status(name, expected, *args):
        actual = functions[name](*args)
        assert actual == expected, (name, expected, actual)
        called.add(name)
    def call(name, *args):
        status(name, 0, *args)
    gaussian, stream, prepared_stream, stream_copy, editable_stream, child = [P() for _ in range(6)]
    arrays = [(scalar * 1)(value) for value in [1, 0, 1, 0.1]]
    call('gaussian_create', c.byref(GaussianParameters(1, 1, *arrays)), c.byref(gaussian))
    call('stream_create', 0.75, (P * 1)(gaussian.value), 1, c.byref(stream))
    call('gaussian_release', c.byref(gaussian))
    call('stream_prepare', stream, c.byref(prepared_stream))
    stream_layout = StreamLayout(99, 99)
    call('prepared_stream_layout', prepared_stream, c.byref(stream_layout))
    assert (stream_layout.weight, stream_layout.emissions) == (0.75, 1)
    call('prepared_stream_parameters', prepared_stream, c.byref(editable_stream))
    call('stream_set_weight', editable_stream, -99)
    call('stream_release', c.byref(editable_stream))
    call('prepared_stream_clone', prepared_stream, c.byref(stream_copy))
    status('prepared_stream_get_emission', 2, stream_copy, 1, c.byref(child))
    assert not child.value
    call('prepared_stream_get_emission', stream_copy, 0, c.byref(child))
    call('prepared_stream_release', c.byref(prepared_stream))
    call('prepared_stream_release', c.byref(stream_copy))
    count = N(99)
    call('prepared_gaussian_cache_length', child, c.byref(count))
    assert count.value == 1
    call('prepared_gaussian_release', c.byref(child))
    duration, model, prepared_model, model_copy, editable_model, prepared_duration = [P() for _ in range(6)]
    call('duration_create', c.byref(DurationParameters(4, 1, 0, 300, -1, 0.05)), c.byref(duration))
    call('model_create', (P * 1)(stream.value), 1, (P * 2)(duration.value, duration.value), 2, c.byref(model))
    call('stream_release', c.byref(stream))
    call('duration_release', c.byref(duration))
    options, fetched = Options(3, 0.75, 0.5), Options(99, 99, 99)
    call('model_prepare', model, c.byref(options), c.byref(prepared_model))
    call('prepared_model_clone', prepared_model, c.byref(model_copy))
    layout = ModelLayout(99, 99)
    call('prepared_model_layout', model_copy, c.byref(layout))
    assert (layout.streams, layout.durations) == (1, 2)
    call('prepared_model_options', model_copy, c.byref(fetched))
    assert (fetched.duration_cache_length, fetched.duration_weight, fetched.temperature) == (3, 0.75, 0.5)
    call('prepared_model_parameters', model_copy, c.byref(editable_model))
    call('model_release', c.byref(editable_model))
    call('prepared_model_stream_layout', model_copy, 0, c.byref(stream_layout))
    assert (stream_layout.weight, stream_layout.emissions) == (0.75, 1)
    call('prepared_model_stream_parameters', model_copy, 0, c.byref(editable_stream))
    call('stream_release', c.byref(editable_stream))
    available = U(99)
    call('prepared_model_has_duration', model_copy, 1, c.byref(available))
    assert available.value == 1
    call('prepared_model_get_duration', model_copy, 1, c.byref(prepared_duration))
    call('prepared_model_get_emission', model_copy, 0, 0, c.byref(child))
    retained = child.value
    status('prepared_model_get_emission', 2, model_copy, 1, 0, c.byref(child))
    assert child.value == retained
    call('prepared_model_release', c.byref(prepared_model))
    call('prepared_model_release', c.byref(model_copy))
    call('prepared_duration_cache_length', prepared_duration, c.byref(count))
    assert count.value == 3
    call('prepared_duration_release', c.byref(prepared_duration))
    call('prepared_gaussian_release', c.byref(child))
    call('model_prepare_selected_durations', model, c.byref(options), (N * 2)(0, 0), 2, c.byref(prepared_model))
    call('prepared_model_has_duration', prepared_model, 1, c.byref(available))
    assert available.value == 0
    status('prepared_model_get_duration', 3, prepared_model, 1, c.byref(prepared_duration))
    assert not prepared_duration.value
    status('prepared_model_has_duration', 2, prepared_model, 2, c.byref(available))
    assert available.value == 0
    call('prepared_model_release', c.byref(prepared_model))
    call('model_prepare_selected_durations', model, c.byref(options), None, 0, c.byref(prepared_model))
    call('model_release', c.byref(model))
    call('prepared_model_layout', prepared_model, c.byref(layout))
    assert layout.durations == 2
    call('prepared_model_release', c.byref(prepared_model))
    call('prepared_model_release', c.byref(prepared_model))
    call('prepared_stream_release', c.byref(prepared_stream))
    status('prepared_model_release', 1, None)
    status('prepared_stream_release', 1, None)
    assert called == set(signatures), set(signatures) - called
print('prepared containers: all36 exports and independent child lifetimes passed')
