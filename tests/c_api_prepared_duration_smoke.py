"""Actual ctypes checks for both immutable prepared-duration snapshots."""
import ctypes as c
from pathlib import Path
import sys

library = c.CDLL(sys.argv[1])
fixtures = Path(__file__).parent / "fixtures"
for suffix, scalar, label in [("f32", c.c_float, "float"), ("f64", c.c_double, "double")]:
    class Parameters(c.Structure):
        _fields_ = [("mean", scalar), ("variance", scalar), ("minimum", c.c_int32),
                    ("maximum", c.c_int32), ("fixed_mean", c.c_int32), ("variance_floor", scalar)]
    class Options(c.Structure):
        _fields_ = [("duration_cache_length", c.c_size_t), ("duration_weight", scalar), ("temperature", scalar)]
    def function(name, arguments):
        result = getattr(library, f"liblrhsmm_rs_{name}_{suffix}")
        result.argtypes, result.restype = arguments, c.c_uint32
        return result
    def compare(actual, expected):
        assert abs(actual-expected)/max(abs(expected), 1) <= 2e-14
    default = function("duration_preparation_default", [c.POINTER(Options)])
    create = function("duration_create", [c.POINTER(Parameters), c.POINTER(c.c_void_p)])
    set_parameters = function("duration_set", [c.c_void_p, c.POINTER(Parameters)])
    release_source = function("duration_release", [c.POINTER(c.c_void_p)])
    prepare = function("duration_prepare", [c.c_void_p, c.POINTER(Options), c.POINTER(c.c_void_p)])
    clone = function("prepared_duration_clone", [c.c_void_p, c.POINTER(c.c_void_p)])
    release = function("prepared_duration_release", [c.POINTER(c.c_void_p)])
    get_parameters = function("prepared_duration_parameters", [c.c_void_p, c.POINTER(Parameters)])
    get_options = function("prepared_duration_options", [c.c_void_p, c.POINTER(Options)])
    normalizer = function("prepared_duration_log_normalizer", [c.c_void_p, c.POINTER(scalar)])
    cache_length = function("prepared_duration_cache_length", [c.c_void_p, c.POINTER(c.c_size_t)])
    copy_cache = function("prepared_duration_cache_copy", [c.c_void_p, c.c_size_t, c.POINTER(scalar), c.c_size_t])
    log_probability = function("prepared_duration_log_probability", [c.c_void_p, c.c_size_t, c.POINTER(scalar)])
    probability = function("prepared_duration_probability", [c.c_void_p, c.c_size_t, c.POINTER(c.c_double)])
    options = Options()
    assert default(c.byref(options)) == 0
    assert (options.duration_cache_length, options.duration_weight, options.temperature) == (300, 1, 1)
    parameters = Parameters(4.25, 2.5, 2, 6, 17, .125)
    source, prepared, cloned = c.c_void_p(), c.c_void_p(), c.c_void_p()
    assert create(c.byref(parameters), c.byref(source)) == 0
    options = Options(8, .5, .75)
    assert prepare(source, c.byref(options), c.byref(prepared)) == 0
    fetched, fetched_options = Parameters(), Options()
    assert get_parameters(prepared, c.byref(fetched)) == 0
    assert (fetched.mean, fetched.variance, fetched.minimum, fetched.maximum, fetched.fixed_mean, fetched.variance_floor) == (4.25, 2.5, 2, 6, 17, .125)
    assert get_options(prepared, c.byref(fetched_options)) == 0
    assert (fetched_options.duration_cache_length, fetched_options.duration_weight, fetched_options.temperature) == (8, .5, .75)
    length = c.c_size_t(99)
    assert cache_length(prepared, c.byref(length)) == 0 and length.value == 8
    cache, constant = (scalar*8)(), scalar(71)
    assert copy_cache(prepared, 0, cache, 8) == 0
    assert normalizer(prepared, c.byref(constant)) == 0
    compared = 0
    for line in (fixtures/f"c-structures-{label}.txt").read_text().splitlines():
        fields = line.split()
        if fields[0] not in ["duration", "cache"]:
            continue
        actual = constant.value if fields[0] == "duration" else cache[int(fields[1])]
        compare(actual, float(fields[-1]))
        compared += 1
    assert compared == 9
    assert clone(prepared, c.byref(cloned)) == 0
    parameters.mean, parameters.minimum = 99, 99
    assert set_parameters(source, c.byref(parameters)) == 0
    assert release_source(c.byref(source)) == 0 and source.value is None
    assert release(c.byref(prepared)) == 0 and prepared.value is None
    assert release(c.byref(prepared)) == 0
    assert get_parameters(cloned, c.byref(fetched)) == 0 and fetched.mean == 4.25 and fetched.minimum == 2
    copied_cache = (scalar*8)()
    assert copy_cache(cloned, 0, copied_cache, 8) == 0 and list(copied_cache) == list(cache)
    log, density = scalar(71), c.c_double(71)
    assert log_probability(cloned, 4, c.byref(log)) == 0 and log.value == cache[4]
    assert probability(cloned, 4, c.byref(density)) == 0 and density.value > 0
    assert copy_cache(cloned, 8, None, 0) == 0 and copy_cache(cloned, 9, None, 0) == 2
    copied_cache[0] = 71
    assert copy_cache(cloned, 7, copied_cache, 2) == 2 and copied_cache[0] == 71
    assert copy_cache(cloned, c.c_size_t(-1).value, copied_cache, 1) == 2
    assert copy_cache(cloned, 0, None, 1) == 1
    log.value, density.value = 71, 71
    assert log_probability(cloned, c.c_size_t(-1).value, c.byref(log)) == 3 and log.value == 71
    assert probability(cloned, c.c_size_t(-1).value, c.byref(density)) == 3 and density.value == 71
    for function_value in [clone, get_parameters, get_options, normalizer, cache_length]:
        assert function_value(cloned, None) == 1
    assert release(c.byref(cloned)) == 0 and cloned.value is None
    assert release(None) == 1 and default(None) == 1
    parameters.mean, parameters.minimum = 4.25, 2
    assert create(c.byref(parameters), c.byref(source)) == 0
    compared = 0
    for line in (fixtures/f"c-precision-{label}.txt").read_text().splitlines():
        fields = line.split()
        if fields[0] != "duration":
            continue
        frames, index = map(int, fields[1:3])
        options = Options(3, .75, [.5, 1, 1.75][index])
        assert prepare(source, c.byref(options), c.byref(prepared)) == 0
        assert log_probability(prepared, frames, c.byref(log)) == 0
        assert probability(prepared, frames, c.byref(density)) == 0
        compare(log.value, float(fields[4])); compare(density.value, float(fields[5]))
        compared += 2
        assert release(c.byref(prepared)) == 0
    assert compared == 54
    if suffix == "f32":
        compared = 0
        for line in (fixtures/"c-preparation.txt").read_text().splitlines():
            fields = line.split()
            assert fields[0] == "D"
            length_value, temperature_index, frames = map(int, fields[1:4])
            options = Options(length_value, .75, temperature_index*.5)
            assert prepare(source, c.byref(options), c.byref(prepared)) == 0
            assert log_probability(prepared, frames, c.byref(log)) == 0
            assert probability(prepared, frames, c.byref(density)) == 0
            assert log.value == c.c_float(float(fields[4])).value
            assert density.value == c.c_float(float(fields[5])).value
            compared += 2
            assert release(c.byref(prepared)) == 0
        assert compared == 144
    options = Options(0, .75, 0)
    assert prepare(source, c.byref(options), c.byref(prepared)) == 0
    assert cache_length(prepared, c.byref(length)) == 0 and length.value == 0
    assert copy_cache(prepared, 0, None, 0) == 0
    assert log_probability(prepared, 1, c.byref(log)) == 0 and log.value == scalar(-1e15).value
    assert log_probability(prepared, 3, c.byref(log)) == 0 and log.value == 0
    assert release(c.byref(prepared)) == 0
    options.duration_cache_length = c.c_size_t(-1).value
    assert prepare(source, c.byref(options), c.byref(prepared)) == 3 and prepared.value is None
    assert prepare(source, None, c.byref(prepared)) == 1
    assert prepare(source, c.byref(options), None) == 1
    assert prepare(None, c.byref(options), c.byref(prepared)) == 1
    assert release_source(c.byref(source)) == 0
print("Python prepared duration: all22 exports and270 original C values passed")
