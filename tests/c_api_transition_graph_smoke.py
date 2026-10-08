"""Exercise every cached graph export and the unchanged binary32 C corpus."""
import ctypes as c
from pathlib import Path
import sys

lib = c.CDLL(sys.argv[1])
P, N, U, I = c.c_void_p, c.c_size_t, c.c_uint32, c.c_int32


class Layout(c.Structure):
    _fields_ = [('streams', N), ('segments', N)]


fixture = (Path(__file__).parent / 'fixtures/c-transitions.txt').read_text().splitlines()
assert len(fixture) == 16
for suffix, scalar in [('f32', c.c_float), ('f64', c.c_double)]:
    class Jump(c.Structure):
        _fields_ = [('delta', I), ('probability', scalar)]

    signatures = {
        'build': [P, c.POINTER(P)],
        'layout': [P, c.POINTER(Layout)],
        'get_segmentation': [P, c.POINTER(P)],
        'incoming_count': [P, N, c.POINTER(N)],
        'outgoing_count': [P, N, c.POINTER(N)],
        'copy_incoming': [P, N, N, c.POINTER(Jump), N],
        'copy_outgoing': [P, N, N, c.POINTER(Jump), N],
        'clone': [P, c.POINTER(P)],
        'release': [c.POINTER(P)],
    }
    functions, called = {}, set()
    for name, arguments in signatures.items():
        symbol = (f'liblrhsmm_rs_segmentation_build_transitions_{suffix}' if name == 'build'
                  else f'liblrhsmm_rs_transition_graph_{name}_{suffix}')
        function = getattr(lib, symbol)
        function.argtypes, function.restype = arguments, U
        functions[name] = function

    def status(name, expected, *arguments):
        actual = functions[name](*arguments)
        assert actual == expected, (suffix, name, actual, expected)
        called.add(name)

    def call(name, *arguments):
        status(name, 0, *arguments)

    def segmentation(name, arguments, *values):
        function = getattr(lib, f'liblrhsmm_rs_segmentation_{name}_{suffix}')
        function.argtypes, function.restype = arguments, U
        assert function(*values) == 0, (suffix, name)

    def probability_bytes(jump):
        return c.string_at(c.addressof(jump) + Jump.probability.offset, c.sizeof(scalar))

    def check(graph):
        layout = Layout(99, 99)
        call('layout', graph, c.byref(layout))
        assert (layout.streams, layout.segments) == (2, 4)
        for line in fixture:
            _, segment, index, delta, probability = line.split()
            segment, index, delta = int(segment), int(index), int(delta)
            actual, count = Jump(), N(99)
            call('incoming_count', graph, segment, c.byref(count))
            assert count.value == [2, 3, 2, 1][segment]
            call('copy_incoming', graph, segment, index, c.byref(actual), 1)
            # Original corpus is binary32; f64 comparison is exact widening.
            expected = scalar(c.c_float(float(probability)).value)
            assert actual.delta == delta and probability_bytes(actual) == bytes(expected)
            call('copy_incoming', graph, segment, count.value, None, 0)
        count, actual = N(99), (Jump * 3)()
        call('outgoing_count', graph, 0, c.byref(count))
        assert count.value == 3
        call('copy_outgoing', graph, 0, 0, actual, 3)
        assert [jump.delta for jump in actual] == [2, -100, 1]
        for jump, probability in zip(actual, [0.2, 0.1, 0.6]):
            assert probability_bytes(jump) == bytes(scalar(c.c_float(probability).value))

    source, child, graph, clone = P(), P(), P(), P()
    segmentation('new', [N, N, c.POINTER(P)], 2, 4, c.byref(source))
    boundaries, durations = (I * 4)(-2147483648, 3, 2, 2147483647), (I * 4)(2147483647, -1, 7, -2147483648)
    rows = [(I * 4)(2, -3, 5, -2147483648), (I * 4)(2147483647, 8, 1, 9)]
    for name, values in [('boundaries', boundaries), ('duration_states', durations)]:
        segmentation(f'set_{name}', [P, N, c.POINTER(I), N], source, 0, values, 4)
    for stream, row in enumerate(rows):
        segmentation('set_output_states', [P, N, N, c.POINTER(I), N], source, stream, 0, row, 4)
    for state, delta, weight in [(0, 2, 0.2), (1, 0, 0.3), (2, -2, 0.4), (3, -2, 0.5), (0, -100, 0.1), (3, 100, 0.1)]:
        segmentation('add_jump', [P, N, I, scalar], source, state, delta, c.c_float(weight).value)
    for state in range(4):
        product = c.c_float(state * c.c_float(0.1).value).value
        weight = c.c_float(c.c_float(0.6).value + product).value
        segmentation('add_jump', [P, N, I, scalar], source, state, 1, weight)
    call('build', source, c.byref(graph))
    check(graph)
    call('clone', graph, c.byref(clone))
    call('get_segmentation', graph, c.byref(child))
    segmentation('add_jump', [P, N, I, scalar], source, 0, 0, 0.25)
    segmentation('release', [c.POINTER(P)], c.byref(source))
    copied = (I * 4)()
    for name, expected in [('boundaries', boundaries), ('duration_states', durations)]:
        segmentation(f'copy_{name}', [P, N, c.POINTER(I), N], child, 0, copied, 4)
        assert list(copied) == list(expected)
    for stream, row in enumerate(rows):
        segmentation('copy_output_states', [P, N, N, c.POINTER(I), N], child, stream, 0, copied, 4)
        assert list(copied) == list(row)
    segmentation('add_jump', [P, N, I, scalar], child, 0, 0, 0.75)
    check(graph)
    check(clone)
    count = N(77)
    for name in ['incoming_count', 'outgoing_count']:
        status(name, 2, graph, 4, c.byref(count))
        assert count.value == 77
    untouched = Jump(77, -0.0)
    status('copy_incoming', 2, graph, 0, N(-1).value, c.byref(untouched), 1)
    assert untouched.delta == 77 and probability_bytes(untouched) == bytes(scalar(-0.0))
    status('copy_outgoing', 2, graph, 0, 3, c.byref(untouched), 1)
    assert untouched.delta == 77
    status('copy_incoming', 1, graph, 0, 0, None, 1)
    call('copy_outgoing', graph, 0, 3, None, 0)
    invalid = (Jump * 1)(Jump(1, float('nan')))
    segmentation('set_outgoing', [P, N, c.POINTER(Jump), N], child, 3, invalid, 1)
    sentinel = P(graph.value)
    status('build', 3, child, c.byref(sentinel))
    assert sentinel.value == graph.value
    status('clone', 1, None, c.byref(sentinel))
    assert sentinel.value == graph.value
    status('get_segmentation', 1, graph, None)
    call('release', c.byref(graph))
    assert graph.value is None
    check(clone)
    segmentation('release', [c.POINTER(P)], c.byref(child))
    call('release', c.byref(clone))
    call('release', c.byref(clone))
    status('release', 1, None)
    for streams in [0, 2]:
        segmentation('new', [N, N, c.POINTER(P)], streams, 0, c.byref(source))
        call('build', source, c.byref(graph))
        layout = Layout(99, 99)
        call('layout', graph, c.byref(layout))
        assert (layout.streams, layout.segments) == (streams, 0)
        status('incoming_count', 2, graph, 0, c.byref(count))
        segmentation('release', [c.POINTER(P)], c.byref(source))
        call('release', c.byref(graph))
    assert called == set(signatures), set(signatures) - called

print('Transition graph Python: all18 exports and original C caches passed')
