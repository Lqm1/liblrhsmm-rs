"""Actual ctypes caller for every model-statistics export in both precisions."""
import ctypes as c
import sys

lib = c.CDLL(sys.argv[1])
P, N, U = c.c_void_p, c.c_size_t, c.c_uint32

class Specification(c.Structure):
    _fields_ = [('emissions', N), ('components', N), ('dimensions', N)]

class Layout(c.Structure):
    _fields_ = [('streams', N), ('durations', N)]

for suffix, scalar, maximum in [('f32', c.c_float, 3.4028234663852886e38), ('f64', c.c_double, sys.float_info.max)]:
    class Values(c.Structure):
        _fields_ = [('value_sum', scalar), ('squared_sum', scalar), ('occupancy', scalar)]
    class Parameters(c.Structure):
        _fields_ = [('mean', scalar), ('variance', scalar), ('minimum', c.c_int32), ('maximum', c.c_int32), ('fixed_mean', c.c_int32), ('variance_floor', scalar)]
    class Options(c.Structure):
        _fields_ = [('duration_mode', U), ('duration_variance_floor', scalar), ('emission_variance_floor', scalar)]
    signatures = {
        'model_new': [c.POINTER(Specification), N, N, c.POINTER(P)],
        'model_get_duration': [P, N, c.POINTER(P)],
        'model_release': [c.POINTER(P)],
        'model_statistics_create': [c.POINTER(P), N, c.POINTER(P), N, c.POINTER(P)],
        'model_statistics_replace': [P, c.POINTER(P), N, c.POINTER(P), N],
        'model_statistics_from_model': [P, c.POINTER(P)],
        'model_statistics_layout': [P, c.POINTER(Layout)],
        'model_statistics_get_stream': [P, N, c.POINTER(P)],
        'model_statistics_set_stream': [P, N, P],
        'model_statistics_get_duration': [P, N, c.POINTER(P)],
        'model_statistics_set_duration': [P, N, P],
        'model_statistics_clone': [P, c.POINTER(P)],
        'model_statistics_clear': [P],
        'model_statistics_release': [c.POINTER(P)],
        'model_update': [P, P, c.POINTER(Options)],
        'stream_statistics_release': [c.POINTER(P)],
        'duration_statistics_set': [P, c.POINTER(Values)],
        'duration_statistics_get': [P, c.POINTER(Values)],
        'duration_statistics_release': [c.POINTER(P)],
        'duration_get': [P, c.POINTER(Parameters)],
        'duration_release': [c.POINTER(P)],
        'update_options_default': [c.POINTER(Options)],
    }
    functions, called = {}, set()
    for name, arguments in signatures.items():
        function = getattr(lib, f'liblrhsmm_rs_{name}_{suffix}')
        function.argtypes, function.restype = arguments, U
        functions[name] = function
    def status(name, expected, *arguments):
        actual = functions[name](*arguments)
        assert actual == expected, (name, actual, expected)
        called.add(name)
    def call(name, *arguments):
        status(name, 0, *arguments)
    model, statistics, copy, assembled, stream, duration, parameters = [P() for _ in range(7)]
    specification = Specification(1, 1, 1)
    call('model_new', c.byref(specification), 1, 2, c.byref(model))
    call('model_statistics_from_model', model, c.byref(statistics))
    layout = Layout(99, 99)
    call('model_statistics_layout', statistics, c.byref(layout))
    assert (layout.streams, layout.durations) == (1, 2)
    call('model_statistics_get_stream', statistics, 0, c.byref(stream))
    call('model_statistics_set_stream', statistics, 0, stream)
    call('model_statistics_get_duration', statistics, 0, c.byref(duration))
    good, bad = Values(6, 20, 2), Values(maximum, 20, 2)
    call('duration_statistics_set', duration, c.byref(good))
    call('model_statistics_set_duration', statistics, 0, duration)
    call('duration_statistics_set', duration, c.byref(bad))
    call('model_statistics_set_duration', statistics, 1, duration)
    options = Options()
    call('update_options_default', c.byref(options))
    def read_duration(index):
        call('model_get_duration', model, index, c.byref(parameters))
        result = Parameters()
        call('duration_get', parameters, c.byref(result))
        call('duration_release', c.byref(parameters))
        return tuple(getattr(result, field) for field, _ in Parameters._fields_)
    before = [read_duration(index) for index in range(2)]
    status('model_update', 3, model, statistics, c.byref(options))
    assert [read_duration(index) for index in range(2)] == before
    call('duration_statistics_set', duration, c.byref(good))
    call('model_statistics_set_duration', statistics, 1, duration)
    call('model_statistics_clone', statistics, c.byref(copy))
    call('model_statistics_clear', statistics)
    call('model_update', model, copy, c.byref(options))
    assert all(read_duration(index)[:2] == (3, 1) for index in range(2))
    streams, durations = (P * 2)(stream.value, stream.value), (P * 2)(duration.value, duration.value)
    call('model_statistics_create', streams, 2, durations, 2, c.byref(assembled))
    call('stream_statistics_release', c.byref(stream))
    call('duration_statistics_release', c.byref(duration))
    call('model_statistics_get_duration', assembled, 1, c.byref(duration))
    observed = Values()
    call('duration_statistics_get', duration, c.byref(observed))
    assert (observed.value_sum, observed.squared_sum, observed.occupancy) == (6, 20, 2)
    call('duration_statistics_release', c.byref(duration))
    invalid = (P * 1)(None)
    status('model_statistics_replace', 1, assembled, None, 0, invalid, 1)
    call('model_statistics_layout', assembled, c.byref(layout))
    assert (layout.streams, layout.durations) == (2, 2)
    status('model_statistics_get_stream', 2, assembled, 2, c.byref(stream))
    status('model_statistics_get_duration', 2, assembled, 2, c.byref(duration))
    assert not stream.value and not duration.value
    call('model_statistics_replace', assembled, None, 0, None, 0)
    call('model_statistics_layout', assembled, c.byref(layout))
    assert (layout.streams, layout.durations) == (0, 0)
    call('model_release', c.byref(model))
    for owner in [statistics, copy, assembled]:
        call('model_statistics_release', c.byref(owner))
        assert not owner.value
    call('model_statistics_release', c.byref(assembled))
    status('model_statistics_release', 1, None)
    assert called == set(signatures), set(signatures) - called
print('model statistics: all24 exports, independent copies and late update failure passed')
