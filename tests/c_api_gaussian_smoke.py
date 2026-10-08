"""Exercise every Gaussian owner export through a real shared library."""
import ctypes as c
import sys
from pathlib import Path

lib = c.CDLL(sys.argv[1])
class Layout(c.Structure):
    _fields_ = [("mixtures", c.c_size_t), ("dimensions", c.c_size_t)]

for suffix, scalar, fixture in [("f32", c.c_float, "float"), ("f64", c.c_double, "double")]:
    pointer = c.POINTER(scalar)
    class Parameters(c.Structure):
        _fields_ = Layout._fields_ + [(name, pointer) for name in ["weights", "means", "variances", "variance_floors"]]
    class Duration(c.Structure):
        _fields_ = [("mean", scalar), ("variance", scalar), ("minimum", c.c_int32), ("maximum", c.c_int32), ("fixed_mean", c.c_int32), ("variance_floor", scalar)]
    signatures = {
        "new": [c.c_size_t, c.c_size_t, c.POINTER(c.c_void_p)],
        "create": [c.POINTER(Parameters), c.POINTER(c.c_void_p)],
        "replace": [c.c_void_p, c.POINTER(Parameters)],
        "layout": [c.c_void_p, c.POINTER(Layout)],
        "copy_field": [c.c_void_p, c.c_uint32, c.c_size_t, pointer, c.c_size_t],
        "set_field": [c.c_void_p, c.c_uint32, c.c_size_t, pointer, c.c_size_t],
        "clone": [c.c_void_p, c.POINTER(c.c_void_p)],
        "release": [c.POINTER(c.c_void_p)],
        "combine": [c.POINTER(c.c_void_p), c.c_size_t, c.POINTER(c.c_void_p)],
        "from_durations": [c.POINTER(c.c_void_p), c.c_size_t, c.POINTER(c.c_void_p)],
        "increase_mixtures": [c.c_void_p, c.c_size_t, scalar],
    }
    functions = {}
    for name, args in signatures.items():
        function = getattr(lib, f"liblrhsmm_rs_gaussian_{name}_{suffix}")
        function.argtypes = args
        function.restype = c.c_uint32
        functions[name] = function
    def call(name, *args):
        status = functions[name](*args)
        assert status == 0, (name, status)
    owners = [c.c_void_p() for _ in range(5)]
    owner, copy, created, combined, converted = owners
    call("new", 1, 2, c.byref(owner))
    layout = Layout()
    call("layout", owner, c.byref(layout))
    assert (layout.mixtures, layout.dimensions) == (1, 2)
    weights = (scalar * 1)(1)
    means = (scalar * 2)(1.25, -2.5)
    variances = (scalar * 2)(2.5, 0.75)
    floors = (scalar * 2)(0.1, 0.2)
    parameters = Parameters(1, 2, weights, means, variances, floors)
    call("replace", owner, c.byref(parameters))
    call("create", c.byref(parameters), c.byref(created))
    call("clone", owner, c.byref(copy))
    call("increase_mixtures", owner, 5, 0.2)
    fields = []
    for selector in range(4):
        count = 5 if selector == 0 else 10
        values = (scalar * count)()
        call("copy_field", owner, selector, 0, values, count)
        fields.append(list(values))
        call("copy_field", owner, selector, count, None, 0)
    compared = 0
    for line in Path(f"tests/fixtures/c-precision-{fixture}.txt").read_text().splitlines():
        tokens = line.split()
        if tokens[0] != "mixture":
            continue
        index = int(tokens[1])
        actual = [fields[0][index]]
        for dimension in range(2):
            actual.extend(fields[field][2 * index + dimension] for field in range(1, 4))
        for value, text in zip(actual, tokens[3:]):
            target = scalar(float(text)).value
            assert abs(value - target) <= 2e-14 * max(1.0, abs(target))
            compared += 1
    assert compared == 35
    changed = scalar(-9)
    call("set_field", copy, 1, 0, c.byref(changed), 1)
    result = scalar()
    call("copy_field", created, 1, 0, c.byref(result), 1)
    assert result.value == 1.25
    assert functions["set_field"](copy, 1, 99, c.byref(changed), 1) == 2
    call("copy_field", copy, 1, 0, c.byref(result), 1)
    assert result.value == -9
    handles = (c.c_void_p * 2)(created.value, created.value)
    call("combine", handles, 2, c.byref(combined))
    call("layout", combined, c.byref(layout))
    assert (layout.mixtures, layout.dimensions) == (1, 4)
    duration = c.c_void_p()
    duration_create = getattr(lib, f"liblrhsmm_rs_duration_create_{suffix}")
    duration_create.argtypes = [c.POINTER(Duration), c.POINTER(c.c_void_p)]
    duration_create.restype = c.c_uint32
    duration_release = getattr(lib, f"liblrhsmm_rs_duration_release_{suffix}")
    duration_release.argtypes = [c.POINTER(c.c_void_p)]
    duration_release.restype = c.c_uint32
    assert duration_create(c.byref(Duration(2, 3, -1, -1, -1, 0.4)), c.byref(duration)) == 0
    call("from_durations", (c.c_void_p * 1)(duration.value), 1, c.byref(converted))
    call("copy_field", converted, 3, 0, c.byref(result), 1)
    assert result.value == 0
    assert duration_release(c.byref(duration)) == 0
    for owned in owners:
        call("release", c.byref(owned))
        assert owned.value is None
    call("release", c.byref(owner))
print("Python Gaussian: all22 exports and70 original C parameter values passed")
