"""Call every Gaussian statistics/update export through a real shared library."""
import ctypes as c
import sys
from pathlib import Path
lib = c.CDLL(sys.argv[1])
for suffix, scalar, fixture in [("f32", c.c_float, "c-updates.txt"), ("f64", c.c_double, "c-double-update.txt")]:
    pointer, slot = c.POINTER(scalar), c.POINTER(c.c_void_p)
    class Parameters(c.Structure):
        _fields_ = [("mixtures", c.c_size_t), ("dimensions", c.c_size_t)] + [(name, pointer) for name in ["weights", "means", "variances", "variance_floors"]]
    class Values(c.Structure):
        _fields_ = [("mixtures", c.c_size_t), ("dimensions", c.c_size_t), ("state_occupancy", scalar)] + [(name, pointer) for name in ["weighted_sums", "weighted_squared_sums", "component_occupancies"]]
    class Layout(c.Structure):
        _fields_ = Values._fields_[:3]
    class Options(c.Structure):
        _fields_ = [("duration_mode", c.c_uint32), ("duration_variance_floor", scalar), ("emission_variance_floor", scalar)]
    def bind(prefix, name, args):
        function = getattr(lib, f"liblrhsmm_rs_{prefix}_{name}_{suffix}")
        function.argtypes, function.restype = args, c.c_uint32
        return function
    functions = {}
    for name, args in {
        "new": [c.c_size_t, c.c_size_t, slot], "create": [c.POINTER(Values), slot],
        "replace": [c.c_void_p, c.POINTER(Values)], "from_gaussian": [c.c_void_p, slot],
        "layout": [c.c_void_p, c.POINTER(Layout)], "set_occupancy": [c.c_void_p, scalar],
        "copy_field": [c.c_void_p, c.c_uint32, c.c_size_t, pointer, c.c_size_t],
        "set_field": [c.c_void_p, c.c_uint32, c.c_size_t, pointer, c.c_size_t],
        "clone": [c.c_void_p, slot], "clear": [c.c_void_p], "release": [slot],
    }.items():
        functions[name] = bind("gaussian_statistics", name, args)
    gaussian_new = bind("gaussian", "new", [c.c_size_t, c.c_size_t, slot])
    gaussian_create = bind("gaussian", "create", [c.POINTER(Parameters), slot])
    gaussian_copy = bind("gaussian", "copy_field", [c.c_void_p, c.c_uint32, c.c_size_t, pointer, c.c_size_t])
    gaussian_release = bind("gaussian", "release", [slot])
    update = bind("gaussian", "update", [c.c_void_p, c.c_void_p, c.POINTER(Options), c.POINTER(c.c_uint32)])
    defaults = getattr(lib, f"liblrhsmm_rs_update_options_default_{suffix}")
    defaults.argtypes, defaults.restype = [c.POINTER(Options)], c.c_uint32
    def call(name, *args):
        status = functions[name](*args)
        assert status == 0, (name, status)
    source, statistics, created, copy, matched = [c.c_void_p() for _ in range(5)]
    call("new", 2, 2, c.byref(statistics))
    assert gaussian_new(2, 2, c.byref(source)) == 0
    call("from_gaussian", source, c.byref(matched))
    assert gaussian_release(c.byref(source)) == 0
    sums, squares, occupancies = (scalar * 4)(-1, 2, 3, 4), (scalar * 4)(1, 4, 9, 16), (scalar * 2)(1, 2)
    values = Values(2, 2, 3, sums, squares, occupancies)
    call("replace", statistics, c.byref(values))
    call("create", c.byref(values), c.byref(created))
    call("clone", statistics, c.byref(copy))
    metadata = Layout()
    call("layout", statistics, c.byref(metadata))
    assert (metadata.mixtures, metadata.dimensions, metadata.state_occupancy) == (2, 2, 3)
    changed, result = scalar(-9), scalar(99)
    call("set_field", statistics, 0, 0, c.byref(changed), 1)
    call("copy_field", copy, 0, 0, c.byref(result), 1)
    assert result.value == -1
    call("set_occupancy", statistics, 5)
    call("layout", statistics, c.byref(metadata))
    assert metadata.state_occupancy == 5
    call("copy_field", statistics, 0, 4, None, 0)
    assert functions["set_field"](statistics, 0, 4, c.byref(changed), 1) == 2
    call("clear", statistics)
    call("copy_field", statistics, 0, 0, c.byref(result), 1)
    assert result.value == 0
    for owner in [statistics, created, copy, matched]:
        call("release", c.byref(owner))
        assert owner.value is None
    call("release", c.byref(statistics))
    assert functions["new"](0, 1, c.byref(statistics)) == 3
    assert statistics.value is None
    compared = 0
    for line in Path(f"tests/fixtures/{fixture}").read_text().splitlines():
        tokens = line.split()
        if tokens[0] != "mixture":
            continue
        scenario, component = map(int, tokens[1:3])
        count = 1 if scenario == 0 else 2
        arrays = [(scalar * len(values))(*values) for values in [[0.5] * count, [-7] * (count * 2), [8] * (count * 2), [0, 0.75] * count]]
        assert gaussian_create(c.byref(Parameters(count, 2, *arrays)), c.byref(source)) == 0
        occupancies = [2 + k for k in range(count)]
        if scenario == 2:
            occupancies[1] = 1e-8
        stats_arrays = [(scalar * len(values))(*values) for values in [[-3.5, 6] * count, [9.25, 12] * count, occupancies]]
        values = Values(count, 2, 1e-8 if scenario == 3 else 4, *stats_arrays)
        call("create", c.byref(values), c.byref(statistics))
        options, updated = Options(), c.c_uint32(99)
        assert defaults(c.byref(options)) == 0
        assert update(source, statistics, c.byref(options), c.byref(updated)) == 0
        assert updated.value == int(scenario != 3)
        weight, means, variances = scalar(), (scalar * 2)(), (scalar * 2)()
        assert gaussian_copy(source, 0, component, c.byref(weight), 1) == 0
        assert gaussian_copy(source, 1, 2 * component, means, 2) == 0
        assert gaussian_copy(source, 2, 2 * component, variances, 2) == 0
        for value, text in zip([weight.value, means[0], variances[0], means[1], variances[1]], tokens[3:]):
            target = scalar(float(text)).value
            assert abs(value - target) <= 2e-14 * max(1.0, abs(target))
            compared += 1
        options.duration_mode = 99
        updated.value = 99
        assert update(source, statistics, c.byref(options), c.byref(updated)) == 3
        assert updated.value == 99
        call("release", c.byref(statistics))
        assert gaussian_release(c.byref(source)) == 0
    assert compared == 35
print("Python Gaussian statistics: all24 exports and70 original C update values passed")
