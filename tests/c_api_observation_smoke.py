"""Actual ctypes calls for every complete-observation export in both precisions."""
import ctypes as c
import sys

lib = c.CDLL(sys.argv[1])
P, N, U = c.c_void_p, c.c_size_t, c.c_uint32
class Layout(c.Structure):
    _fields_ = [('frames', N), ('streams', N)]
class StreamLayout(c.Structure):
    _fields_ = [('frames', N), ('dimensions', N), ('values', N)]

for suffix, scalar, bit_type, patterns in [
    ('f32', c.c_float, c.c_uint32, [0x7fc01234, 0x80000000, 0x7f800000, 0x3fa00000, 0xff800000, 1]),
    ('f64', c.c_double, c.c_uint64, [0x7ff8000000001234, 0x8000000000000000, 0x7ff0000000000000, 0x3ff4000000000001, 0xfff0000000000000, 1]),
]:
    signatures = {
        'new': [N, c.POINTER(N), N, c.POINTER(P)],
        'create': [N, c.POINTER(P), N, c.POINTER(P)],
        'replace': [P, N, c.POINTER(P), N],
        'layout': [P, c.POINTER(Layout)],
        'get_stream_layout': [P, N, c.POINTER(StreamLayout)],
        'get_stream': [P, N, c.POINTER(P)],
        'set_stream': [P, N, P],
        'copy_values': [P, N, N, c.POINTER(scalar), N],
        'set_values': [P, N, N, c.POINTER(scalar), N],
        'copy_frame': [P, N, N, c.POINTER(scalar), N],
        'set_frame': [P, N, N, c.POINTER(scalar), N],
        'clone': [P, c.POINTER(P)],
        'release': [c.POINTER(P)],
    }
    functions, called = {}, set()
    for name, arguments in signatures.items():
        function = getattr(lib, f'liblrhsmm_rs_observation_{name}_{suffix}')
        function.argtypes, function.restype = arguments, U
        functions[name] = function
    def status(name, expected, *arguments):
        actual = functions[name](*arguments)
        assert actual == expected, (name, actual, expected)
        called.add(name)
    def call(name, *arguments):
        status(name, 0, *arguments)
    def stream(name, arguments, *values):
        function = getattr(lib, f'liblrhsmm_rs_observation_stream_{name}_{suffix}')
        function.argtypes, function.restype = arguments, U
        assert function(*values) == 0
    first, second, child, owner, clone, zeroed = [P() for _ in range(6)]
    raw, values, copied = (bit_type * 6)(*patterns), (scalar * 6)(), (scalar * 6)()
    c.memmove(values, raw, c.sizeof(raw))
    row, dimensions = (scalar * 3)(4, -0.0, 9), (N * 2)(2, 3)
    call('new', 2, dimensions, 2, c.byref(zeroed))
    call('copy_values', zeroed, 1, 0, copied, 6)
    assert list(copied) == [0] * 6
    call('release', c.byref(zeroed))
    stream('create', [N, c.POINTER(scalar), N, c.POINTER(P)], 2, values, 4, c.byref(first))
    stream('create', [N, c.POINTER(scalar), N, c.POINTER(P)], 3, values, 6, c.byref(second))
    inputs = (P * 2)(first.value, second.value)
    call('create', 2, inputs, 2, c.byref(owner))
    call('clone', owner, c.byref(clone))
    layout, child_layout = Layout(99, 99), StreamLayout(99, 99, 99)
    call('layout', owner, c.byref(layout))
    assert (layout.frames, layout.streams) == (2, 2)
    call('get_stream_layout', owner, 1, c.byref(child_layout))
    assert (child_layout.frames, child_layout.dimensions, child_layout.values) == (2, 3, 6)
    call('get_stream', owner, 1, c.byref(child))
    call('copy_values', owner, 1, 0, copied, 6)
    assert bytes(copied) == bytes(raw)
    call('copy_frame', owner, 1, 1, copied, 3)
    assert bytes(copied)[:3 * c.sizeof(scalar)] == bytes(raw)[3 * c.sizeof(scalar):]
    call('set_frame', owner, 1, 1, row, 3)
    call('set_values', owner, 1, 0, row, 3)
    call('set_stream', owner, 0, second)
    call('get_stream_layout', owner, 0, c.byref(child_layout))
    assert child_layout.dimensions == 3
    stream('replace', [P, N, c.POINTER(scalar), N], first, 1, row, 3)
    status('set_stream', 3, owner, 1, first)
    status('replace', 3, owner, 2, inputs, 2)
    call('copy_frame', owner, 1, 0, copied, 3)
    assert bytes(copied)[:c.sizeof(row)] == bytes(row)
    status('set_values', 2, owner, 1, 6, row, 1)
    status('copy_frame', 2, owner, 1, 2, copied, 3)
    status('copy_frame', 2, owner, 1, 0, copied, 2)
    status('copy_values', 1, owner, 1, 0, None, 1)
    call('copy_values', owner, 1, 6, None, 0)
    repeated = (P * 2)(second.value, second.value)
    call('replace', owner, 2, repeated, 2)
    call('release', c.byref(owner))
    stream('release', [c.POINTER(P)], c.byref(first))
    stream('release', [c.POINTER(P)], c.byref(second))
    call('copy_values', clone, 1, 0, copied, 6)
    assert bytes(copied) == bytes(raw)
    call('release', c.byref(clone))
    stream('copy_values', [P, N, c.POINTER(scalar), N], child, 0, copied, 6)
    assert bytes(copied) == bytes(raw)
    stream('release', [c.POINTER(P)], c.byref(child))
    call('new', 0, dimensions, 2, c.byref(owner))
    call('copy_values', owner, 0, 0, None, 0)
    status('copy_frame', 2, owner, 0, 0, copied, 2)
    call('release', c.byref(owner))
    call('release', c.byref(owner))
    assert owner.value is None and called == set(signatures)
print('Complete observation C/Python ownership, all fields/bits, frames and errors verified')
