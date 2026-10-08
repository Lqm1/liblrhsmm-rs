"""Actual ctypes ownership and original-source duration comparisons."""
import ctypes as c
from pathlib import Path
import sys

library = c.CDLL(sys.argv[1])
for suffix, scalar, label in [("f32", c.c_float, "float"), ("f64", c.c_double, "double")]:
    class Parameters(c.Structure):
        _fields_ = [("mean", scalar), ("variance", scalar), ("minimum", c.c_int32),
                    ("maximum", c.c_int32), ("fixed_mean", c.c_int32), ("variance_floor", scalar)]

    def function(name, arguments):
        result = getattr(library, f"liblrhsmm_rs_duration_{name}_{suffix}")
        result.argtypes, result.restype = arguments, c.c_uint32
        return result

    default = function("default", [c.POINTER(Parameters)])
    create = function("create", [c.POINTER(Parameters), c.POINTER(c.c_void_p)])
    get = function("get", [c.c_void_p, c.POINTER(Parameters)])
    set_parameters = function("set", [c.c_void_p, c.POINTER(Parameters)])
    clone = function("clone", [c.c_void_p, c.POINTER(c.c_void_p)])
    release = function("release", [c.POINTER(c.c_void_p)])
    log_probability = function("log_probability", [c.c_void_p, scalar, scalar, c.POINTER(scalar)])
    annealed = function("annealed_log_probability", [c.c_void_p, c.c_size_t, scalar, scalar, c.POINTER(scalar)])
    probability = function("probability", [c.c_void_p, c.c_size_t, scalar, scalar, c.POINTER(c.c_double)])
    parameters = Parameters()
    assert default(c.byref(parameters)) == 0
    assert (parameters.mean, parameters.variance, parameters.minimum, parameters.maximum,
            parameters.fixed_mean, parameters.variance_floor) == (0, 0, -1, -1, -1, 0)
    owner, copy = c.c_void_p(), c.c_void_p()
    assert create(c.byref(parameters), c.byref(owner)) == 0
    density = c.c_double(71)
    assert probability(owner, 3, 1, 1, c.byref(density)) == 3 and density.value == 71
    parameters = Parameters(4.25, 2.5, 2, 6, 17, .125)
    assert set_parameters(owner, c.byref(parameters)) == 0
    compared = 0
    for line in (Path(__file__).parent / "fixtures" / f"c-precision-{label}.txt").read_text().splitlines():
        fields = line.split()
        if fields[0] != "duration":
            continue
        frames, index = map(int, fields[1:3])
        temperature = [.5, 1, 1.75][index]
        log, annealed_log = scalar(71), scalar(71)
        assert log_probability(owner, frames, .75, c.byref(log)) == 0
        assert annealed(owner, frames, .75, temperature, c.byref(annealed_log)) == 0
        assert probability(owner, frames, .75, temperature, c.byref(density)) == 0
        for actual, expected in zip([log.value, annealed_log.value, density.value], map(float, fields[3:])):
            assert abs(actual - expected) / max(abs(expected), 1) <= 2e-14, line
            compared += 1
    assert compared == 81
    assert clone(owner, c.byref(copy)) == 0
    parameters.mean = 8
    assert set_parameters(copy, c.byref(parameters)) == 0
    assert get(owner, c.byref(parameters)) == 0
    assert (parameters.mean, parameters.fixed_mean, parameters.variance_floor) == (4.25, 17, .125)
    assert set_parameters(owner, None) == 1 and create(c.byref(parameters), None) == 1
    assert clone(owner, None) == 1 and get(owner, None) == 1 and default(None) == 1
    output = scalar(71)
    assert annealed(owner, c.c_size_t(-1).value, .75, 1, c.byref(output)) == 3 and output.value == 71
    assert log_probability(owner, float("nan"), .75, c.byref(output)) == 3 and output.value == 71
    assert annealed(owner, 1, .75, 0, c.byref(output)) == 0 and output.value == scalar(-1e15).value
    assert annealed(owner, 3, .75, 0, c.byref(output)) == 0 and output.value == 0
    assert probability(owner, 8, .75, 1, c.byref(density)) == 0 and density.value > 0
    assert release(c.byref(owner)) == 0 and owner.value is None
    assert release(c.byref(owner)) == 0
    assert get(copy, c.byref(parameters)) == 0 and parameters.mean == 8
    assert release(c.byref(copy)) == 0 and copy.value is None
    assert release(None) == 1
print("Python duration: all18 exports and162 original C values passed")
