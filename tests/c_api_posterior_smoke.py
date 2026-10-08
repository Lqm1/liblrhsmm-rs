"""Exercise all posterior C exports from ctypes, including complete owned storage."""
import ctypes as c
import math
import struct
import sys

lib = c.CDLL(sys.argv[1])
P, N, U, I = c.c_void_p, c.c_size_t, c.c_uint32, c.c_int32

class Layout(c.Structure):
    _fields_ = [('frames', N), ('states', N), ('values', N)]

class PosteriorInput(c.Structure):
    _fields_ = [('present', U), ('minimum', N), ('values', c.POINTER(c.c_float)), ('count', N)]

class Slice(c.Structure):
    _fields_ = [('present', U), ('minimum', N), ('values', N)]

class Arrival(c.Structure):
    _fields_ = [('present', U), ('stay', c.c_float), ('advance', c.c_float)]

common_functions, common_called = {}, set()

def common_register(name, arguments):
    function = getattr(lib, 'liblrhsmm_rs_' + name)
    function.argtypes, function.restype = arguments, U
    common_functions[name] = function

def common_status(name, expected, *arguments):
    actual = common_functions[name](*arguments)
    assert actual == expected, (name, actual, expected)
    common_called.add(name)

def common_call(name, *arguments):
    common_status(name, 0, *arguments)

def common_release(family, handle):
    common_call(family + '_release', c.byref(handle))
    assert handle.value is None

for family, descriptor in [('duration_occupancies', PosteriorInput), ('mixture_occupancies', PosteriorInput), ('transition_occupancies', Arrival)]:
    common_register(family + '_create', [N, N, c.POINTER(descriptor), N, c.POINTER(P)])
    common_register(family + '_replace', [P, N, N, c.POINTER(descriptor), N])
    common_register(family + '_layout', [P, c.POINTER(Layout)])
    common_register(family + '_clone', [P, c.POINTER(P)])
    common_register(family + '_release', [c.POINTER(P)])
    if family == 'transition_occupancies':
        common_register(family + '_get', [P, N, N, c.POINTER(Arrival)])
    else:
        common_register(family + '_get_slice', [P, N, N, c.POINTER(Slice)])
        common_register(family + '_copy_values', [P, N, N, N, c.POINTER(c.c_float), N])
common_register('duration_occupancies_probability', [P, N, N, N, c.POINTER(c.c_float)])
assert len(common_functions) == 21
weights = (c.c_float * 4).from_buffer_copy(struct.pack('=IIII', 0, 0x80000000, 1, 0x40000000))
expected = bytes(weights)
maximum = N(-1).value
inputs = (PosteriorInput * 3)(PosteriorInput(0, maximum, None, maximum), PosteriorInput(1, maximum, None, 0), PosteriorInput(1, 3, weights, 4))
layout, copied, probability = Layout(), (c.c_float * 4)(), c.c_float()
for family in ['duration_occupancies', 'mixture_occupancies']:
    owner, clone = P(), P()
    common_call(family + '_create', 1, 3, inputs, 3, c.byref(owner))
    common_call(family + '_clone', owner, c.byref(clone))
    common_call(family + '_layout', owner, c.byref(layout))
    assert (layout.frames, layout.states, layout.values) == (1, 3, 3)
    for state in range(3):
        description = Slice()
        common_call(family + '_get_slice', owner, 0, state, c.byref(description))
        assert description.present == (state != 0)
        assert description.values == (4 if state == 2 else 0)
        minimum = (0 if state == 0 else maximum if state == 1 else 3) if family == 'duration_occupancies' else 0
        assert description.minimum == minimum
    weights[3] = math.nan
    common_status(family + '_replace', 3, owner, 3, 1, inputs, 3)
    common_call(family + '_copy_values', owner, 0, 2, 0, copied, 4)
    assert bytes(copied) == expected
    common_call(family + '_copy_values', owner, 0, 0, 0, None, 0)
    common_status(family + '_copy_values', 2, owner, 0, 2, 5, None, 0)
    inputs[2].present = 2
    common_status(family + '_replace', 3, owner, 1, 3, inputs, 3)
    inputs[2].present = 1
    if family == 'duration_occupancies':
        for index in range(4):
            common_call('duration_occupancies_probability', owner, 0, 2, index + 3, c.byref(probability))
            assert bytes(probability) == expected[4 * index:4 * (index + 1)]
    weights[3] = 2
    common_call(family + '_replace', owner, 3, 1, inputs, 3)
    common_release(family, owner)
    common_call(family + '_copy_values', clone, 0, 2, 0, copied, 4)
    assert bytes(copied) == expected
    common_release(family, clone)
