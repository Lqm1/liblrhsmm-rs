"""Call every segmentation export through ctypes, with independent owned copies."""
import ctypes as c
import sys

lib = c.CDLL(sys.argv[1])
P, N, U, I = c.c_void_p, c.c_size_t, c.c_uint32, c.c_int32
IP = c.POINTER(I)


class Layout(c.Structure):
    _fields_ = [('streams', N), ('segments', N)]


class Occurrence(c.Structure):
    _fields_ = [('state', N), ('end', N)]


for suffix, scalar, bit_type, patterns in [
    ('f32', c.c_float, c.c_uint32, [0x7fc01234, 0x80000000, 1]),
    ('f64', c.c_double, c.c_uint64, [0x7ff8000000001234, 0x8000000000000000, 0x3ff0000000000001]),
]:
    class Jump(c.Structure):
        _fields_ = [('delta', I), ('probability', scalar)]

    class Group(c.Structure):
        _fields_ = [('jumps', c.POINTER(Jump)), ('count', N)]

    full = [N, N, IP, IP, c.POINTER(IP), c.POINTER(Group)]
    signatures = {
        'new': [N, N, c.POINTER(P)],
        'create': full + [c.POINTER(P)],
        'replace': [P] + full,
        'layout': [P, c.POINTER(Layout)],
        'copy_boundaries': [P, N, IP, N],
        'set_boundaries': [P, N, IP, N],
        'copy_duration_states': [P, N, IP, N],
        'set_duration_states': [P, N, IP, N],
        'copy_output_states': [P, N, N, IP, N],
        'set_output_states': [P, N, N, IP, N],
        'outgoing_count': [P, N, c.POINTER(N)],
        'copy_outgoing': [P, N, N, c.POINTER(Jump), N],
        'set_outgoing': [P, N, c.POINTER(Jump), N],
        'add_jump': [P, N, I, scalar],
        'resegment': [P, c.POINTER(Occurrence), N, c.POINTER(P)],
        'clone': [P, c.POINTER(P)],
        'release': [c.POINTER(P)],
    }
    functions, called = {}, set()
    for name, arguments in signatures.items():
        function = getattr(lib, f'liblrhsmm_rs_segmentation_{name}_{suffix}')
        function.argtypes, function.restype = arguments, U
        functions[name] = function

    def status(name, expected, *arguments):
        actual = functions[name](*arguments)
        assert actual == expected, (suffix, name, actual, expected)
        called.add(name)

    def call(name, *arguments):
        status(name, 0, *arguments)

    def probability_bits(jump):
        return bit_type.from_address(c.addressof(jump) + Jump.probability.offset).value

    owner, clone, result = P(), P(), P()
    boundaries, durations = (I * 3)(2, 4, 8), (I * 3)(10, 11, 12)
    states, copied, replacement = (I * 3)(20, -2147483648, 2147483647), (I * 3)(), (I * 3)(3, 5, 9)
    rows = (IP * 2)(states, states)
    jumps, output = (Jump * 3)(), (Jump * 4)()
    for index, (delta, bits) in enumerate(zip([-2, 0, 1], patterns)):
        jumps[index].delta = delta
        bit_type.from_address(c.addressof(jumps[index]) + Jump.probability.offset).value = bits
    tail = c.cast(c.byref(jumps, 2 * c.sizeof(Jump)), c.POINTER(Jump))
    middle = c.cast(c.byref(jumps, c.sizeof(Jump)), c.POINTER(Jump))
    groups = (Group * 3)(Group(jumps, 3), Group(tail, 1), Group(middle, 2))
    call('new', 2, 3, c.byref(owner))
    call('copy_boundaries', owner, 0, copied, 3)
    assert list(copied) == [0, 0, 0]
    call('release', c.byref(owner))
    call('create', 2, 3, boundaries, durations, rows, groups, c.byref(owner))
    call('clone', owner, c.byref(clone))
    layout, count = Layout(99, 99), N(99)
    call('layout', owner, c.byref(layout))
    assert (layout.streams, layout.segments) == (2, 3)
    call('copy_duration_states', owner, 0, copied, 3)
    assert list(copied) == list(durations)
    call('copy_output_states', owner, 1, 0, copied, 3)
    assert list(copied) == list(states)
    call('outgoing_count', owner, 0, c.byref(count))
    assert count.value == 3
    call('copy_outgoing', owner, 0, 0, output, 3)
    assert [output[i].delta for i in range(3)] == [-2, 0, 1]
    assert [probability_bits(output[i]) for i in range(3)] == patterns
    for field in ['boundaries', 'duration_states']:
        call(f'set_{field}', owner, 0, replacement, 3)
    call('set_output_states', owner, 0, 0, replacement, 3)
    call('copy_output_states', owner, 1, 0, copied, 3)
    assert list(copied) == list(states)
    call('set_outgoing', owner, 1, jumps, 3)
    call('add_jump', owner, 1, 0, 0.25)
    call('copy_outgoing', owner, 1, 0, output, 4)
    assert output[2].delta == 0 and output[2].probability == 0.25 and output[3].delta == 1
    call('add_jump', owner, 1, 1, 0.5)
    call('copy_outgoing', owner, 1, 3, output, 1)
    assert output[0].probability == 0.5
    status('set_outgoing', 3, owner, 1, None, 0)
    call('outgoing_count', owner, 1, c.byref(count))
    assert count.value == 4
    copied[0] = 77
    status('copy_boundaries', 2, owner, 3, copied, 1)
    assert copied[0] == 77
    status('copy_boundaries', 1, owner, 0, None, 1)
    call('copy_boundaries', owner, 3, None, 0)
    status('set_output_states', 2, owner, 2, 0, replacement, 1)
    status('outgoing_count', 2, owner, 3, c.byref(count))
    assert count.value == 4
    malformed = (Group * 3)(Group(jumps, 3), Group(jumps, 3), Group(jumps, 2))
    status('replace', 3, owner, 2, 3, boundaries, durations, rows, malformed)
    call('copy_boundaries', owner, 0, copied, 3)
    assert list(copied) == list(replacement)
    occurrences = (Occurrence * 3)(Occurrence(2, 2), Occurrence(0, 2), Occurrence(2, 7))
    call('resegment', owner, occurrences, 3, c.byref(result))
    call('copy_duration_states', result, 0, copied, 3)
    assert list(copied) == [9, 3, 9]
    call('copy_boundaries', result, 0, copied, 3)
    assert list(copied) == [2, 2, 7]
    call('outgoing_count', result, 0, c.byref(count))
    assert count.value == 1
    call('release', c.byref(result))
    occurrences[0].state = 3
    result.value = clone.value
    status('resegment', 3, owner, occurrences, 3, c.byref(result))
    assert result.value == clone.value
    result.value = None
    call('replace', owner, 2, 3, boundaries, durations, rows, groups)
    states[0], boundaries[0], jumps[0].probability = 99, 99, 99
    call('copy_boundaries', owner, 0, copied, 3)
    assert copied[0] == 2
    call('release', c.byref(owner))
    assert owner.value is None
    call('copy_output_states', clone, 0, 0, copied, 3)
    assert copied[0] == 20
    call('copy_outgoing', clone, 0, 0, output, 3)
    assert [probability_bits(output[i]) for i in range(3)] == patterns
    call('release', c.byref(clone))
    empty_rows = (IP * 2)()
    call('create', 2, 0, None, None, empty_rows, None, c.byref(owner))
    call('replace', owner, 0, 0, None, None, None, None)
    call('layout', owner, c.byref(layout))
    assert (layout.streams, layout.segments) == (0, 0)
    call('resegment', owner, None, 0, c.byref(result))
    call('release', c.byref(result))
    call('release', c.byref(owner))
    call('release', c.byref(owner))
    status('release', 1, None)
    assert called == set(signatures), (suffix, set(signatures) - called)

print('Segmentation Python: all 34 exports passed')
