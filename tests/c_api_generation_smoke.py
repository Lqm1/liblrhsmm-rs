"""Actual ctypes sampling and generation against unchanged original C fixtures."""
import ctypes as c
import math
from pathlib import Path
import struct
import sys

lib = c.CDLL(sys.argv[1])
P, N, U, I = c.c_void_p, c.c_size_t, c.c_uint32, c.c_int32
Next32 = c.CFUNCTYPE(U, P, c.POINTER(c.c_uint32))
Next64 = c.CFUNCTYPE(U, P, c.POINTER(c.c_uint64))


class RandomSource(c.Structure):
    _fields_ = [('context', P), ('next_u32', Next32), ('next_u64', Next64)]


class ObservationLayout(c.Structure):
    _fields_ = [('frames', N), ('streams', N)]


class StreamLayout(c.Structure):
    _fields_ = [('frames', N), ('dimensions', N), ('values', N)]


for suffix, scalar, code, fixtures in [
    ('f32', c.c_float, 'f', ['c-generation.txt']),
    ('f64', c.c_double, 'd', ['c-double-generation-32767.txt', 'c-double-generation-2147483647.txt']),
]:
    Draw = c.CFUNCTYPE(U, P, c.POINTER(scalar))

    class UniformSource(c.Structure):
        _fields_ = [('context', P), ('draw', Draw)]

    class Gaussian(c.Structure):
        _fields_ = [('mixtures', N), ('dimensions', N), ('weights', c.POINTER(scalar)), ('means', c.POINTER(scalar)), ('variances', c.POINTER(scalar)), ('floors', c.POINTER(scalar))]

    class Duration(c.Structure):
        _fields_ = [('mean', scalar), ('variance', scalar), ('minimum', I), ('maximum', I), ('fixed_mean', I), ('variance_floor', scalar)]

    class Jump(c.Structure):
        _fields_ = [('delta', I), ('probability', scalar)]

    class JumpSlice(c.Structure):
        _fields_ = [('jumps', c.POINTER(Jump)), ('count', N)]

    functions, tracked, called = {}, set(), set()

    def register(name, arguments, new=False):
        fn = getattr(lib, f'liblrhsmm_rs_{name}_{suffix}')
        fn.argtypes, fn.restype = arguments, U
        functions[name] = fn
        if new:
            tracked.add(name)

    def status(name, expected, *arguments):
        actual = functions[name](*arguments)
        assert actual == expected, (suffix, name, actual, expected)
        if name in tracked:
            called.add(name)

    def call(name, *arguments):
        status(name, 0, *arguments)

    def release(family, owner):
        call(f'{family}_release', c.byref(owner))
        assert owner.value is None

    register('uniform_sample', [c.POINTER(RandomSource), c.POINTER(scalar)], True)
    register('gaussian_sample', [scalar, scalar, scalar, scalar, c.POINTER(scalar)], True)
    register('choose_component', [c.POINTER(scalar), N, scalar, c.POINTER(N)], True)
    register('model_generate', [P, P, c.POINTER(RandomSource), c.POINTER(P)], True)
    register('model_generate_with_uniform', [P, P, c.POINTER(UniformSource), c.POINTER(P)], True)
    register('gaussian_create', [c.POINTER(Gaussian), c.POINTER(P)])
    register('stream_create', [scalar, c.POINTER(P), N, c.POINTER(P)])
    register('duration_default', [c.POINTER(Duration)])
    register('duration_create', [c.POINTER(Duration), c.POINTER(P)])
    register('model_create', [c.POINTER(P), N, c.POINTER(P), N, c.POINTER(P)])
    register('segmentation_create', [N, N, c.POINTER(I), c.POINTER(I), c.POINTER(c.POINTER(I)), c.POINTER(JumpSlice), c.POINTER(P)])
    register('segmentation_clone', [P, c.POINTER(P)])
    for operation in ['copy', 'set']:
        register(f'segmentation_{operation}_boundaries', [P, N, c.POINTER(I), N])
    register('observation_layout', [P, c.POINTER(ObservationLayout)])
    register('observation_get_stream', [P, N, c.POINTER(P)])
    register('observation_stream_layout', [P, c.POINTER(StreamLayout)])
    register('observation_stream_copy_values', [P, N, c.POINTER(scalar), N])
    for family in ['gaussian', 'stream', 'duration', 'model', 'segmentation', 'observation', 'observation_stream']:
        register(f'{family}_release', [c.POINTER(P)])

    def make(scenario):
        streams, durations = [], []
        for stream in range(2):
            components, dimensions = (2, 2) if stream == 0 else (1, 1)
            gaussians = []
            for state in range(2):
                weights = (scalar * components)(*([0.25, 0.75] if stream == 0 else [1]))
                means = (scalar * (components * dimensions))(*[state + index * 0.5 + stream * 2 for index in range(components * dimensions)])
                variances = (scalar * len(means))(*[0.25 + index * 0.25 if scenario == 0 else 0 for index in range(len(means))])
                floors = (scalar * len(means))()
                parameters = Gaussian(components, dimensions, weights, means, variances, floors)
                owner = P()
                call('gaussian_create', c.byref(parameters), c.byref(owner))
                gaussians.append(owner)
            owner = P()
            call('stream_create', 1, (P * 2)(*[g.value for g in gaussians]), 2, c.byref(owner))
            streams.append(owner)
            for child in gaussians:
                release('gaussian', child)
        for state in range(2):
            parameters, owner = Duration(), P()
            call('duration_default', c.byref(parameters))
            parameters.mean = 2.5 - state if scenario == 0 else 0
            parameters.variance = 0.75 - state * 0.5 if scenario == 0 else 0
            parameters.minimum = 9
            call('duration_create', c.byref(parameters), c.byref(owner))
            durations.append(owner)
        model, sequence = P(), P()
        call('model_create', (P * 2)(*[s.value for s in streams]), 2, (P * 2)(*[d.value for d in durations]), 2, c.byref(model))
        for child in streams:
            release('stream', child)
        for child in durations:
            release('duration', child)
        first, second = (I * 2)(0, 1), (I * 2)(1, 0)
        rows = (c.POINTER(I) * 2)(first, second)
        jump = Jump(1, 1)
        jumps = (JumpSlice * 2)(JumpSlice(c.pointer(jump), 1), JumpSlice(c.pointer(jump), 1))
        call('segmentation_create', 2, 2, (I * 2)(999, 999), (I * 2)(0, 1), rows, jumps, c.byref(sequence))
        return model, sequence

    def boundaries(sequence):
        values = (I * 2)()
        call('segmentation_copy_boundaries', sequence, 0, values, 2)
        return list(values)

    def read(owner):
        layout = ObservationLayout()
        call('observation_layout', owner, c.byref(layout))
        assert layout.streams == 2
        rows = [[] for _ in range(layout.frames)]
        for stream, dimensions in enumerate([2, 1]):
            child, shape = P(), StreamLayout()
            call('observation_get_stream', owner, stream, c.byref(child))
            call('observation_stream_layout', child, c.byref(shape))
            assert (shape.frames, shape.dimensions, shape.values) == (layout.frames, dimensions, layout.frames * dimensions)
            values = (scalar * shape.values)()
            call('observation_stream_copy_values', child, 0, values, len(values))
            for frame, row in enumerate(rows):
                row.extend(values[frame * dimensions:(frame + 1) * dimensions])
            release('observation_stream', child)
        return rows

    def bits(value):
        return struct.pack('<' + code, value)

    def check(actual, expected):
        if suffix == 'f32':
            assert bits(actual) == bits(expected), (actual, expected)
        else:
            assert math.isfinite(actual) and abs(actual - expected) / max(1, abs(expected)) <= 2e-14, (actual, expected)

    class Source:
        def __init__(self, maximum=2147483647, fail=None, index=0, converted=False):
            self.maximum, self.fail, self.index, self.converted = maximum, fail, index, converted
            self.word_callback = (Next32 if suffix == 'f32' else Next64)(self.word)
            self.draw_callback = Draw(self.draw)
            self.random = RandomSource(None, self.word_callback if suffix == 'f32' else Next32(), self.word_callback if suffix == 'f64' else Next64())
            self.uniform = UniformSource(None, self.draw_callback)

        def word(self, context, output):
            index = self.index
            self.index += 1
            if index == self.fail:
                return 77
            width = 32 if suffix == 'f32' else 64
            maximum = (1 << width) - 1
            output[0] = [0, maximum, 1 << (width - 1), 0x12345678 if width == 32 else 0x123456789abcdef0, 1, maximum - 1][index % 6]
            return 0

        def value(self):
            maximum = self.maximum
            values = [maximum // 8, maximum // 4, maximum // 2, maximum // 4 * 3, maximum // 8 * 7, maximum]
            value = scalar(scalar(values[self.index % 6]).value / scalar(maximum).value).value
            self.index += 1
            return value

        def draw(self, context, output):
            if self.converted:
                word = (c.c_uint32 if suffix == 'f32' else c.c_uint64)()
                result = self.word(context, c.pointer(word))
                if result:
                    return result
                shift, precision = (8, 24) if suffix == 'f32' else (11, 53)
                output[0] = ((word.value >> shift) + 1) * 2.0 ** -precision
                return 0
            if self.index == self.fail:
                self.index += 1
                return 99
            output[0] = self.value()
            return 0

    for fixture in fixtures:
        records = [line.split() for line in (Path(__file__).parent / 'fixtures' / fixture).read_text().splitlines()]
        maximum = next((int(r[1]) for r in records if r[0] == 'maximum'), 2147483647)
        for scenario in range(2):
            model, sequence = make(scenario)
            source, observation = Source(maximum), P()
            call('model_generate_with_uniform', model, sequence, c.byref(source.uniform), c.byref(observation))
            expected_bounds = next([int(v) for v in r[2:]] for r in records if r[:2] == ['boundaries', str(scenario)])
            assert boundaries(sequence) == expected_bounds
            assert source.index == next(int(r[2]) for r in records if r[:2] == ['draws', str(scenario)])
            values = read(observation)
            expected_rows = [r for r in records if r[:2] == ['frame', str(scenario)]]
            assert len(values) == len(expected_rows) == expected_bounds[-1]
            for row, expected in zip(values, expected_rows):
                for actual, reference in zip(row, expected[3:]):
                    check(actual, float(reference))
            if scenario == 0:
                for index in range(6):
                    draws = Source(maximum, index=index)
                    radial, angular, result, choice = draws.value(), draws.value(), scalar(), N()
                    call('gaussian_sample', -0.75, 1.25, radial, angular, c.byref(result))
                    check(result.value, float(next(r[2] for r in records if r[:2] == ['gaussian', str(index)])))
                    call('choose_component', (scalar * 3)(0.25, 0.5, 0.1), 3, radial, c.byref(choice))
                    assert choice.value == int(next(r[2] for r in records if r[:2] == ['choice', str(index)]))
            release('model', model)
            release('segmentation', sequence)
            assert read(observation) == values
            release('observation', observation)

    source, result = Source(), scalar()
    call('uniform_sample', c.byref(source.random), c.byref(result))
    assert result.value == 2.0 ** (-24 if suffix == 'f32' else -53)
    call('uniform_sample', c.byref(source.random), c.byref(result))
    assert result.value == 1
    source.fail, result.value = source.index, 99
    status('uniform_sample', 3, c.byref(source.random), c.byref(result))
    assert result.value == 99
    count = source.index
    status('uniform_sample', 1, c.byref(source.random), None)
    assert source.index == count
    model, sequence = make(0)
    duplicate, raw, uniform = P(), P(), P()
    call('segmentation_clone', sequence, c.byref(duplicate))
    words, converted = Source(), Source(converted=True)
    call('model_generate', model, sequence, c.byref(words.random), c.byref(raw))
    call('model_generate_with_uniform', model, duplicate, c.byref(converted.uniform), c.byref(uniform))
    assert words.index == converted.index and boundaries(sequence) == boundaries(duplicate)
    left, right = read(raw), read(uniform)
    assert len(left) == len(right)
    assert [[bits(v) for v in row] for row in left] == [[bits(v) for v in row] for row in right]
    release('observation', raw)
    release('observation', uniform)
    for failure in [0, 1, 4, 5, 10]:
        for name, descriptor in [('model_generate', 'random'), ('model_generate_with_uniform', 'uniform')]:
            call('segmentation_set_boundaries', sequence, 0, (I * 2)(999, 999), 2)
            source, sentinel = Source(fail=failure), P(0x1000)
            status(name, 3, model, sequence, c.byref(getattr(source, descriptor)), c.byref(sentinel))
            assert sentinel.value == 0x1000 and source.index == failure + 1
            assert boundaries(sequence) == [999, 999]
    release('model', model)
    release('segmentation', sequence)
    release('segmentation', duplicate)
    assert tracked == called and len(called) == 5

print('Generation ctypes: all10 exports, original C corpora and callback transactions passed')
