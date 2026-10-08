"""Actual ctypes inference callers, including unchanged original C score/path corpora."""
import ctypes as c
import math
from pathlib import Path
import struct
import sys

lib = c.CDLL(sys.argv[1])
P, N, U, I = c.c_void_p, c.c_size_t, c.c_uint32, c.c_int32


class Layout(c.Structure):
    _fields_ = [('frames', N), ('states', N), ('occurrences', N)]


class Occurrence(c.Structure):
    _fields_ = [('state', N), ('end', N)]


for suffix, scalar, code, corpora in [
    ('f32', c.c_float, 'f', ['c-hsmm.txt']),
    ('f64', c.c_double, 'd', ['c-double-hsmm.txt', 'c-double-hsmm-fractional.txt']),
]:
    class Hsmm(c.Structure):
        _fields_ = [('temperature', scalar), ('duration_weight', scalar), ('state_radius', scalar), ('duration_extra', N), ('duration_extra_factor', scalar)]

    class Geometric(c.Structure):
        _fields_ = [('temperature', scalar), ('pruning_slope', scalar)]

    class Preparation(c.Structure):
        _fields_ = [('duration_cache_length', N), ('duration_weight', scalar), ('temperature', scalar)]

    class Duration(c.Structure):
        _fields_ = [('mean', scalar), ('variance', scalar), ('minimum', I), ('maximum', I), ('fixed_mean', I), ('variance_floor', scalar)]

    class Jump(c.Structure):
        _fields_ = [('delta', I), ('probability', scalar)]

    class JumpSlice(c.Structure):
        _fields_ = [('jumps', c.POINTER(Jump)), ('count', N)]

    functions, tracked, called = {}, set(), set()

    def register(name, arguments, new=True):
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

    for engine, descriptor in [('hsmm', Hsmm), ('geometric', Geometric)]:
        register(f'{engine}_settings_default', [c.POINTER(descriptor)])
        for owner in (['model', 'prepared_model'] if engine == 'hsmm' else ['model']):
            for cached in ([False, True] if engine == 'hsmm' else [False]):
                for operation in ['forward', 'backward', 'viterbi']:
                    register(f'{owner}_{operation}_{engine}' + ('_with_graph' if cached else ''), [P, P, P, c.POINTER(descriptor), c.POINTER(P)])
    for family in ['hsmm_scores', 'hsmm_path', 'geometric_path']:
        register(f'{family}_clone', [P, c.POINTER(P)])
        register(f'{family}_release', [c.POINTER(P)])
        if family == 'hsmm_scores':
            for row in ['begin', 'end']:
                register(f'{family}_get_{row}', [P, c.POINTER(P)])
            register(f'{family}_create', [P, P, c.POINTER(P)])
        else:
            register(f'{family}_layout', [P, c.POINTER(Layout)])
            register(f'{family}_log_probability', [P, c.POINTER(scalar)])
            register(f'{family}_get_scores', [P, c.POINTER(P)])
            if family == 'hsmm_path':
                register(f'{family}_copy_occurrences', [P, N, c.POINTER(Occurrence), N])
            else:
                for array in ['states', 'boundaries']:
                    register(f'{family}_copy_{array}', [P, N, c.POINTER(N), N])
    assert len(tracked) == 35
    register('duration_default', [c.POINTER(Duration)], False)
    register('duration_create', [c.POINTER(Duration), c.POINTER(P)], False)
    register('model_create', [c.POINTER(P), N, c.POINTER(P), N, c.POINTER(P)], False)
    register('segmentation_create', [N, N, c.POINTER(I), c.POINTER(I), c.POINTER(c.POINTER(I)), c.POINTER(JumpSlice), c.POINTER(P)], False)
    register('segmentation_build_transitions', [P, c.POINTER(P)], False)
    register('log_emissions_create', [N, N, c.POINTER(scalar), N, c.POINTER(P)], False)
    register('model_prepare', [P, c.POINTER(Preparation), c.POINTER(P)], False)
    register('state_scores_copy_values', [P, N, c.POINTER(scalar), N], False)
    for family in ['duration', 'model', 'segmentation', 'transition_graph', 'log_emissions', 'prepared_model', 'state_scores']:
        register(f'{family}_release', [c.POINTER(P)], False)

    def release(family, owner):
        call(f'{family}_release', c.byref(owner))
        assert owner.value is None

    def scores(owner):
        begin, end = P(), P()
        call('hsmm_scores_get_begin', owner, c.byref(begin))
        call('hsmm_scores_get_end', owner, c.byref(end))
        imported, clone = P(), P()
        call('hsmm_scores_create', begin, end, c.byref(imported))
        call('hsmm_scores_clone', imported, c.byref(clone))
        release('hsmm_scores', imported)
        release('hsmm_scores', clone)
        release('hsmm_scores', owner)
        result = []
        for child in [begin, end]:
            values = (scalar * 48)()
            call('state_scores_copy_values', child, 0, values, 48)
            result.append(bytes(values))
            release('state_scores', child)
        return result

    comparisons = 0
    for fractional, filename in enumerate(corpora):
        lines = [line.split() for line in (Path(__file__).parent / 'fixtures' / filename).read_text().splitlines()]
        for scenario in range(4):
            durations = []
            for state in range(4):
                descriptor = Duration()
                call('duration_default', c.byref(descriptor))
                descriptor.mean, descriptor.variance = 2.5 + state * 0.25, 0.75 + state * 0.25
                descriptor.minimum = 2 if state == 1 else -1
                descriptor.maximum = 3 if scenario == 3 and state == 1 else 7
                owner = P()
                call('duration_create', c.byref(descriptor), c.byref(owner))
                durations.append(owner)
            borrowed = (P * 4)(*[v.value for v in durations])
            model, seg, graph, log, prepared = P(), P(), P(), P(), P()
            call('model_create', None, 0, borrowed, 4, c.byref(model))
            for owner in durations:
                release('duration', owner)
            jumps = []
            for state in range(4):
                row = [(1, 1)]
                if scenario and state < 3:
                    row = [([2, 0, -1][state], [0.2, 0.15, 0.1][state]), (1, 0.8)]
                jumps.append((Jump * len(row))(*[Jump(delta, probability) for delta, probability in row]))
            slices = (JumpSlice * 4)(*[JumpSlice(row, len(row)) for row in jumps])
            call('segmentation_create', 0, 4, (I * 4)(3, 6, 9, 12), (I * 4)(0, 1, 2, 3), None, slices, c.byref(seg))
            call('segmentation_build_transitions', seg, c.byref(graph))
            desired = [0, 0, 2, 2, 1, 1, 1, 1, 2, 3, 3, 3]
            values = (scalar * 48)(*[(0 if state == desired[frame] else -50) if scenario == 3 else -float(frame - state * 3) ** 2 * 0.125 for frame in range(12) for state in range(4)])
            call('log_emissions_create', 12, 4, values, 48, c.byref(log))
            options = Hsmm()
            call('hsmm_settings_default', c.byref(options))
            assert (options.temperature, options.duration_weight, options.state_radius, options.duration_extra, options.duration_extra_factor) == (1, 1, 5, 30, 1)
            options = Hsmm(0.5 if scenario == 2 else 1, 0.75, [0.25, 0.5, 1.75, 2.5][scenario] if fractional else (1 if scenario == 2 else 5), 3, 0.5)
            preparation = Preparation(2, options.duration_weight, options.temperature)
            call('model_prepare', model, c.byref(preparation), c.byref(prepared))
            forward, backward = P(), P()
            call('model_forward_hsmm', model, seg, log, c.byref(options), c.byref(forward))
            call('model_backward_hsmm', model, seg, log, c.byref(options), c.byref(backward))
            reference = {'forward': scores(forward), 'backward': scores(backward)}
            actual = [struct.unpack('=' + code * 48, data) for data in reference['forward'] + reference['backward']]
            for fields in lines:
                if fields[0] != 'score' or int(fields[1]) != scenario:
                    continue
                index = int(fields[2])
                for field, expected in enumerate(map(float, fields[3:])):
                    if suffix == 'f32':
                        assert struct.pack('=f', actual[field][index]) == struct.pack('=f', expected), fields
                    else:
                        assert abs(actual[field][index] - expected) / max(1, abs(expected)) <= 2e-14, fields
                    comparisons += 1
            path = P()
            for owner, handle in [('model', model), ('prepared_model', prepared)]:
                for cached, topology in [(False, seg), (True, graph)]:
                    tail = '_with_graph' if cached else ''
                    for operation in ['forward', 'backward']:
                        result = P()
                        call(f'{owner}_{operation}_hsmm{tail}', handle, topology, log, c.byref(options), c.byref(result))
                        assert scores(result) == reference[operation]
                    result = P()
                    call(f'{owner}_viterbi_hsmm{tail}', handle, topology, log, c.byref(options), c.byref(result))
                    release('hsmm_path', path)
                    path = result
            retained = P()
            call('hsmm_path_clone', path, c.byref(retained))
            release('hsmm_path', path)
            release('prepared_model', prepared)
            layout = Layout()
            call('hsmm_path_layout', retained, c.byref(layout))
            expected_path = [(int(fields[4]), int(fields[3])) for fields in lines if fields[0] == 'path' and int(fields[1]) == scenario]
            assert (layout.frames, layout.states, layout.occurrences) == (12, 4, len(expected_path))
            occurrences = (Occurrence * layout.occurrences)()
            call('hsmm_path_copy_occurrences', retained, 0, occurrences, len(occurrences))
            assert [(v.state, v.end) for v in occurrences] == expected_path
            call('hsmm_path_copy_occurrences', retained, len(occurrences), None, 0)
            likelihood = scalar()
            call('hsmm_path_log_probability', retained, c.byref(likelihood))
            assert math.isfinite(likelihood.value)
            child = P()
            call('hsmm_path_get_scores', retained, c.byref(child))
            release('hsmm_path', retained)
            scores(child)
            geo = Geometric()
            call('geometric_settings_default', c.byref(geo))
            geo.pruning_slope = 10
            for operation in ['forward', 'backward']:
                child = P()
                call(f'model_{operation}_geometric', model, seg, log, c.byref(geo), c.byref(child))
                release('state_scores', child)
            path, clone = P(), P()
            call('model_viterbi_geometric', model, seg, log, c.byref(geo), c.byref(path))
            call('geometric_path_clone', path, c.byref(clone))
            release('geometric_path', path)
            options.temperature = math.nan
            sentinel = P(0x1000)
            status('model_forward_hsmm', 3, model, seg, log, c.byref(options), c.byref(sentinel))
            assert sentinel.value == 0x1000
            for family, owner in [('model', model), ('segmentation', seg), ('transition_graph', graph), ('log_emissions', log)]:
                release(family, owner)
            call('geometric_path_layout', clone, c.byref(layout))
            assert (layout.frames, layout.states, layout.occurrences) == (12, 4, 4)
            states, ends = (N * 12)(), (N * 4)()
            call('geometric_path_copy_states', clone, 0, states, 12)
            call('geometric_path_copy_boundaries', clone, 0, ends, 4)
            assert states[0] == 0 and states[11] == 3 and ends[3] == 12
            assert all(states[i - 1] <= states[i] <= states[i - 1] + 1 for i in range(1, 12))
            call('geometric_path_log_probability', clone, c.byref(likelihood))
            assert math.isfinite(likelihood.value)
            call('geometric_path_get_scores', clone, c.byref(child))
            release('geometric_path', clone)
            release('state_scores', child)
            release('geometric_path', clone)
    assert comparisons == 768 * len(corpora)
    assert called == tracked, tracked - called
print('Inference Python: all70 exports, original C scores and occurrence paths passed')
