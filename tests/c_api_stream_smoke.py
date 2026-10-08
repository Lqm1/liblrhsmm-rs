"""Verify every stream and builder export through a real shared library."""
import ctypes as c
import sys

lib = c.CDLL(sys.argv[1])


class GaussianLayout(c.Structure):
    _fields_ = [("mixtures", c.c_size_t), ("dimensions", c.c_size_t)]


for suffix, scalar in [("f32", c.c_float), ("f64", c.c_double)]:
    class Layout(c.Structure):
        _fields_ = [("weight", scalar), ("emissions", c.c_size_t)]

    owner_output = c.POINTER(c.c_void_p)
    signatures = {
        "stream_new": [c.c_size_t, c.c_size_t, c.c_size_t, owner_output],
        "stream_create": [scalar, owner_output, c.c_size_t, owner_output],
        "stream_replace": [c.c_void_p, scalar, owner_output, c.c_size_t],
        "stream_layout": [c.c_void_p, c.POINTER(Layout)],
        "stream_set_weight": [c.c_void_p, scalar],
        "stream_get_emission": [c.c_void_p, c.c_size_t, owner_output],
        "stream_set_emission": [c.c_void_p, c.c_size_t, c.c_void_p],
        "stream_clone": [c.c_void_p, owner_output],
        "stream_release": [owner_output],
        "stream_builder_new": [c.c_size_t, owner_output],
        "stream_builder_layout": [c.c_void_p, c.POINTER(Layout)],
        "stream_builder_set_weight": [c.c_void_p, scalar],
        "stream_builder_has_emission": [c.c_void_p, c.c_size_t, c.POINTER(c.c_uint32)],
        "stream_builder_get_emission": [c.c_void_p, c.c_size_t, owner_output],
        "stream_builder_set_emission": [c.c_void_p, c.c_size_t, c.c_void_p],
        "stream_builder_clear_emission": [c.c_void_p, c.c_size_t],
        "stream_builder_clone": [c.c_void_p, owner_output],
        "stream_builder_build": [c.c_void_p, owner_output],
        "stream_builder_release": [owner_output],
        "gaussian_new": [c.c_size_t, c.c_size_t, owner_output],
        "gaussian_layout": [c.c_void_p, c.POINTER(GaussianLayout)],
        "gaussian_copy_field": [c.c_void_p, c.c_uint32, c.c_size_t, c.POINTER(scalar), c.c_size_t],
        "gaussian_set_field": [c.c_void_p, c.c_uint32, c.c_size_t, c.POINTER(scalar), c.c_size_t],
        "gaussian_release": [owner_output],
    }
    functions = {}
    called = set()
    for name, args in signatures.items():
        function = getattr(lib, f"liblrhsmm_rs_{name}_{suffix}")
        function.argtypes = args
        function.restype = c.c_uint32
        functions[name] = function

    def status(name, expected, *args):
        result = functions[name](*args)
        assert result == expected, (name, result, expected)
        called.add(name)

    def call(name, *args):
        status(name, 0, *args)

    def check(stream, weight, count, shape=None, expected_fields=None):
        layout = Layout()
        call("stream_layout", stream, c.byref(layout))
        assert bytes(scalar(layout.weight)) == bytes(scalar(weight))
        assert layout.emissions == count
        if count:
            child = c.c_void_p()
            call("stream_get_emission", stream, 0, c.byref(child))
            actual_shape = GaussianLayout()
            call("gaussian_layout", child, c.byref(actual_shape))
            assert (actual_shape.mixtures, actual_shape.dimensions) == shape
            if expected_fields is not None:
                for field, value in enumerate(expected_fields):
                    result = scalar()
                    call("gaussian_copy_field", child, field, 0, c.byref(result), 1)
                    assert bytes(result) == bytes(scalar(value))
            call("gaussian_release", c.byref(child))

    stream, copy, created, built, rebuilt, a, b, child, builder, saved = [c.c_void_p() for _ in range(10)]
    call("stream_new", 2, 1, 2, c.byref(stream))
    check(stream, 1, 2, (1, 2), [1, 0, 0, 0])
    call("gaussian_new", 1, 1, c.byref(a))
    call("gaussian_new", 2, 3, c.byref(b))
    fields = [0.125, -0.0, float("inf"), -2.5]
    for owner in (a, b):
        for field, value in enumerate(fields):
            entry = scalar(value)
            call("gaussian_set_field", owner, field, 0, c.byref(entry), 1)
    inputs = (c.c_void_p * 3)(a.value, b.value, a.value)
    call("stream_replace", stream, -0.0, inputs, 3)
    call("stream_create", -0.0, inputs, 3, c.byref(created))
    call("stream_clone", stream, c.byref(copy))
    call("stream_set_weight", stream, 0.5)
    call("stream_set_emission", stream, 0, b)
    check(stream, 0.5, 3, (2, 3), fields)
    call("stream_get_emission", stream, 0, c.byref(child))
    changed = scalar(99)
    call("gaussian_set_field", child, 1, 0, c.byref(changed), 1)
    call("gaussian_release", c.byref(child))
    check(stream, 0.5, 3, (2, 3), fields)
    status("stream_set_emission", 2, stream, 3, a)
    status("stream_get_emission", 2, stream, 3, c.byref(child))
    assert child.value is None
    invalid = (c.c_void_p * 2)(a.value, None)
    status("stream_replace", 1, stream, 9, invalid, 2)
    check(stream, 0.5, 3, (2, 3), fields)
    failure = c.c_void_p(stream.value)
    status("stream_create", 1, 1, None, 2, c.byref(failure))
    status("stream_new", 3, 1, 0, 1, c.byref(failure))
    assert failure.value == stream.value
    status("stream_clone", 1, stream, None)
    call("stream_builder_new", 2, c.byref(builder))
    layout = Layout()
    call("stream_builder_layout", builder, c.byref(layout))
    assert (layout.weight, layout.emissions) == (1, 2)
    present = c.c_uint32(99)
    call("stream_builder_has_emission", builder, 0, c.byref(present))
    assert present.value == 0
    child.value = a.value
    status("stream_builder_get_emission", 3, builder, 0, c.byref(child))
    assert child.value == a.value
    status("stream_builder_build", 3, builder, c.byref(failure))
    assert failure.value == stream.value
    call("stream_builder_set_weight", builder, -0.0)
    call("stream_builder_set_emission", builder, 0, a)
    call("stream_builder_set_emission", builder, 1, b)
    call("stream_builder_has_emission", builder, 0, c.byref(present))
    assert present.value == 1
    call("stream_builder_clone", builder, c.byref(saved))
    call("stream_builder_build", builder, c.byref(built))
    child.value = None
    call("stream_builder_get_emission", builder, 0, c.byref(child))
    call("gaussian_set_field", child, 1, 0, c.byref(changed), 1)
    call("gaussian_release", c.byref(child))
    call("stream_builder_clear_emission", builder, 0)
    status("stream_builder_build", 3, builder, c.byref(failure))
    assert failure.value == stream.value
    present.value = 99
    status("stream_builder_has_emission", 2, builder, 2, c.byref(present))
    assert present.value == 99
    status("stream_builder_set_emission", 2, builder, 2, a)
    status("stream_builder_clear_emission", 2, builder, 2)
    status("stream_builder_get_emission", 2, builder, 2, c.byref(child))
    assert child.value is None
    call("gaussian_release", c.byref(a))
    call("gaussian_release", c.byref(b))
    call("stream_builder_release", c.byref(builder))
    assert builder.value is None
    call("stream_builder_release", c.byref(builder))
    call("stream_builder_build", saved, c.byref(rebuilt))
    check(built, -0.0, 2, (1, 1), fields)
    check(rebuilt, -0.0, 2, (1, 1), fields)
    call("stream_builder_release", c.byref(saved))
    call("stream_release", c.byref(stream))
    check(copy, -0.0, 3, (1, 1), fields)
    check(created, -0.0, 3, (1, 1), fields)
    for owner in (copy, created, built, rebuilt):
        call("stream_release", c.byref(owner))
    call("stream_create", -0.0, None, 0, c.byref(stream))
    check(stream, -0.0, 0)
    call("stream_replace", stream, 1, None, 0)
    check(stream, 1, 0)
    call("stream_release", c.byref(stream))
    call("stream_new", 0, 0, 0, c.byref(stream))
    check(stream, 1, 0)
    call("stream_release", c.byref(stream))
    call("stream_builder_new", 0, c.byref(builder))
    call("stream_builder_build", builder, c.byref(stream))
    check(stream, 1, 0)
    call("stream_release", c.byref(stream))
    call("stream_builder_release", c.byref(builder))
    status("stream_release", 1, None)
    status("stream_builder_release", 1, None)
    assert called == set(signatures)

print("stream: all 38 exports passed")
