"""Actual ctypes weighted multistream emission computation in both storage precisions."""
import ctypes as c
import math
from pathlib import Path
import struct
import sys

lib = c.CDLL(sys.argv[1])
P, N, U, I = c.c_void_p, c.c_size_t, c.c_uint32, c.c_int32


class Layout(c.Structure):
    _fields_ = [('frames', N), ('states', N), ('values', N)]


for suffix, scalar, code, files in [
    ('f32', c.c_float, 'f', ['c-emission-tables.txt', 'c-fractional-emissions.txt']),
    ('f64', c.c_double, 'd', ['c-double-emissions.txt', 'c-double-emissions-fractional.txt']),
]:
    class Settings(c.Structure):
        _fields_ = [('temperature', scalar), ('pruning', U), ('radius', scalar), ('slope', scalar)]

    class Gaussian(c.Structure):
        _fields_ = [('mixtures', N), ('dimensions', N), ('weights', c.POINTER(scalar)), ('means', c.POINTER(scalar)), ('variances', c.POINTER(scalar)), ('variance_floors', c.POINTER(scalar))]

    class Duration(c.Structure):
        _fields_ = [('mean', scalar), ('variance', scalar), ('minimum', I), ('maximum', I), ('fixed_mean', I), ('variance_floor', scalar)]

    class Preparation(c.Structure):
        _fields_ = [('duration_cache_length', N), ('duration_weight', scalar), ('temperature', scalar)]

    class Jump(c.Structure):
        _fields_ = [('delta', I), ('probability', scalar)]

    class JumpSlice(c.Structure):
        _fields_ = [('jumps', c.POINTER(Jump)), ('count', N)]

    functions, tracked, called = {}, set(), set()

    def register(name, arguments, new=False):
        function = getattr(lib, f'liblrhsmm_rs_{name}_{suffix}')
        function.argtypes, function.restype = arguments, U
        functions[name] = function
        if new:
            tracked.add(name)

    def status(name, expected, *arguments):
        actual = functions[name](*arguments)
        assert actual == expected, (suffix, name, actual, expected)
        if name in tracked:
            called.add(name)

    def call(name, *arguments):
        status(name, 0, *arguments)

    def release(family, handle):
        call(f'{family}_release', c.byref(handle))
        assert handle.value is None

    register('emission_settings_default', [c.POINTER(Settings)], True)
    for owner in ['model', 'prepared_model']:
        for method in ['emission_log_probabilities', 'emission_probabilities']:
            register(f'{owner}_{method}', [P, P, P, c.POINTER(Settings), c.POINTER(P)], True)
    assert len(tracked) == 5
    register('gaussian_create', [c.POINTER(Gaussian), c.POINTER(P)])
    register('stream_create', [scalar, c.POINTER(P), N, c.POINTER(P)])
    register('duration_default', [c.POINTER(Duration)])
    register('duration_create', [c.POINTER(Duration), c.POINTER(P)])
    register('model_create', [c.POINTER(P), N, c.POINTER(P), N, c.POINTER(P)])
    register('model_prepare', [P, c.POINTER(Preparation), c.POINTER(P)])
    register('observation_stream_create', [N, c.POINTER(scalar), N, c.POINTER(P)])
    register('observation_create', [N, c.POINTER(P), N, c.POINTER(P)])
    register('segmentation_create', [N, N, c.POINTER(I), c.POINTER(I), c.POINTER(c.POINTER(I)), c.POINTER(JumpSlice), c.POINTER(P)])
    for family in ['log_emissions', 'emission_densities']:
        register(f'{family}_layout', [P, c.POINTER(Layout)])
        register(f'{family}_copy_values', [P, N, c.POINTER(scalar), N])
    for family in ['gaussian', 'stream', 'duration', 'model', 'prepared_model', 'observation_stream', 'observation', 'segmentation', 'log_emissions', 'emission_densities']:
        register(f'{family}_release', [c.POINTER(P)])

    def values(family, owner):
        layout = Layout()
        call(f'{family}_layout', owner, c.byref(layout))
        assert (layout.frames, layout.states, layout.values) == (12, 3, 36)
        result = (scalar * 36)()
        call(f'{family}_copy_values', owner, 0, result, 36)
        return bytes(result)

    streams, features = [], []
    for stream in range(2):
        components = dimensions = 2 if stream == 0 else 1
        emissions = []
        for state in range(3):
            weights = (scalar * components)(*[1 / components] * components)
            means = (scalar * (components * dimensions))(*[state * 0.5 + index * 0.25 for index in range(components * dimensions)])
            variances = (scalar * (components * dimensions))(*[0.75 + index * 0.25 for index in range(components * dimensions)])
            floors = (scalar * (components * dimensions))()
            descriptor = Gaussian(components, dimensions, weights, means, variances, floors)
            handle = P()
            call('gaussian_create', c.byref(descriptor), c.byref(handle))
            emissions.append(handle)
        borrowed = (P * 3)(*[handle.value for handle in emissions])
        handle = P()
        call('stream_create', 0.5 if stream == 0 else 1.25, borrowed, 3, c.byref(handle))
        streams.append(handle)
        for child in emissions:
            release('gaussian', child)
        samples = (scalar * (12 * dimensions))(*[frame * 0.125 + dimension * 0.5 for frame in range(12) for dimension in range(dimensions)])
        handle = P()
        call('observation_stream_create', dimensions, samples, len(samples), c.byref(handle))
        features.append(handle)
    durations = []
    for _ in range(3):
        descriptor = Duration()
        call('duration_default', c.byref(descriptor))
        descriptor.mean, descriptor.variance = 4, 1
        handle = P()
        call('duration_create', c.byref(descriptor), c.byref(handle))
        durations.append(handle)
    model, observation, segmentation, prepared = P(), P(), P(), P()
    call('model_create', (P * 2)(*[v.value for v in streams]), 2, (P * 3)(*[v.value for v in durations]), 3, c.byref(model))
    call('observation_create', 12, (P * 2)(*[v.value for v in features]), 2, c.byref(observation))
    for family, handles in [('stream', streams), ('duration', durations), ('observation_stream', features)]:
        for handle in handles:
            release(family, handle)
    labels = [(I * 3)(0, 1, 2), (I * 3)(2, 1, 0)]
    rows = (c.POINTER(I) * 2)(*labels)
    jump = (Jump * 1)(Jump(1, 1))
    slices = (JumpSlice * 3)(*[JumpSlice(jump, 1) for _ in range(3)])
    call('segmentation_create', 2, 3, (I * 3)(4, 8, 12), (I * 3)(0, 1, 2), rows, slices, c.byref(segmentation))
    preparation = Preparation(2, 1, 1)
    call('model_prepare', model, c.byref(preparation), c.byref(prepared))
    defaults = Settings()
    call('emission_settings_default', c.byref(defaults))
    assert (defaults.temperature, defaults.pruning, defaults.radius) == (1, 0, 5)
    tables = []
    for filename, count in zip(files, [2, 4]):
        groups = [[None] * 36 for _ in range(count)]
        for line in (Path(__file__).parent / 'fixtures' / filename).read_text().splitlines():
            group, index, value = line.split()
            groups[int(group)][int(index)] = float(value)
        assert all(all(value is not None for value in group) for group in groups)
        tables += groups
    density_reference = [float(line.split()[2]) for line in (Path(__file__).parent / 'fixtures/c-probability-forms.txt').read_text().splitlines() if line.startswith('emission ')]
    assert len(density_reference) == 36
    comparisons = 0
    for mode in range(7):
        options = Settings(0.75, 0 if mode == 6 else (2 if mode == 1 else 1), math.nan if mode == 6 else [1, 1, 0.25, 0.5, 1.75, 2.5][mode], 0.34 if mode == 1 else math.nan)
        results = {}
        for owner, handle in [('model', model), ('prepared_model', prepared)]:
            for method, family in [('emission_log_probabilities', 'log_emissions'), ('emission_probabilities', 'emission_densities')]:
                result = P()
                call(f'{owner}_{method}', handle, observation, segmentation, c.byref(options), c.byref(result))
                results[owner, family] = values(family, result)
                release(family, result)
        for family in ['log_emissions', 'emission_densities']:
            assert results['model', family] == results['prepared_model', family]
        if mode < 6:
            actual = struct.unpack('=' + code * 36, results['model', 'log_emissions'])
            for actual_value, expected in zip(actual, tables[mode]):
                if suffix == 'f32':
                    assert struct.pack('=f', actual_value) == struct.pack('=f', expected)
                else:
                    assert abs(actual_value - expected) / max(1, abs(expected)) <= 2e-14
                comparisons += 1
        if suffix == 'f32' and mode == 0:
            assert results['model', 'emission_densities'] == struct.pack('=36f', *density_reference)
    assert comparisons == 216
    invalid = Settings(1, 99, 5, 0.3)
    sentinel = P(0x1000)
    for owner, handle in [('model', model), ('prepared_model', prepared)]:
        status(f'{owner}_emission_log_probabilities', 3, handle, observation, segmentation, c.byref(invalid), c.byref(sentinel))
        assert sentinel.value == 0x1000
    status('emission_settings_default', 1, None)
    status('model_emission_log_probabilities', 1, None, observation, segmentation, c.byref(defaults), c.byref(sentinel))
    release('model', model)
    log, density = P(), P()
    call('prepared_model_emission_log_probabilities', prepared, observation, segmentation, c.byref(defaults), c.byref(log))
    call('prepared_model_emission_probabilities', prepared, observation, segmentation, c.byref(defaults), c.byref(density))
    release('prepared_model', prepared)
    release('observation', observation)
    release('segmentation', segmentation)
    values('log_emissions', log)
    values('emission_densities', density)
    release('log_emissions', log)
    release('emission_densities', density)
    assert called == tracked, tracked - called
print('Emission Python: all10 exports,432 original C log values and36 float densities passed')
