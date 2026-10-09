"""Exercise all four resize exports through an actual shared library."""
import ctypes as c
import sys

library = c.CDLL(sys.argv[1])
owner = c.c_void_p
size = c.c_size_t
status = c.c_uint32

class ModelLayout(c.Structure):
    _fields_ = [('streams', size), ('durations', size)]

def bind(name, arguments):
    function = getattr(library, 'liblrhsmm_rs_' + name)
    function.argtypes = arguments
    function.restype = status
    return function

def check(function, *arguments, expected=0):
    actual = function(*arguments)
    if actual != expected:
        raise AssertionError((function.__name__, actual, expected))

for precision, scalar in [(32, c.c_float), (64, c.c_double)]:
    suffix = f'_f{precision}'
    class StreamLayout(c.Structure):
        _fields_ = [('weight', scalar), ('emissions', size)]
    class DurationParameters(c.Structure):
        _fields_ = [('mean', scalar), ('variance', scalar), ('minimum', c.c_int32),
                    ('maximum', c.c_int32), ('fixed_mean', c.c_int32), ('variance_floor', scalar)]
    new_stream_builder = bind('stream_builder_new' + suffix, [size, c.POINTER(owner)])
    resize_stream = bind('stream_builder_resize' + suffix, [owner, size])
    stream_shape = bind('stream_builder_layout' + suffix, [owner, c.POINTER(StreamLayout)])
    set_weight = bind('stream_builder_set_weight' + suffix, [owner, scalar])
    set_emission = bind('stream_builder_set_emission' + suffix, [owner, size, owner])
    has_emission = bind('stream_builder_has_emission' + suffix, [owner, size, c.POINTER(status)])
    gaussian_new = bind('gaussian_new' + suffix, [size, size, c.POINTER(owner)])
    gaussian_release = bind('gaussian_release' + suffix, [c.POINTER(owner)])
    stream_builder_release = bind('stream_builder_release' + suffix, [c.POINTER(owner)])
    clone_stream = bind('stream_builder_clone' + suffix, [owner, c.POINTER(owner)])
    gaussian = owner()
    stream_builder = owner()
    check(gaussian_new, 1, 1, c.byref(gaussian))
    check(new_stream_builder, 2, c.byref(stream_builder))
    check(set_weight, stream_builder, 0.75)
    check(set_emission, stream_builder, 0, gaussian)
    check(resize_stream, stream_builder, 4)
    present = status()
    check(has_emission, stream_builder, 0, c.byref(present))
    assert present.value == 1
    check(has_emission, stream_builder, 3, c.byref(present))
    assert present.value == 0
    check(set_emission, stream_builder, 3, gaussian)
    check(resize_stream, stream_builder, 1)
    check(resize_stream, stream_builder, 4)
    check(has_emission, stream_builder, 3, c.byref(present))
    assert present.value == 0
    check(resize_stream, stream_builder, 0x80000000, expected=3)
    shape = StreamLayout()
    check(stream_shape, stream_builder, c.byref(shape))
    assert (shape.weight, shape.emissions) == (0.75, 4)
    saved = owner()
    check(clone_stream, stream_builder, c.byref(saved))
    check(resize_stream, stream_builder, 0)
    check(stream_shape, saved, c.byref(shape))
    assert (shape.weight, shape.emissions) == (0.75, 4)
    check(resize_stream, None, 1, expected=1)
    check(gaussian_release, c.byref(gaussian))
    check(stream_builder_release, c.byref(stream_builder))
    check(has_emission, saved, 0, c.byref(present))
    assert present.value == 1
    check(stream_builder_release, c.byref(saved))

    new_model_builder = bind('model_builder_new' + suffix, [size, size, c.POINTER(owner)])
    resize_model = bind('model_builder_resize' + suffix, [owner, size, size])
    model_shape = bind('model_builder_layout' + suffix, [owner, c.POINTER(ModelLayout)])
    stream_new = bind('stream_new' + suffix, [size, size, size, c.POINTER(owner)])
    duration_new = bind('duration_create' + suffix, [c.POINTER(DurationParameters), c.POINTER(owner)])
    set_stream = bind('model_builder_set_stream' + suffix, [owner, size, owner])
    set_duration = bind('model_builder_set_duration' + suffix, [owner, size, owner])
    has_stream = bind('model_builder_has_stream' + suffix, [owner, size, c.POINTER(status)])
    has_duration = bind('model_builder_has_duration' + suffix, [owner, size, c.POINTER(status)])
    clone_model = bind('model_builder_clone' + suffix, [owner, c.POINTER(owner)])
    model_release = bind('model_builder_release' + suffix, [c.POINTER(owner)])
    stream_release = bind('stream_release' + suffix, [c.POINTER(owner)])
    duration_release = bind('duration_release' + suffix, [c.POINTER(owner)])
    model, stream, duration = owner(), owner(), owner()
    check(new_model_builder, 2, 2, c.byref(model))
    check(stream_new, 1, 1, 1, c.byref(stream))
    parameters = DurationParameters(3, 2, -1, -1, -1, 0.5)
    check(duration_new, c.byref(parameters), c.byref(duration))
    check(set_stream, model, 0, stream)
    check(set_duration, model, 0, duration)
    check(resize_model, model, 4, 3)
    check(set_stream, model, 3, stream)
    check(set_duration, model, 2, duration)
    for streams, durations in [(0x80000000, 1), (1, 0x80000000)]:
        check(resize_model, model, streams, durations, expected=3)
        layout = ModelLayout()
        check(model_shape, model, c.byref(layout))
        assert (layout.streams, layout.durations) == (4, 3)
        check(has_stream, model, 3, c.byref(present))
        assert present.value == 1
        check(has_duration, model, 2, c.byref(present))
        assert present.value == 1
    check(resize_model, model, 1, 1)
    check(resize_model, model, 4, 3)
    for function, index in [(has_stream, 3), (has_duration, 2)]:
        check(function, model, index, c.byref(present))
        assert present.value == 0
    saved = owner()
    check(clone_model, model, c.byref(saved))
    check(resize_model, model, 0, 0)
    check(model_release, c.byref(model))
    check(stream_release, c.byref(stream))
    check(duration_release, c.byref(duration))
    check(model_shape, saved, c.byref(layout))
    assert (layout.streams, layout.durations) == (4, 3)
    for function in [has_stream, has_duration]:
        check(function, saved, 0, c.byref(present))
        assert present.value == 1
    check(resize_model, None, 1, 1, expected=1)
    check(model_release, c.byref(saved))

print('Actual Python C ABI: four resize exports, both precisions, retained slots and atomic errors passed.')
