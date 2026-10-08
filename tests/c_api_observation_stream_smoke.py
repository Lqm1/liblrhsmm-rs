"""Actual ctypes calls for every observation-stream export, preserving scalar bits."""
import ctypes as c
import sys

lib = c.CDLL(sys.argv[1])
P, N, U = c.c_void_p, c.c_size_t, c.c_uint32
class Layout(c.Structure):
    _fields_ = [('frames', N), ('dimensions', N), ('values', N)]

for suffix, scalar, bit_type, patterns in [
    ('f32', c.c_float, c.c_uint32, [0x7fc01234, 0x80000000, 0x7f800000, 0x3fa00000, 0xff800000, 1]),
    ('f64', c.c_double, c.c_uint64, [0x7ff8000000001234, 0x8000000000000000, 0x7ff0000000000000, 0x3ff4000000000001, 0xfff0000000000000, 1]),
]:
    signatures = {
        'new': [N, N, c.POINTER(P)],
        'create': [N, c.POINTER(scalar), N, c.POINTER(P)],
        'replace': [P, N, c.POINTER(scalar), N],
        'layout': [P, c.POINTER(Layout)],
        'copy_values': [P, N, c.POINTER(scalar), N],
        'set_values': [P, N, c.POINTER(scalar), N],
        'copy_frame': [P, N, c.POINTER(scalar), N],
        'set_frame': [P, N, c.POINTER(scalar), N],
        'clone': [P, c.POINTER(P)],
        'release': [c.POINTER(P)],
    }
    functions, called = {}, set()
    for name, arguments in signatures.items():
        function = getattr(lib, f'liblrhsmm_rs_observation_stream_{name}_{suffix}')
        function.argtypes, function.restype = arguments, U
        functions[name] = function
    def status(name, expected, *arguments):
        actual = functions[name](*arguments)
        assert actual == expected, (name, actual, expected)
        called.add(name)
    def call(name, *arguments):
        status(name, 0, *arguments)
    owner, clone, zeroed = P(), P(), P()
    raw = (bit_type * 6)(*patterns)
    values, copied = (scalar * 6)(), (scalar * 6)()
    c.memmove(values, raw, c.sizeof(raw))
    call('new', 2, 3, c.byref(zeroed))
    call('copy_values', zeroed, 0, copied, 6)
    assert list(copied) == [0] * 6
    call('release', c.byref(zeroed))
    call('create', 3, values, 6, c.byref(owner))
    call('clone', owner, c.byref(clone))
    c.memset(values, 0, c.sizeof(values))
    layout = Layout(99, 99, 99)
    call('layout', owner, c.byref(layout))
    assert (layout.frames, layout.dimensions, layout.values) == (2, 3, 6)
    call('copy_values', owner, 0, copied, 6)
    assert bytes(copied) == bytes(raw)
    call('copy_frame', owner, 1, copied, 3)
    assert bytes(copied)[:c.sizeof(scalar) * 3] == bytes(raw)[c.sizeof(scalar) * 3:]
    row = (scalar * 3)(1, 2, 3)
    call('set_frame', owner, 1, row, 3)
    call('set_values', owner, 0, row, 3)
    call('copy_frame', owner, 0, copied, 3)
    assert list(copied)[:3] == [1, 2, 3]
    status('replace', 3, owner, 3, row, 2)
    status('set_values', 2, owner, 6, row, 1)
    status('copy_values', 1, owner, 0, None, 1)
    call('copy_values', owner, 6, None, 0)
    call('set_values', owner, 6, None, 0)
    status('copy_frame', 2, owner, 0, copied, 2)
    call('replace', owner, 1, row, 3)
    call('layout', owner, c.byref(layout))
    assert (layout.frames, layout.dimensions) == (3, 1)
    call('release', c.byref(owner))
    call('copy_values', clone, 0, copied, 6)
    assert bytes(copied) == bytes(raw)
    call('release', c.byref(clone))
    call('create', 3, None, 0, c.byref(owner))
    call('layout', owner, c.byref(layout))
    assert (layout.frames, layout.values) == (0, 0)
    call('release', c.byref(owner))
    assert not owner.value
    call('release', c.byref(owner))
    status('release', 1, None)
    assert called == set(signatures), set(signatures) - called
print('observation stream: all20 exports and exact scalar bits passed')