arrivals = (Arrival * 3)(Arrival(1, -0.0, weights[2]), Arrival(0, math.nan, math.inf), Arrival(1, 2, 3))
owner, clone, arrival = P(), P(), Arrival()
common_call('transition_occupancies_create', 1, 3, arrivals, 3, c.byref(owner))
common_call('transition_occupancies_clone', owner, c.byref(clone))
common_call('transition_occupancies_layout', owner, c.byref(layout))
assert (layout.frames, layout.states, layout.values) == (1, 3, 3)
arrivals[2].advance = -1
common_status('transition_occupancies_replace', 3, owner, 3, 1, arrivals, 3)
common_call('transition_occupancies_get', owner, 0, 2, c.byref(arrival))
assert (arrival.present, arrival.stay, arrival.advance) == (1, 2, 3)
arrivals[2].advance = 3
common_call('transition_occupancies_replace', owner, 3, 1, arrivals, 3)
common_release('transition_occupancies', owner)
common_call('transition_occupancies_get', clone, 0, 0, c.byref(arrival))
assert arrival.present == 1
assert struct.pack('=ff', arrival.stay, arrival.advance) == expected[4:12]
common_call('transition_occupancies_get', clone, 0, 1, c.byref(arrival))
assert (arrival.present, arrival.stay, arrival.advance) == (0, 0, 0)
common_release('transition_occupancies', clone)
assert common_called == set(common_functions)

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

    register('emission_settings_default', [c.POINTER(Settings)])
    for owner in ['model', 'prepared_model']:
        for method in ['emission_log_probabilities', 'emission_probabilities']:
            register(f'{owner}_{method}', [P, P, P, c.POINTER(Settings), c.POINTER(P)])
    assert not tracked
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


    class HsmmSettings(c.Structure):
        _fields_ = [('temperature', scalar), ('duration_weight', scalar), ('state_radius', scalar), ('duration_extra', N), ('duration_extra_factor', scalar)]

    class GeometricSettings(c.Structure):
        _fields_ = [('temperature', scalar), ('pruning_slope', scalar)]

    register('state_occupancies_create', [N, N, c.POINTER(scalar), N, c.POINTER(P)], True)
    register('state_occupancies_replace', [P, N, N, c.POINTER(scalar), N], True)
    register('state_occupancies_layout', [P, c.POINTER(Layout)], True)
    register('state_occupancies_get', [P, N, N, c.POINTER(scalar)], True)
    register('state_occupancies_copy_values', [P, N, c.POINTER(scalar), N], True)
    register('state_occupancies_copy_frame', [P, N, c.POINTER(scalar), N], True)
    register('state_occupancies_clone', [P, c.POINTER(P)], True)
    register('state_occupancies_release', [c.POINTER(P)], True)
    register('state_scores_geometric_occupancies', [P, P, scalar, scalar, c.POINTER(P)], True)
    for name in ['model_duration_occupancies', 'model_duration_occupancies_with_graph']:
        register(name, [P, P, P, P, P, scalar, c.POINTER(HsmmSettings), c.POINTER(P)], True)
    register('model_hsmm_state_occupancies', [P, P, P, P, c.POINTER(HsmmSettings), c.POINTER(P)], True)
    register('model_geometric_transition_occupancies', [P, P, P, P, P, scalar, c.POINTER(GeometricSettings), c.POINTER(P)], True)
    for name in ['model_mixture_occupancies_hsmm', 'model_mixture_occupancies_geometric']:
        register(name, [P, P, P, P, N, scalar, scalar, c.POINTER(P)], True)
    assert len(tracked) == 15
    register('hsmm_settings_default', [c.POINTER(HsmmSettings)])
    register('geometric_settings_default', [c.POINTER(GeometricSettings)])
    for name in ['model_forward_hsmm', 'model_backward_hsmm']:
        register(name, [P, P, P, c.POINTER(HsmmSettings), c.POINTER(P)])
    for name in ['model_forward_geometric', 'model_backward_geometric']:
        register(name, [P, P, P, c.POINTER(GeometricSettings), c.POINTER(P)])
    register('hsmm_scores_get_end', [P, c.POINTER(P)])
    register('state_scores_total_through_state', [P, P, N, c.POINTER(scalar)])
    register('state_scores_total', [P, c.POINTER(scalar)])
    register('segmentation_build_transitions', [P, c.POINTER(P)])
    for family in ['hsmm_scores', 'state_scores', 'transition_graph']:
        register(family + '_release', [c.POINTER(P)])

    bits_code, negative_zero = ('I', 0x80000000) if suffix == 'f32' else ('Q', 0x8000000000000000)
    weights = (scalar * 4).from_buffer_copy(struct.pack('=' + bits_code * 4, 0, negative_zero, 1, 0x40000000 if suffix == 'f32' else 0x4000000000000000))
    expected, copied = bytes(weights), (scalar * 4)()
    owner, clone, value = P(), P(), scalar()
    call('state_occupancies_create', 2, 2, weights, 4, c.byref(owner))
    call('state_occupancies_clone', owner, c.byref(clone))
    weights[3] = math.nan
    status('state_occupancies_replace', 3, owner, 1, 4, weights, 4)
    call('state_occupancies_layout', owner, c.byref(layout))
    assert (layout.frames, layout.states, layout.values) == (2, 2, 4)
    call('state_occupancies_copy_values', owner, 0, copied, 4)
    assert bytes(copied) == expected
    call('state_occupancies_copy_frame', owner, 0, copied, 2)
    assert bytes(copied)[:2 * c.sizeof(scalar)] == expected[:2 * c.sizeof(scalar)]
    call('state_occupancies_get', owner, 1, 0, c.byref(value))
    assert bytes(value) == struct.pack('=' + bits_code, 1)
    weights[3] = 2
    call('state_occupancies_replace', owner, 1, 4, weights, 4)
    call('state_occupancies_copy_values', owner, 4, None, 0)
    status('state_occupancies_copy_values', 2, owner, 5, None, 0)
    release('state_occupancies', owner)
    call('state_occupancies_copy_values', clone, 0, copied, 4)
    assert bytes(copied) == expected
    release('state_occupancies', clone)

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

    emission = Settings(0.75, 0, 0, 0)
    log, forward, backward, forward_end, backward_end, graph = P(), P(), P(), P(), P(), P()
    call('model_emission_log_probabilities', model, observation, segmentation, c.byref(emission), c.byref(log))
    hsmm = HsmmSettings()
    call('hsmm_settings_default', c.byref(hsmm))
    hsmm.temperature = 0.75
    call('model_forward_hsmm', model, segmentation, log, c.byref(hsmm), c.byref(forward))
    call('model_backward_hsmm', model, segmentation, log, c.byref(hsmm), c.byref(backward))
    call('hsmm_scores_get_end', forward, c.byref(forward_end))
    call('hsmm_scores_get_end', backward, c.byref(backward_end))
    total = scalar()
    call('state_scores_total_through_state', forward_end, backward_end, 0, c.byref(total))
    assert math.isfinite(total.value)
    call('segmentation_build_transitions', segmentation, c.byref(graph))
    duration, cached, hsmm_state, geometric_state = P(), P(), P(), P()
    call('model_duration_occupancies', model, segmentation, log, forward, backward, total, c.byref(hsmm), c.byref(duration))
    call('model_duration_occupancies_with_graph', model, graph, log, forward, backward, total, c.byref(hsmm), c.byref(cached))
    positive_durations = 0
    for frame in range(12):
        for state in range(3):
            descriptions, tables = [], []
            for handle in [duration, cached]:
                description = Slice()
                common_call('duration_occupancies_get_slice', handle, frame, state, c.byref(description))
                values = (c.c_float * description.values)()
                common_call('duration_occupancies_copy_values', handle, frame, state, 0, values, len(values))
                descriptions.append((description.present, description.minimum, description.values))
                tables.append(bytes(values))
                assert all(math.isfinite(v) and v >= 0 for v in values)
                positive_durations += sum(v > 0 for v in values)
            assert descriptions[0] == descriptions[1] and tables[0] == tables[1]
    assert positive_durations > 0
    call('model_hsmm_state_occupancies', model, segmentation, log, duration, c.byref(hsmm), c.byref(hsmm_state))
    geometric = GeometricSettings(0.75, 10)
    gf, gb, transition = P(), P(), P()
    call('model_forward_geometric', model, segmentation, log, c.byref(geometric), c.byref(gf))
    call('model_backward_geometric', model, segmentation, log, c.byref(geometric), c.byref(gb))
    call('state_scores_total', gf, c.byref(total))
    call('state_scores_geometric_occupancies', gf, gb, total, geometric.pruning_slope, c.byref(geometric_state))
    call('model_geometric_transition_occupancies', model, segmentation, log, gf, gb, total, c.byref(geometric), c.byref(transition))
    positive_mixtures = 0
    for stream in range(2):
        for method, occupancy, pruning in [('model_mixture_occupancies_hsmm', hsmm_state, 0.5), ('model_mixture_occupancies_geometric', geometric_state, 0.34)]:
            mixture = P()
            call(method, model, observation, segmentation, occupancy, stream, 0.75, pruning, c.byref(mixture))
            for frame in range(12):
                for state in range(3):
                    description = Slice()
                    common_call('mixture_occupancies_get_slice', mixture, frame, state, c.byref(description))
                    values = (c.c_float * description.values)()
                    common_call('mixture_occupancies_copy_values', mixture, frame, state, 0, values, len(values))
                    assert all(math.isfinite(v) and v >= 0 for v in values)
                    positive_mixtures += sum(v > 0 for v in values)
            common_release('mixture_occupancies', mixture)
    assert positive_mixtures > 0
    sentinel = P(0x1000)
    status('model_duration_occupancies', 3, model, segmentation, log, forward, backward, math.nan, c.byref(hsmm), c.byref(sentinel))
    assert sentinel.value == 0x1000
    for family, handle in [('model', model), ('observation', observation), ('segmentation', segmentation)]:
        release(family, handle)
    retained = (scalar * 36)()
    call('state_occupancies_copy_values', hsmm_state, 0, retained, 36)
    assert all(math.isfinite(v) and v >= 0 for v in retained)
    for family, handle in [('state_occupancies', hsmm_state), ('state_occupancies', geometric_state), ('state_scores', gf), ('state_scores', gb), ('state_scores', forward_end), ('state_scores', backward_end), ('hsmm_scores', forward), ('hsmm_scores', backward), ('transition_graph', graph), ('log_emissions', log)]:
        release(family, handle)
    for family, handle in [('duration_occupancies', duration), ('duration_occupancies', cached), ('transition_occupancies', transition)]:
        common_release(family, handle)
    assert called == tracked, (suffix, tracked - called)
print('All 51 posterior ctypes exports passed')
