"""Verify every model and builder export through an actual shared library."""
import ctypes as c
import sys

lib = c.CDLL(sys.argv[1])


class Specification(c.Structure):
    _fields_ = [(name, c.c_size_t) for name in ("emissions", "components", "dimensions")]


class ModelLayout(c.Structure):
    _fields_ = [("streams", c.c_size_t), ("durations", c.c_size_t)]


for suffix, scalar in [("f32", c.c_float), ("f64", c.c_double)]:
    class StreamLayout(c.Structure):
        _fields_ = [("weight", scalar), ("emissions", c.c_size_t)]

    class Duration(c.Structure):
        _fields_ = [("mean", scalar), ("variance", scalar), ("minimum", c.c_int32),
                    ("maximum", c.c_int32), ("fixed_mean", c.c_int32), ("variance_floor", scalar)]

    output = c.POINTER(c.c_void_p)
    signatures = {
        "model_new": [c.POINTER(Specification), c.c_size_t, c.c_size_t, output],
        "model_create": [output, c.c_size_t, output, c.c_size_t, output],
        "model_replace": [c.c_void_p, output, c.c_size_t, output, c.c_size_t],
        "model_layout": [c.c_void_p, c.POINTER(ModelLayout)],
        "model_get_stream": [c.c_void_p, c.c_size_t, output],
        "model_set_stream": [c.c_void_p, c.c_size_t, c.c_void_p],
        "model_get_duration": [c.c_void_p, c.c_size_t, output],
        "model_set_duration": [c.c_void_p, c.c_size_t, c.c_void_p],
        "model_clone": [c.c_void_p, output],
        "model_release": [output],
        "model_builder_new": [c.c_size_t, c.c_size_t, output],
        "model_builder_layout": [c.c_void_p, c.POINTER(ModelLayout)],
        "model_builder_has_stream": [c.c_void_p, c.c_size_t, c.POINTER(c.c_uint32)],
        "model_builder_get_stream": [c.c_void_p, c.c_size_t, output],
        "model_builder_set_stream": [c.c_void_p, c.c_size_t, c.c_void_p],
        "model_builder_clear_stream": [c.c_void_p, c.c_size_t],
        "model_builder_has_duration": [c.c_void_p, c.c_size_t, c.POINTER(c.c_uint32)],
        "model_builder_get_duration": [c.c_void_p, c.c_size_t, output],
        "model_builder_set_duration": [c.c_void_p, c.c_size_t, c.c_void_p],
        "model_builder_clear_duration": [c.c_void_p, c.c_size_t],
        "model_builder_clone": [c.c_void_p, output],
        "model_builder_build": [c.c_void_p, output],
        "model_builder_release": [output],
        "stream_new": [c.c_size_t, c.c_size_t, c.c_size_t, output],
        "stream_layout": [c.c_void_p, c.POINTER(StreamLayout)],
        "stream_set_weight": [c.c_void_p, scalar],
        "stream_release": [output],
        "duration_create": [c.POINTER(Duration), output],
        "duration_get": [c.c_void_p, c.POINTER(Duration)],
        "duration_set": [c.c_void_p, c.POINTER(Duration)],
        "duration_release": [output],
    }
    functions = {}
    called = set()
    for name, args in signatures.items():
        function = getattr(lib, f"liblrhsmm_rs_{name}_{suffix}")
        function.argtypes = args
        function.restype = c.c_uint32
        functions[name] = function

    def status(name, expected, *args):
        actual = functions[name](*args)
        assert actual == expected, (name, actual, expected)
        called.add(name)

    def call(name, *args):
        status(name, 0, *args)

    def check(model, streams, durations, weight=0, emissions=0, expected=None):
        layout = ModelLayout()
        call("model_layout", model, c.byref(layout))
        assert (layout.streams, layout.durations) == (streams, durations)
        if streams:
            stream = c.c_void_p()
            shape = StreamLayout()
            call("model_get_stream", model, 0, c.byref(stream))
            call("stream_layout", stream, c.byref(shape))
            assert bytes(scalar(shape.weight)) == bytes(scalar(weight))
            assert shape.emissions == emissions
            call("stream_release", c.byref(stream))
        if durations and expected is not None:
            duration = c.c_void_p()
            actual = Duration()
            call("model_get_duration", model, 0, c.byref(duration))
            call("duration_get", duration, c.byref(actual))
            for field in ("mean", "variance", "variance_floor"):
                assert bytes(scalar(getattr(actual, field))) == bytes(scalar(getattr(expected, field)))
            for field in ("minimum", "maximum", "fixed_mean"):
                assert getattr(actual, field) == getattr(expected, field)
            call("duration_release", c.byref(duration))

    model, copy, created, built, rebuilt, builder, saved, a, b, duration, returned_stream, returned_duration = [c.c_void_p() for _ in range(12)]
    layouts = (Specification * 2)(Specification(2, 3, 2), Specification(1, 2, 1))
    flat = Duration(0, 0, -1, -1, -1, 0)
    call("model_new", layouts, 2, 3, c.byref(model))
    check(model, 2, 3, 1, 2, flat)
    failure = c.c_void_p(model.value)
    status("model_new", 1, None, 2, 3, c.byref(failure))
    invalid_layout = Specification(1, 0, 1)
    status("model_new", 3, c.byref(invalid_layout), 1, 0, c.byref(failure))
    assert failure.value == model.value
    status("model_clone", 1, model, None)
    call("stream_new", 2, 1, 2, c.byref(a))
    call("stream_new", 1, 2, 3, c.byref(b))
    call("stream_set_weight", a, -0.0)
    call("stream_set_weight", b, 0.5)
    fields = Duration(-0.0, float("inf"), -(2**31), 2**31-1, 7, -2.5)
    revised = Duration(5, 2, 1, 9, -1, 0.25)
    call("duration_create", c.byref(fields), c.byref(duration))
    streams = (c.c_void_p * 3)(a.value, b.value, a.value)
    durations = (c.c_void_p * 2)(duration.value, duration.value)
    call("model_replace", model, streams, 3, durations, 2)
    call("model_create", streams, 3, durations, 2, c.byref(created))
    call("model_clone", model, c.byref(copy))
    call("model_set_stream", model, 0, b)
    call("duration_set", duration, c.byref(revised))
    call("model_set_duration", model, 0, duration)
    check(model, 3, 2, 0.5, 1, revised)
    check(copy, 3, 2, -0.0, 2, fields)
    check(created, 3, 2, -0.0, 2, fields)
    call("model_get_stream", model, 0, c.byref(returned_stream))
    call("model_get_duration", model, 0, c.byref(returned_duration))
    call("stream_set_weight", returned_stream, 99)
    call("duration_set", returned_duration, c.byref(fields))
    call("stream_release", c.byref(returned_stream))
    call("duration_release", c.byref(returned_duration))
    check(model, 3, 2, 0.5, 1, revised)
    bad_streams = (c.c_void_p * 2)(a.value, None)
    bad_durations = (c.c_void_p * 2)(duration.value, None)
    status("model_replace", 1, model, bad_streams, 2, durations, 2)
    status("model_replace", 1, model, streams, 3, bad_durations, 2)
    check(model, 3, 2, 0.5, 1, revised)
    status("model_create", 1, None, 1, durations, 2, c.byref(failure))
    assert failure.value == model.value
    status("model_get_stream", 2, model, 3, c.byref(returned_stream))
    status("model_get_duration", 2, model, 2, c.byref(returned_duration))
    assert returned_stream.value is None and returned_duration.value is None
    status("model_set_stream", 2, model, 3, a)
    status("model_set_duration", 2, model, 2, duration)
    status("model_set_stream", 1, model, 0, None)
    status("model_set_duration", 1, model, 0, None)
    call("model_builder_new", 2, 2, c.byref(builder))
    layout = ModelLayout()
    call("model_builder_layout", builder, c.byref(layout))
    assert (layout.streams, layout.durations) == (2, 2)
    present = c.c_uint32(99)
    for name in ("stream", "duration"):
        call(f"model_builder_has_{name}", builder, 0, c.byref(present))
        assert present.value == 0
    returned_stream.value = a.value
    returned_duration.value = duration.value
    status("model_builder_get_stream", 3, builder, 0, c.byref(returned_stream))
    status("model_builder_get_duration", 3, builder, 0, c.byref(returned_duration))
    assert returned_stream.value == a.value and returned_duration.value == duration.value
    status("model_builder_build", 3, builder, c.byref(failure))
    call("model_builder_set_stream", builder, 0, a)
    call("model_builder_set_stream", builder, 1, b)
    status("model_builder_build", 3, builder, c.byref(failure))
    assert failure.value == model.value
    call("duration_set", duration, c.byref(fields))
    for index in range(2):
        call("model_builder_set_duration", builder, index, duration)
    for name in ("stream", "duration"):
        call(f"model_builder_has_{name}", builder, 0, c.byref(present))
        assert present.value == 1
    call("model_builder_clone", builder, c.byref(saved))
    call("model_builder_build", builder, c.byref(built))
    returned_stream.value = None
    returned_duration.value = None
    call("model_builder_get_stream", builder, 0, c.byref(returned_stream))
    call("model_builder_get_duration", builder, 0, c.byref(returned_duration))
    call("stream_set_weight", returned_stream, 99)
    call("duration_set", returned_duration, c.byref(revised))
    call("stream_release", c.byref(returned_stream))
    call("duration_release", c.byref(returned_duration))
    call("model_builder_clear_stream", builder, 0)
    status("model_builder_build", 3, builder, c.byref(failure))
    call("model_builder_set_stream", builder, 0, b)
    call("model_builder_clear_duration", builder, 1)
    status("model_builder_build", 3, builder, c.byref(failure))
    assert failure.value == model.value
    present.value = 99
    for name, child, returned in [("stream", a, returned_stream), ("duration", duration, returned_duration)]:
        status(f"model_builder_has_{name}", 2, builder, 2, c.byref(present))
        assert present.value == 99
        status(f"model_builder_get_{name}", 2, builder, 2, c.byref(returned))
        assert returned.value is None
        status(f"model_builder_set_{name}", 2, builder, 2, child)
        status(f"model_builder_clear_{name}", 2, builder, 2)
    call("stream_release", c.byref(a))
    call("stream_release", c.byref(b))
    call("duration_release", c.byref(duration))
    call("model_builder_release", c.byref(builder))
    assert builder.value is None
    call("model_builder_release", c.byref(builder))
    call("model_builder_build", saved, c.byref(rebuilt))
    check(built, 2, 2, -0.0, 2, fields)
    check(rebuilt, 2, 2, -0.0, 2, fields)
    call("model_builder_release", c.byref(saved))
    check(model, 3, 2, 0.5, 1, revised)
    call("model_release", c.byref(model))
    check(copy, 3, 2, -0.0, 2, fields)
    for owner in (copy, created, built, rebuilt):
        call("model_release", c.byref(owner))
    call("model_create", None, 0, None, 0, c.byref(model))
    check(model, 0, 0)
    call("model_replace", model, None, 0, None, 0)
    call("model_release", c.byref(model))
    call("model_new", None, 0, 1, c.byref(model))
    check(model, 0, 1, expected=flat)
    call("model_release", c.byref(model))
    call("model_new", layouts, 1, 0, c.byref(model))
    check(model, 1, 0, 1, 2)
    call("model_release", c.byref(model))
    call("model_builder_new", 0, 0, c.byref(builder))
    call("model_builder_build", builder, c.byref(model))
    check(model, 0, 0)
    call("model_release", c.byref(model))
    call("model_builder_release", c.byref(builder))
    status("model_release", 1, None)
    status("model_builder_release", 1, None)
    assert called == set(signatures)

print("model: all 46 exports passed")
