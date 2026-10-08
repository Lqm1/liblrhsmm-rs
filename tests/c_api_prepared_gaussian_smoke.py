"""Verify all prepared Gaussian exports through a real shared library."""
import ctypes as c
import sys
from pathlib import Path
lib = c.CDLL(sys.argv[1])
for suffix, scalar, fixture in [("f32", c.c_float, "float"), ("f64", c.c_double, "double")]:
    pointer = c.POINTER(scalar)
    slot = c.POINTER(c.c_void_p)
    class Parameters(c.Structure):
        _fields_ = [("mixtures", c.c_size_t), ("dimensions", c.c_size_t)] + [(name, pointer) for name in ["weights", "means", "variances", "variance_floors"]]
    def bind(prefix, name, args):
        function = getattr(lib, f"liblrhsmm_rs_{prefix}_{name}_{suffix}")
        function.argtypes = args
        function.restype = c.c_uint32
        return function
    gaussian = {
        "create": bind("gaussian", "create", [c.POINTER(Parameters), slot]),
        "increase": bind("gaussian", "increase_mixtures", [c.c_void_p, c.c_size_t, scalar]),
        "prepare": bind("gaussian", "prepare", [c.c_void_p, slot]),
        "edit": bind("gaussian", "set_field", [c.c_void_p, c.c_uint32, c.c_size_t, pointer, c.c_size_t]),
        "release": bind("gaussian", "release", [slot]),
    }
    functions = {}
    for name, args in {
        "parameters": [c.c_void_p, slot], "clone": [c.c_void_p, slot], "release": [slot],
        "cache_length": [c.c_void_p, c.POINTER(c.c_size_t)],
        "cache_copy": [c.c_void_p, c.c_size_t, pointer, c.c_size_t],
        "component_log_probability": [c.c_void_p, pointer, c.c_size_t, c.c_size_t, pointer],
        "component_probability": [c.c_void_p, pointer, c.c_size_t, c.c_size_t, c.POINTER(c.c_double)],
        "log_probability": [c.c_void_p, pointer, c.c_size_t, scalar, pointer],
        "probability": [c.c_void_p, pointer, c.c_size_t, scalar, c.POINTER(c.c_double)],
    }.items():
        functions[name] = bind("prepared_gaussian", name, args)
    def call(name, *args):
        status = functions[name](*args)
        assert status == 0, (name, status)
    source, editable, prepared, cloned = [c.c_void_p() for _ in range(4)]
    arrays = [(scalar * len(values))(*values) for values in [[1], [1.25, -2.5], [2.5, 0.75], [0.1, 0.2]]]
    assert gaussian["create"](c.byref(Parameters(1, 2, *arrays)), c.byref(source)) == 0
    assert gaussian["increase"](source, 5, 0.2) == 0
    assert gaussian["prepare"](source, c.byref(prepared)) == 0
    call("parameters", prepared, c.byref(editable))
    call("clone", prepared, c.byref(cloned))
    changed = scalar(999)
    for owner in [source, editable]:
        assert gaussian["edit"](owner, 1, 0, c.byref(changed), 1) == 0
        assert gaussian["release"](c.byref(owner)) == 0
    count = c.c_size_t()
    call("cache_length", prepared, c.byref(count))
    assert count.value == 5
    cache = (scalar * 5)()
    call("cache_copy", prepared, 0, cache, 5)
    call("cache_copy", prepared, 5, None, 0)
    compared = 0
    for line in Path(f"tests/fixtures/c-precision-{fixture}.txt").read_text().splitlines():
        tokens = line.split()
        if tokens[0] not in ["component", "output"]:
            continue
        index, secondary = map(int, tokens[1:3])
        observation = (scalar * 2)(index * 0.3 - 0.5, index * -0.2 + 0.75)
        result, probability = scalar(77), c.c_double(77)
        if tokens[0] == "component":
            call("component_log_probability", prepared, observation, 2, secondary, c.byref(result))
            call("component_probability", prepared, observation, 2, secondary, c.byref(probability))
        else:
            temperature = [0.5, 1.0, 1.75][secondary]
            call("log_probability", prepared, observation, 2, temperature, c.byref(result))
            call("probability", prepared, observation, 2, temperature, c.byref(probability))
        for value, target in [(result.value, scalar(float(tokens[3])).value), (probability.value, float(tokens[4]))]:
            assert abs(value - target) <= 2e-14 * max(1.0, abs(target))
            compared += 1
    assert compared == 80
    result, other = scalar(77), scalar(77)
    observation = (scalar * 2)(0, 0)
    call("log_probability", prepared, observation, 2, 1, c.byref(result))
    call("log_probability", cloned, observation, 2, 1, c.byref(other))
    assert result.value == other.value
    result = scalar(77)
    assert functions["component_log_probability"](prepared, observation, 2, 5, c.byref(result)) == 3
    assert result.value == 77
    assert functions["cache_copy"](prepared, 5, c.byref(result), 1) == 2
    assert result.value == 77
    for owner in [prepared, cloned]:
        call("release", c.byref(owner))
        assert owner.value is None
    call("release", c.byref(prepared))
    arrays = [(scalar * len(values))(*values) for values in [[1 / 3] * 3, [0] * 6, [0.75, 1.25, 2.5, 3.75, 0.125, 0.5], [0] * 6]]
    assert gaussian["create"](c.byref(Parameters(3, 2, *arrays)), c.byref(source)) == 0
    assert gaussian["prepare"](source, c.byref(prepared)) == 0
    call("cache_copy", prepared, 0, cache, 3)
    compared = 0
    for line in Path(f"tests/fixtures/c-structures-{fixture}.txt").read_text().splitlines():
        tokens = line.split()
        if tokens[0] == "gaussian":
            target = scalar(float(tokens[2])).value
            assert abs(cache[int(tokens[1])] - target) <= 2e-14 * max(1.0, abs(target))
            compared += 1
    assert compared == 3
    assert gaussian["release"](c.byref(source)) == 0
    call("release", c.byref(prepared))
print("Python prepared Gaussian: all20 exports and166 original C cache/density values passed")
