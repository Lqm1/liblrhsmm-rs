"""Actual ctypes paired regrouping, complete fields and independent ownership."""
import ctypes as c
import struct
import sys
from pathlib import Path

lib = c.CDLL(sys.argv[1])
P, N, U, I = c.c_void_p, c.c_size_t, c.c_uint32, c.c_int32
called = set()


class ObservationLayout(c.Structure):
    _fields_ = [('frames', N), ('streams', N)]


class StreamLayout(c.Structure):
    _fields_ = [('frames', N), ('dimensions', N), ('values', N)]


class SegmentationLayout(c.Structure):
    _fields_ = [('streams', N), ('segments', N)]


for suffix, scalar, code, epsilon in [('f32', c.c_float, 'f', 2**-23), ('f64', c.c_double, 'd', 2**-52)]:
    functions = {}

    def register(name, args):
        symbol = f'liblrhsmm_rs_{name}_{suffix}'
        fn = getattr(lib, symbol)
        fn.argtypes, fn.restype = args, U
        functions[name] = (symbol, fn)

    def status(name, expected, *args):
        symbol, fn = functions[name]
        actual = fn(*args)
        assert actual == expected, (symbol, actual, expected)
        if name == 'regroup_data' or name.startswith('dataset_'):
            called.add(symbol)

    def call(name, *args):
        status(name, 0, *args)

    def bits(value):
        return struct.pack('<' + code, value)

    class Jump(c.Structure):
        _fields_ = [('delta', I), ('probability', scalar)]

    class JumpSlice(c.Structure):
        _fields_ = [('jumps', c.POINTER(Jump)), ('count', N)]

    register('regroup_data', [P, P, N, c.POINTER(P)])
    register('dataset_length', [P, c.POINTER(N)])
    for method in ['get_observations', 'get_segmentations', 'clone']:
        register('dataset_' + method, [P, c.POINTER(P)])
    register('dataset_release', [c.POINTER(P)])
    register('observation_stream_create', [N, c.POINTER(scalar), N, c.POINTER(P)])
    register('observation_create', [N, c.POINTER(P), N, c.POINTER(P)])
    register('observation_layout', [P, c.POINTER(ObservationLayout)])
    register('observation_get_stream_layout', [P, N, c.POINTER(StreamLayout)])
    register('observation_copy_values', [P, N, N, c.POINTER(scalar), N])
    register('segmentation_create', [N, N, c.POINTER(I), c.POINTER(I), c.POINTER(c.POINTER(I)), c.POINTER(JumpSlice), c.POINTER(P)])
    register('segmentation_layout', [P, c.POINTER(SegmentationLayout)])
    register('segmentation_copy_boundaries', [P, N, c.POINTER(I), N])
    register('segmentation_set_boundaries', [P, N, c.POINTER(I), N])
    register('segmentation_copy_duration_states', [P, N, c.POINTER(I), N])
    register('segmentation_copy_output_states', [P, N, N, c.POINTER(I), N])
    register('segmentation_outgoing_count', [P, N, c.POINTER(N)])
    register('segmentation_copy_outgoing', [P, N, N, c.POINTER(Jump), N])
    for family in ['observation', 'segmentation']:
        register(f'{family}_set_create', [c.POINTER(P), N, c.POINTER(P)])
        register(f'{family}_set_length', [P, c.POINTER(N)])
        register(f'{family}_set_get', [P, N, c.POINTER(P)])
        register(f'{family}_set_set', [P, N, P])
        register(f'{family}_set_replace', [P, c.POINTER(P), N])
        register(f'{family}_set_release', [c.POINTER(P)])
        register(f'{family}_release', [c.POINTER(P)])
    register('observation_stream_release', [c.POINTER(P)])

    def sample(variant):
        frames, dimensions, boundaries = [(8, [2, 1], [1, 3, 3, 6, 8]), (3, [1], [0, 1, 3]), (0, [1], [])][variant]
        values = []
        streams = []
        for stream, dimension in enumerate(dimensions):
            if variant == 0:
                data = [(index // dimension) * 10 + stream * 100 + index % dimension for index in range(frames * dimension)] if suffix == 'f32' else [1 + (index + stream + 1) * epsilon for index in range(frames * dimension)]
            else:
                data = [scalar(scalar(2 + index / 8).value + epsilon).value for index in range(frames * dimension)]
            owner = P()
            call('observation_stream_create', dimension, (scalar * len(data))(*data), len(data), c.byref(owner))
            streams.append(owner)
            values.append(data)
        observation = P()
        call('observation_create', frames, (P * len(streams))(*[owner.value for owner in streams]), len(streams), c.byref(observation))
        for owner in streams:
            call('observation_stream_release', c.byref(owner))
        segments = len(boundaries)
        durations = list(range(10, 10 + segments))
        labels = [list(range(20, 20 + segments))] + ([list(range(40, 40 - segments, -1))] if len(dimensions) == 2 else [])
        groups = [[(1, 1.)] for _ in boundaries]
        if variant == 1:
            groups[0].insert(0, (2, scalar(.25 + epsilon).value))
        rows = [(I * segments)(*row) for row in labels]
        jumps = [(Jump * len(row))(*[Jump(delta, probability) for delta, probability in row]) for row in groups]
        slices = (JumpSlice * segments)(*[JumpSlice(row, len(row)) for row in jumps])
        segmentation = P()
        call('segmentation_create', len(dimensions), segments, (I * segments)(*boundaries), (I * segments)(*durations), (c.POINTER(I) * len(rows))(*rows), slices, c.byref(segmentation))
        return observation, segmentation, (frames, dimensions, values, boundaries, durations, labels)

    def partition(source, unit):
        frames, dimensions, values, boundaries, durations, labels = source
        result = []
        for first in range(0, len(boundaries), unit):
            end = min(first + unit, len(boundaries))
            begin_frame = boundaries[first - 1] if first else 0
            end_frame = boundaries[end - 1]
            result.append((end_frame - begin_frame, dimensions, [row[begin_frame * dimension:end_frame * dimension] for row, dimension in zip(values, dimensions)], [b - begin_frame for b in boundaries[first:end]], durations[first:end], [row[first:end] for row in labels]))
        return result

    def compare(observation, segmentation, expected):
        frames, dimensions, values, boundaries, durations, labels = expected
        shape = ObservationLayout()
        call('observation_layout', observation, c.byref(shape))
        assert (shape.frames, shape.streams) == (frames, len(dimensions))
        for stream, (dimension, row) in enumerate(zip(dimensions, values)):
            layout = StreamLayout()
            call('observation_get_stream_layout', observation, stream, c.byref(layout))
            assert (layout.frames, layout.dimensions, layout.values) == (frames, dimension, len(row))
            actual = (scalar * len(row))()
            call('observation_copy_values', observation, stream, 0, actual, len(row))
            assert [bits(v) for v in actual] == [bits(v) for v in row]
        shape = SegmentationLayout()
        call('segmentation_layout', segmentation, c.byref(shape))
        assert (shape.streams, shape.segments) == (len(dimensions), len(boundaries))
        integers = (I * len(boundaries))()
        call('segmentation_copy_boundaries', segmentation, 0, integers, len(integers))
        assert list(integers) == boundaries
        call('segmentation_copy_duration_states', segmentation, 0, integers, len(integers))
        assert list(integers) == durations
        for stream, row in enumerate(labels):
            call('segmentation_copy_output_states', segmentation, stream, 0, integers, len(integers))
            assert list(integers) == row
        for state in range(len(boundaries)):
            count = N()
            call('segmentation_outgoing_count', segmentation, state, c.byref(count))
            assert count.value == 1
            jump = Jump()
            call('segmentation_copy_outgoing', segmentation, state, 0, c.byref(jump), 1)
            assert jump.delta == 1 and bits(jump.probability) == bits(1.)

    samples = [sample(index) for index in range(3)]
    obs, seg, first = samples[0]
    observations, segmentations = P(), P()
    input_observations = (P * 4)(obs.value, samples[1][0].value, obs.value, samples[2][0].value)
    input_segmentations = (P * 4)(seg.value, samples[1][1].value, seg.value, samples[2][1].value)
    call('observation_set_create', input_observations, 4, c.byref(observations))
    call('segmentation_set_create', input_segmentations, 4, c.byref(segmentations))
    records = Path(__file__).with_name('fixtures').joinpath('corrected-c-double-datasets.txt').read_text().splitlines()
    assert len(records) == 150
    checked_records = 0
    for unit in [1, 2, 3, 5, 8, N(-1).value]:
        expected = partition(first, unit) + partition(samples[1][2], unit) + partition(first, unit)
        dataset, copied = P(), P()
        call('regroup_data', observations, segmentations, unit, c.byref(dataset))
        length = N()
        call('dataset_length', dataset, c.byref(length))
        assert length.value == len(expected)
        call('dataset_clone', dataset, c.byref(copied))
        call('dataset_release', c.byref(dataset))
        result_obs, result_seg = P(), P()
        call('dataset_get_observations', copied, c.byref(result_obs))
        call('dataset_get_segmentations', copied, c.byref(result_seg))
        call('dataset_release', c.byref(copied))
        if suffix == 'f64':
            for record in records:
                fields = record.split()
                if fields[0] == 'count' and int(fields[1]) == unit:
                    assert len(partition(first, unit)) == int(fields[2])
                    checked_records += 1
        for index, value in enumerate(expected):
            child_obs, child_seg = P(), P()
            call('observation_set_get', result_obs, index, c.byref(child_obs))
            call('segmentation_set_get', result_seg, index, c.byref(child_seg))
            compare(child_obs, child_seg, value)
            # Independent corrected-C records are checked against the actual getters.
            if suffix == 'f64' and index < len(partition(first, unit)):
                for record in records:
                    fields = record.split()
                    if int(fields[1]) != unit or int(fields[2]) != index:
                        continue
                    if fields[0] == 'value':
                        stream, element = map(int, fields[3:5])
                        actual = scalar()
                        call('observation_copy_values', child_obs, stream, element, c.byref(actual), 1)
                        assert bits(actual.value) == bits(float(fields[5]))
                        checked_records += 1
                    elif fields[0] == 'state':
                        state = int(fields[3])
                        actual = I()
                        values = []
                        for method in ['segmentation_copy_boundaries', 'segmentation_copy_duration_states']:
                            call(method, child_seg, state, c.byref(actual), 1)
                            values.append(actual.value)
                        for stream in [0, 1]:
                            call('segmentation_copy_output_states', child_seg, stream, state, c.byref(actual), 1)
                            values.append(actual.value)
                        assert values == list(map(int, fields[4:]))
                        checked_records += 1
            call('observation_release', c.byref(child_obs))
            call('segmentation_release', c.byref(child_seg))
        call('observation_set_release', c.byref(result_obs))
        call('segmentation_set_release', c.byref(result_seg))
    if suffix == 'f64':
        assert checked_records == 150
    sentinel = P(0x1000)
    status('regroup_data', 3, observations, segmentations, 0, c.byref(sentinel))
    assert sentinel.value == 0x1000
    status('regroup_data', 1, None, segmentations, 2, c.byref(sentinel))
    status('regroup_data', 1, observations, None, 2, c.byref(sentinel))
    status('regroup_data', 1, observations, segmentations, 2, None)
    dataset = P()
    call('regroup_data', observations, segmentations, 2, c.byref(dataset))
    for method in ['dataset_length', 'dataset_clone', 'dataset_get_observations', 'dataset_get_segmentations']:
        status(method, 1, dataset, None)
    status('dataset_clone', 1, None, c.byref(sentinel))
    assert sentinel.value == 0x1000
    call('segmentation_set_replace', segmentations, None, 0)
    status('regroup_data', 3, observations, segmentations, 2, c.byref(sentinel))
    assert sentinel.value == 0x1000
    call('observation_set_release', c.byref(observations))
    call('segmentation_set_release', c.byref(segmentations))
    for observation, segmentation, _ in samples:
        call('observation_release', c.byref(observation))
        call('segmentation_release', c.byref(segmentation))
    retained_obs, retained_seg = P(), P()
    call('dataset_get_observations', dataset, c.byref(retained_obs))
    call('dataset_get_segmentations', dataset, c.byref(retained_seg))
    call('dataset_release', c.byref(dataset))
    assert dataset.value is None
    call('dataset_release', c.byref(dataset))
    status('dataset_release', 1, None)
    child_obs, child_seg = P(), P()
    call('observation_set_get', retained_obs, 0, c.byref(child_obs))
    call('segmentation_set_get', retained_seg, 0, c.byref(child_seg))
    call('observation_set_release', c.byref(retained_obs))
    call('segmentation_set_release', c.byref(retained_seg))
    compare(child_obs, child_seg, partition(first, 2)[0])
    call('observation_release', c.byref(child_obs))
    call('segmentation_release', c.byref(child_seg))
    call('observation_set_create', None, 0, c.byref(observations))
    call('segmentation_set_create', None, 0, c.byref(segmentations))
    call('regroup_data', observations, segmentations, 1, c.byref(dataset))
    length = N(99)
    call('dataset_length', dataset, c.byref(length))
    assert length.value == 0
    call('dataset_release', c.byref(dataset))
    call('observation_set_release', c.byref(observations))
    call('segmentation_set_release', c.byref(segmentations))

assert len(called) == 12
print('Dataset ctypes: all12 exports, complete fields, corrected-C double records and independent lifetimes passed')
