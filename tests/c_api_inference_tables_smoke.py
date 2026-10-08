"""Actual ctypes calls for all typed inference-table exports in both precisions."""
import ctypes as c
import sys

lib = c.CDLL(sys.argv[1])
P, N, U = c.c_void_p, c.c_size_t, c.c_uint32


class Layout(c.Structure):
    _fields_ = [('frames', N), ('states', N), ('values', N)]


for suffix, scalar, bit_type, finite_bits, raw_bits in [
    ('f32', c.c_float, c.c_uint32, [0, 0x80000000, 1, 0x80000001, 0x3f800001, 0xbf000000], [0x7fc01234, 0x80000000, 0x7f800000, 0xff800000, 1, 0x3f800001]),
    ('f64', c.c_double, c.c_uint64, [0, 0x8000000000000000, 1, 0x8000000000000001, 0x3ff0000000000001, 0xbfe0000000000000], [0x7ff8000000001234, 0x8000000000000000, 0x7ff0000000000000, 0xfff0000000000000, 1, 0x3ff0000000000001]),
]:
    common = {
        'layout': [P, c.POINTER(Layout)],
        'copy_values': [P, N, c.POINTER(scalar), N],
        'copy_frame': [P, N, c.POINTER(scalar), N],
        'get': [P, N, N, c.POINTER(scalar)],
        'clone': [P, c.POINTER(P)],
        'release': [c.POINTER(P)],
    }
    signatures = {}
    for family in ['log_emissions', 'state_scores', 'emission_densities']:
        for name, arguments in common.items():
            signatures[family, name] = arguments
        if family != 'emission_densities':
            signatures[family, 'create'] = [N, N, c.POINTER(scalar), N, c.POINTER(P)]
            signatures[family, 'replace'] = [P, N, N, c.POINTER(scalar), N]
    signatures['log_emissions', 'probabilities'] = [P, c.POINTER(P)]
    signatures['state_scores', 'total'] = [P, c.POINTER(scalar)]
    signatures['state_scores', 'total_through_state'] = [P, P, N, c.POINTER(scalar)]
    signatures['state_scores', 'backward_total'] = [P, P, c.POINTER(scalar)]
    assert len(signatures) == 26
    functions, called = {}, set()
    for (family, name), arguments in signatures.items():
        function = getattr(lib, f'liblrhsmm_rs_{family}_{name}_{suffix}')
        function.argtypes, function.restype = arguments, U
        functions[family, name] = function

    def status(family, name, expected, *arguments):
        actual = functions[family, name](*arguments)
        assert actual == expected, (suffix, family, name, actual, expected)
        called.add((family, name))

    def call(family, name, *arguments):
        status(family, name, 0, *arguments)

    def check(family, owner, expected):
        layout, copied = Layout(99, 99, 99), (scalar * 6)()
        call(family, 'layout', owner, c.byref(layout))
        assert (layout.frames, layout.states, layout.values) == (2, 3, 6)
        call(family, 'copy_values', owner, 0, copied, 6)
        assert bytes(copied) == bytes(expected)
        call(family, 'copy_frame', owner, 1, copied, 3)
        assert bytes(copied)[:3 * c.sizeof(scalar)] == bytes(expected)[3 * c.sizeof(scalar):]
        value = scalar(-77)
        for frame in range(2):
            for state in range(3):
                call(family, 'get', owner, frame, state, c.byref(value))
                begin = (frame * 3 + state) * c.sizeof(scalar)
                assert bytes(value) == bytes(expected)[begin:begin + c.sizeof(scalar)]
        value.value = -77
        status(family, 'copy_values', 2, owner, 6, c.byref(value), 1)
        assert value.value == -77
        call(family, 'copy_values', owner, 6, None, 0)
        status(family, 'copy_values', 1, owner, 0, None, 1)
        status(family, 'get', 2, owner, 2, 0, c.byref(value))
        assert value.value == -77
        status(family, 'copy_frame', 2, owner, 0, c.byref(value), 2)
        assert value.value == -77

    finite, raw, density_values = (scalar * 6)(), (scalar * 6)(), (scalar * 6)()
    c.memmove(finite, (bit_type * 6)(*finite_bits), c.sizeof(finite))
    c.memmove(raw, (bit_type * 6)(*raw_bits), c.sizeof(raw))
    log, log_clone, scores, score_clone, density, density_clone = [P() for _ in range(6)]
    call('log_emissions', 'create', 2, 3, finite, 6, c.byref(log))
    call('state_scores', 'create', 2, 3, raw, 6, c.byref(scores))
    call('log_emissions', 'probabilities', log, c.byref(density))
    call('emission_densities', 'copy_values', density, 0, density_values, 6)
    assert density_values[0] == 1 and density_values[1] == 1
    for family, owner, expected, clone in [
        ('log_emissions', log, finite, log_clone),
        ('state_scores', scores, raw, score_clone),
        ('emission_densities', density, density_values, density_clone),
    ]:
        check(family, owner, expected)
        call(family, 'clone', owner, c.byref(clone))
    replacement = (scalar * 6)(-0.5, -1, -1.5, -2, -2.5, -3)
    call('log_emissions', 'replace', log, 2, 3, replacement, 6)
    call('state_scores', 'replace', scores, 2, 3, replacement, 6)
    invalid = (scalar * 6)(-0.5, -1, -1.5, -2, -2.5, float('nan'))
    status('log_emissions', 'replace', 3, log, 2, 3, invalid, 6)
    status('state_scores', 'replace', 3, scores, 2, 4, replacement, 6)
    check('log_emissions', log, replacement)
    check('state_scores', scores, replacement)
    call('state_scores', 'replace', score_clone, 2, 3, replacement, 6)
    log_add = getattr(lib, f'liblrhsmm_rs_log_add_{suffix}')
    log_add.argtypes, log_add.restype = [scalar, scalar, c.POINTER(scalar)], U
    expected, total = scalar(-1e15), scalar(99)
    for value in list(replacement)[3:]:
        assert log_add(expected, value, c.byref(expected)) == 0
    call('state_scores', 'total', scores, c.byref(total))
    assert bytes(total) == bytes(expected)
    expected.value = -1e15
    for index in [0, 3]:
        assert log_add(expected, replacement[index] * 2, c.byref(expected)) == 0
    call('state_scores', 'total_through_state', scores, score_clone, 0, c.byref(total))
    assert bytes(total) == bytes(expected)
    call('state_scores', 'backward_total', score_clone, log, c.byref(total))
    assert total.value == -1
    total.value = 77
    status('state_scores', 'total_through_state', 3, scores, score_clone, 3, c.byref(total))
    assert total.value == 77
    for family, owner in [('log_emissions', log), ('state_scores', scores), ('emission_densities', density)]:
        call(family, 'release', c.byref(owner))
        assert owner.value is None
    for family, clone, expected in [('log_emissions', log_clone, finite), ('state_scores', score_clone, replacement), ('emission_densities', density_clone, density_values)]:
        check(family, clone, expected)
        call(family, 'release', c.byref(clone))
        call(family, 'release', c.byref(clone))
    assert called == set(signatures), set(signatures) - called

print('Inference tables Python: all52 exports and independent table storage passed')
