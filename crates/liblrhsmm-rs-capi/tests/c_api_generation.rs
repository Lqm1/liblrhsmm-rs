use liblrhsmm_rs::data::{ObservationData, SegmentationData};
use liblrhsmm_rs::model::{ModelParameters, StreamLayout};
use liblrhsmm_rs::scalar::Scalar;
use liblrhsmm_rs_capi::*;
use rand::{Rng, SeedableRng, rngs::SmallRng};
use std::ffi::c_void;
use std::ptr::{null, null_mut};

fn sample<T: Scalar>(scenario: usize) -> (ModelParameters<T>, SegmentationData<T>) {
    let mut model = ModelParameters::new(
        &[
            StreamLayout {
                emissions: 2,
                components: 2,
                dimensions: 2,
            },
            StreamLayout {
                emissions: 2,
                components: 1,
                dimensions: 1,
            },
        ],
        2,
    )
    .unwrap();
    for (stream, parameters) in model.streams.iter_mut().enumerate() {
        for (state, gaussian) in parameters.mixtures.iter_mut().enumerate() {
            if stream == 0 {
                gaussian.weights = vec![T::from_f64(0.25), T::from_f64(0.75)];
            }
            for index in 0..gaussian.means.len() {
                gaussian.means[index] =
                    T::from_f64(state as f64 + index as f64 * 0.5 + stream as f64 * 2.0);
                gaussian.variances[index] = T::from_f64(if scenario == 0 {
                    0.25 + index as f64 * 0.25
                } else {
                    0.0
                });
            }
        }
    }
    for (state, duration) in model.durations.iter_mut().enumerate() {
        duration.mean = T::from_f64(if scenario == 0 {
            2.5 - state as f64
        } else {
            0.0
        });
        duration.variance = T::from_f64(if scenario == 0 {
            0.75 - state as f64 * 0.5
        } else {
            0.0
        });
        duration.minimum = 9;
    }
    let mut sequence = SegmentationData::new(2, 2);
    sequence.duration_states = vec![0, 1];
    sequence.output_states = vec![vec![0, 1], vec![1, 0]];
    (model, sequence)
}
struct Words {
    rng: SmallRng,
    calls: usize,
    fail_at: Option<usize>,
    fixed: Option<u64>,
}
impl Words {
    fn new() -> Self {
        Self {
            rng: SmallRng::seed_from_u64(42),
            calls: 0,
            fail_at: None,
            fixed: None,
        }
    }
}
unsafe extern "C" fn next32(context: *mut c_void, output: *mut u32) -> u32 {
    unsafe {
        let source = &mut *context.cast::<Words>();
        let index = source.calls;
        source.calls += 1;
        if source.fail_at == Some(index) {
            return 77;
        }
        output.write(
            source
                .fixed
                .map_or_else(|| source.rng.next_u32(), |word| word as u32),
        );
        0
    }
}
unsafe extern "C" fn next64(context: *mut c_void, output: *mut u64) -> u32 {
    unsafe {
        let source = &mut *context.cast::<Words>();
        let index = source.calls;
        source.calls += 1;
        if source.fail_at == Some(index) {
            return 77;
        }
        output.write(source.fixed.unwrap_or_else(|| source.rng.next_u64()));
        0
    }
}
struct Draws {
    calls: usize,
    maximum: i32,
    fail_at: Option<usize>,
}
impl Draws {
    fn new(maximum: i32) -> Self {
        Self {
            calls: 0,
            maximum,
            fail_at: None,
        }
    }
}

unsafe fn import_f32(
    model: &ModelParameters<f32>,
    segmentation: &SegmentationData<f32>,
) -> (*mut LiblrhsmmRsModelF32, *mut LiblrhsmmRsSegmentationF32) {
    unsafe {
        let mut streams = Vec::new();
        for native in &model.streams {
            let mut emissions = Vec::new();
            for mixture in &native.mixtures {
                let descriptor = LiblrhsmmRsGaussianParametersF32 {
                    mixtures: mixture.weights.len(),
                    dimensions: mixture.dimensions,
                    weights: mixture.weights.as_ptr(),
                    means: mixture.means.as_ptr(),
                    variances: mixture.variances.as_ptr(),
                    variance_floors: mixture.variance_floors.as_ptr(),
                };
                let mut handle = null_mut();
                assert_eq!(
                    liblrhsmm_rs_gaussian_create_f32(&descriptor, &mut handle),
                    0
                );
                emissions.push(handle);
            }
            let borrowed: Vec<_> = emissions.iter().map(|p| p.cast_const()).collect();
            let mut stream = null_mut();
            assert_eq!(
                liblrhsmm_rs_stream_create_f32(
                    native.weight,
                    borrowed.as_ptr(),
                    borrowed.len(),
                    &mut stream
                ),
                0
            );
            for handle in &mut emissions {
                assert_eq!(liblrhsmm_rs_gaussian_release_f32(handle), 0);
            }
            streams.push(stream);
        }
        let mut durations = Vec::new();
        for native in &model.durations {
            let descriptor = LiblrhsmmRsDurationParametersF32::from(native);
            let mut handle = null_mut();
            assert_eq!(
                liblrhsmm_rs_duration_create_f32(&descriptor, &mut handle),
                0
            );
            durations.push(handle);
        }
        let borrowed_streams: Vec<_> = streams.iter().map(|p| p.cast_const()).collect();
        let borrowed_durations: Vec<_> = durations.iter().map(|p| p.cast_const()).collect();
        let mut model_handle = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_create_f32(
                borrowed_streams.as_ptr(),
                streams.len(),
                borrowed_durations.as_ptr(),
                durations.len(),
                &mut model_handle
            ),
            0
        );
        for handle in &mut streams {
            assert_eq!(liblrhsmm_rs_stream_release_f32(handle), 0);
        }
        for handle in &mut durations {
            assert_eq!(liblrhsmm_rs_duration_release_f32(handle), 0);
        }
        let states: Vec<_> = segmentation
            .output_states
            .iter()
            .map(|row| row.as_ptr())
            .collect();
        let jumps: Vec<Vec<_>> = segmentation
            .outgoing
            .iter()
            .map(|row| {
                row.iter()
                    .map(|v| LiblrhsmmRsJumpF32 {
                        delta: v.delta,
                        probability: v.probability,
                    })
                    .collect()
            })
            .collect();
        let groups: Vec<_> = jumps
            .iter()
            .map(|row| LiblrhsmmRsJumpSliceF32 {
                jumps: row.as_ptr(),
                count: row.len(),
            })
            .collect();
        let mut seg = null_mut();
        assert_eq!(
            liblrhsmm_rs_segmentation_create_f32(
                2,
                2,
                segmentation.boundaries.as_ptr(),
                segmentation.duration_states.as_ptr(),
                states.as_ptr(),
                groups.as_ptr(),
                &mut seg
            ),
            0
        );
        (model_handle, seg)
    }
}

fn draw_f32(index: usize, maximum: i32) -> f32 {
    let values = [
        maximum / 8,
        maximum / 4,
        maximum / 2,
        maximum / 4 * 3,
        maximum / 8 * 7,
        maximum,
    ];
    values[index % 6] as f32 / maximum as f32
}
unsafe extern "C" fn draw_callback_f32(context: *mut c_void, output: *mut f32) -> u32 {
    unsafe {
        let source = &mut *context.cast::<Draws>();
        let index = source.calls;
        source.calls += 1;
        if source.fail_at == Some(index) {
            return 99;
        }
        output.write(draw_f32(index, source.maximum));
        0
    }
}
unsafe fn assert_sequence_f32(
    owner: *const LiblrhsmmRsSegmentationF32,
    expected: &SegmentationData<f32>,
) {
    unsafe {
        let mut values = vec![0; expected.boundaries.len()];
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_boundaries_f32(
                owner,
                0,
                values.as_mut_ptr(),
                values.len()
            ),
            0
        );
        assert_eq!(values, expected.boundaries);
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_duration_states_f32(
                owner,
                0,
                values.as_mut_ptr(),
                values.len()
            ),
            0
        );
        assert_eq!(values, expected.duration_states);
        for (stream, states) in expected.output_states.iter().enumerate() {
            assert_eq!(
                liblrhsmm_rs_segmentation_copy_output_states_f32(
                    owner,
                    stream,
                    0,
                    values.as_mut_ptr(),
                    values.len()
                ),
                0
            );
            assert_eq!(&values, states);
        }
        for (state, jumps) in expected.outgoing.iter().enumerate() {
            let mut count = 0;
            assert_eq!(
                liblrhsmm_rs_segmentation_outgoing_count_f32(owner, state, &mut count),
                0
            );
            assert_eq!(count, jumps.len());
            let mut actual = vec![
                LiblrhsmmRsJumpF32 {
                    delta: 0,
                    probability: 0.0
                };
                count
            ];
            assert_eq!(
                liblrhsmm_rs_segmentation_copy_outgoing_f32(
                    owner,
                    state,
                    0,
                    actual.as_mut_ptr(),
                    count
                ),
                0
            );
            for (a, e) in actual.iter().zip(jumps) {
                assert_eq!(a.delta, e.delta);
                assert_eq!(a.probability.to_bits(), e.probability.to_bits());
            }
        }
    }
}
unsafe fn read_observation_f32(owner: *const LiblrhsmmRsObservationF32) -> ObservationData<f32> {
    unsafe {
        let mut shape = LiblrhsmmRsObservationLayout {
            frames: 0,
            streams: 0,
        };
        assert_eq!(liblrhsmm_rs_observation_layout_f32(owner, &mut shape), 0);
        let mut streams = Vec::new();
        for stream in 0..shape.streams {
            let mut child = null_mut();
            assert_eq!(
                liblrhsmm_rs_observation_get_stream_f32(owner, stream, &mut child),
                0
            );
            let mut layout = LiblrhsmmRsObservationStreamLayout {
                frames: 0,
                dimensions: 0,
                values: 0,
            };
            assert_eq!(
                liblrhsmm_rs_observation_stream_layout_f32(child, &mut layout),
                0
            );
            assert_eq!(layout.frames, shape.frames);
            let mut values = vec![0.0; layout.values];
            assert_eq!(
                liblrhsmm_rs_observation_stream_copy_values_f32(
                    child,
                    0,
                    values.as_mut_ptr(),
                    values.len()
                ),
                0
            );
            streams.push(liblrhsmm_rs::data::ObservationStreamData {
                dimensions: layout.dimensions,
                values,
            });
            assert_eq!(liblrhsmm_rs_observation_stream_release_f32(&mut child), 0);
        }
        ObservationData {
            frames: shape.frames,
            streams,
        }
    }
}
fn assert_observation_f32(actual: &ObservationData<f32>, expected: &ObservationData<f32>) {
    assert_eq!(
        (actual.frames, actual.streams.len()),
        (expected.frames, expected.streams.len())
    );
    for (a, e) in actual.streams.iter().zip(&expected.streams) {
        assert_eq!(a.dimensions, e.dimensions);
        assert_eq!(
            a.values.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            e.values.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
        );
    }
}
#[test]
fn f32_scalar_sampling_and_selected_random_words_preserve_native_bits() {
    unsafe {
        let mut words = Words::new();
        let source = LiblrhsmmRsRandomSource {
            context: (&mut words as *mut Words).cast(),
            next_u32: Some(next32),
            next_u64: None,
        };
        let mut native = SmallRng::seed_from_u64(42);
        for _ in 0..32 {
            let mut value = 0.0_f32;
            assert_eq!(liblrhsmm_rs_uniform_sample_f32(&source, &mut value), 0);
            assert_eq!(
                value.to_bits(),
                liblrhsmm_rs::generation::uniform_with_precision::<f32, _>(&mut native).to_bits()
            );
        }
        assert_eq!(words.calls, 32);
        for (word, expected) in [(0, f32::EPSILON * 0.5), (u64::MAX, 1.0)] {
            words.fixed = Some(word);
            let mut value = 0.0;
            assert_eq!(liblrhsmm_rs_uniform_sample_f32(&source, &mut value), 0);
            assert_eq!(value, expected);
        }
        words.fail_at = Some(words.calls);
        let mut retained = 99.0;
        assert_eq!(liblrhsmm_rs_uniform_sample_f32(&source, &mut retained), 3);
        assert_eq!(retained, 99.0);
        let before = words.calls;
        assert_eq!(liblrhsmm_rs_uniform_sample_f32(&source, null_mut()), 1);
        assert_eq!(words.calls, before);
        let absent = LiblrhsmmRsRandomSource {
            context: source.context,
            next_u32: None,
            next_u64: None,
        };
        assert_eq!(liblrhsmm_rs_uniform_sample_f32(&absent, &mut retained), 1);
        assert_eq!(words.calls, before);
        assert_eq!(liblrhsmm_rs_uniform_sample_f32(null(), &mut retained), 1);
        for index in 0..6 {
            let radial = draw_f32(index, i32::MAX);
            let angular = draw_f32(index + 1, i32::MAX);
            let mut actual = 0.0_f32;
            assert_eq!(
                liblrhsmm_rs_gaussian_sample_f32(-0.75, 1.25, radial, angular, &mut actual),
                0
            );
            assert_eq!(
                actual.to_bits(),
                liblrhsmm_rs::generation::gaussian_with_precision(-0.75_f32, 1.25, radial, angular)
                    .unwrap()
                    .to_bits()
            );
            let mut component = 99;
            let weights = [0.25, 0.5, 0.1];
            assert_eq!(
                liblrhsmm_rs_choose_component_f32(weights.as_ptr(), 3, radial, &mut component),
                0
            );
            assert_eq!(
                component,
                liblrhsmm_rs::generation::choose_with_precision(&weights, radial).unwrap()
            );
        }
        let beyond = 1.0_f32 + f32::EPSILON;
        assert_eq!(
            liblrhsmm_rs_gaussian_sample_f32(beyond, 0.0, 0.5, 0.5, &mut retained),
            0
        );
        assert_eq!(retained.to_bits(), beyond.to_bits());
        let weights = [0.5 + f32::EPSILON, 0.5];
        let mut component = 99;
        assert_eq!(
            liblrhsmm_rs_choose_component_f32(weights.as_ptr(), 2, 0.5, &mut component),
            0
        );
        assert_eq!(component, 0);
        component = 99;
        assert_eq!(
            liblrhsmm_rs_choose_component_f32(null(), 0, 0.5, &mut component),
            3
        );
        assert_eq!(component, 99);
        assert_eq!(
            liblrhsmm_rs_choose_component_f32(null(), 1, 0.5, &mut component),
            1
        );
        retained = 99.0;
        assert_eq!(
            liblrhsmm_rs_gaussian_sample_f32(0.0, 1.0, 0.0, 0.5, &mut retained),
            3
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_sample_f32(0.0, -1.0, 0.5, 0.5, &mut retained),
            3
        );
        assert_eq!(retained, 99.0);
    }
}
#[test]
fn f32_uniform_generation_complete_native_and_original_c_corpora() {
    let fixtures = [include_str!("../../../tests/fixtures/c-generation.txt")];
    for fixture in fixtures {
        let maximum = fixture
            .lines()
            .find_map(|line| line.strip_prefix("maximum "))
            .map_or(i32::MAX, |value| value.parse().unwrap());
        for scenario in 0..2 {
            let (model, mut expected_sequence) = sample::<f32>(scenario);
            let original = expected_sequence.clone();
            let mut draws = 0;
            let expected = model
                .generate_with_uniform(&mut expected_sequence, || {
                    let value = draw_f32(draws, maximum);
                    draws += 1;
                    value
                })
                .unwrap();
            unsafe {
                let (mut owner, mut sequence) = import_f32(&model, &original);
                let mut source = Draws::new(maximum);
                let descriptor = LiblrhsmmRsUniformSourceF32 {
                    context: (&mut source as *mut Draws).cast(),
                    draw: Some(draw_callback_f32),
                };
                let mut observation = null_mut();
                assert_eq!(
                    liblrhsmm_rs_model_generate_with_uniform_f32(
                        owner,
                        sequence,
                        &descriptor,
                        &mut observation
                    ),
                    0
                );
                assert_eq!(source.calls, draws);
                assert_sequence_f32(sequence, &expected_sequence);
                let actual = read_observation_f32(observation);
                assert_observation_f32(&actual, &expected);
                for line in fixture.lines() {
                    let fields: Vec<_> = line.split_whitespace().collect();
                    if fields[0] == "maximum" {
                        continue;
                    }
                    let index: usize = fields[1].parse().unwrap();
                    let (values, first) = match fields[0] {
                        "gaussian" if scenario == 0 => {
                            let mut value = 0.0;
                            assert_eq!(
                                liblrhsmm_rs_gaussian_sample_f32(
                                    -0.75,
                                    1.25,
                                    draw_f32(index, maximum),
                                    draw_f32(index + 1, maximum),
                                    &mut value
                                ),
                                0
                            );
                            (vec![value], 2)
                        }
                        "choice" if scenario == 0 => {
                            let weights = [0.25, 0.5, 0.1];
                            let mut choice = 99;
                            assert_eq!(
                                liblrhsmm_rs_choose_component_f32(
                                    weights.as_ptr(),
                                    3,
                                    draw_f32(index, maximum),
                                    &mut choice
                                ),
                                0
                            );
                            assert_eq!(choice, fields[2].parse::<usize>().unwrap());
                            continue;
                        }
                        "boundaries" if index == scenario => {
                            assert_eq!(
                                expected_sequence.boundaries,
                                fields[2..]
                                    .iter()
                                    .map(|v| v.parse::<i32>().unwrap())
                                    .collect::<Vec<_>>()
                            );
                            continue;
                        }
                        "draws" if index == scenario => {
                            assert_eq!(source.calls, fields[2].parse::<usize>().unwrap());
                            continue;
                        }
                        "frame" if index == scenario => {
                            let time = fields[2].parse::<usize>().unwrap();
                            (
                                (0..2)
                                    .flat_map(|stream| {
                                        actual.frame(stream, time).unwrap().iter().copied()
                                    })
                                    .collect(),
                                3,
                            )
                        }
                        _ => continue,
                    };
                    let reference: Vec<f32> =
                        fields[first..].iter().map(|v| v.parse().unwrap()).collect();
                    assert_eq!(values.len(), reference.len());
                    for (a, e) in values.into_iter().zip(reference) {
                        assert_eq!(a.to_bits(), e.to_bits());
                    }
                }
                assert_eq!(liblrhsmm_rs_model_release_f32(&mut owner), 0);
                assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut sequence), 0);
                assert_observation_f32(&read_observation_f32(observation), &expected);
                assert_eq!(liblrhsmm_rs_observation_release_f32(&mut observation), 0);
            }
        }
    }
}
#[test]
fn f32_rng_generation_and_callback_failures_are_atomic() {
    let (model, original) = sample::<f32>(0);
    let mut expected_sequence = original.clone();
    let expected = model
        .generate(&mut expected_sequence, &mut SmallRng::seed_from_u64(42))
        .unwrap();
    unsafe {
        let (mut owner, mut sequence) = import_f32(&model, &original);
        let mut source = Words::new();
        let descriptor = LiblrhsmmRsRandomSource {
            context: (&mut source as *mut Words).cast(),
            next_u32: Some(next32),
            next_u64: None,
        };
        let mut observation = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_generate_f32(owner, sequence, &descriptor, &mut observation),
            0
        );
        assert_eq!(source.calls, 4 + 8 * expected.frames);
        assert_sequence_f32(sequence, &expected_sequence);
        assert_observation_f32(&read_observation_f32(observation), &expected);
        assert_eq!(liblrhsmm_rs_observation_release_f32(&mut observation), 0);
        assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut sequence), 0);
        for failure in [0, 1, 4, 5, 10] {
            let mut before = original.clone();
            before.boundaries.fill(999);
            let (mut unused_model, mut input) = import_f32(&model, &before);
            let mut source = Words::new();
            source.fail_at = Some(failure);
            let descriptor = LiblrhsmmRsRandomSource {
                context: (&mut source as *mut Words).cast(),
                next_u32: Some(next32),
                next_u64: None,
            };
            let sentinel = std::ptr::without_provenance_mut(0x1000);
            let mut retained = sentinel;
            assert_eq!(
                liblrhsmm_rs_model_generate_f32(owner, input, &descriptor, &mut retained),
                3
            );
            assert_eq!(retained, sentinel);
            assert_eq!(source.calls, failure + 1);
            assert_sequence_f32(input, &before);
            let mut draws = Draws::new(i32::MAX);
            draws.fail_at = Some(failure);
            let callback = LiblrhsmmRsUniformSourceF32 {
                context: (&mut draws as *mut Draws).cast(),
                draw: Some(draw_callback_f32),
            };
            assert_eq!(
                liblrhsmm_rs_model_generate_with_uniform_f32(
                    owner,
                    input,
                    &callback,
                    &mut retained
                ),
                3
            );
            assert_eq!(retained, sentinel);
            assert_eq!(draws.calls, failure + 1);
            assert_sequence_f32(input, &before);
            assert_eq!(
                liblrhsmm_rs_model_generate_with_uniform_f32(owner, input, &callback, null_mut()),
                1
            );
            assert_eq!(draws.calls, failure + 1);
            assert_eq!(
                liblrhsmm_rs_model_generate_f32(owner, input, &descriptor, null_mut()),
                1
            );
            assert_eq!(source.calls, failure + 1);
            assert_eq!(liblrhsmm_rs_model_release_f32(&mut unused_model), 0);
            assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut input), 0);
        }
        let mut invalid = model.clone();
        invalid.streams[1].mixtures[0].variances[0] = -1.0;
        let (mut bad, mut sequence) = import_f32(&invalid, &original);
        let mut source = Draws::new(i32::MAX);
        let callback = LiblrhsmmRsUniformSourceF32 {
            context: (&mut source as *mut Draws).cast(),
            draw: Some(draw_callback_f32),
        };
        let mut output = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_generate_with_uniform_f32(bad, sequence, &callback, &mut output),
            3
        );
        assert_eq!(source.calls, 0);
        assert!(output.is_null());
        assert_sequence_f32(sequence, &original);
        assert_eq!(liblrhsmm_rs_model_release_f32(&mut bad), 0);
        assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut sequence), 0);
        assert_eq!(liblrhsmm_rs_model_release_f32(&mut owner), 0);
    }
}

unsafe fn import_f64(
    model: &ModelParameters<f64>,
    segmentation: &SegmentationData<f64>,
) -> (*mut LiblrhsmmRsModelF64, *mut LiblrhsmmRsSegmentationF64) {
    unsafe {
        let mut streams = Vec::new();
        for native in &model.streams {
            let mut emissions = Vec::new();
            for mixture in &native.mixtures {
                let descriptor = LiblrhsmmRsGaussianParametersF64 {
                    mixtures: mixture.weights.len(),
                    dimensions: mixture.dimensions,
                    weights: mixture.weights.as_ptr(),
                    means: mixture.means.as_ptr(),
                    variances: mixture.variances.as_ptr(),
                    variance_floors: mixture.variance_floors.as_ptr(),
                };
                let mut handle = null_mut();
                assert_eq!(
                    liblrhsmm_rs_gaussian_create_f64(&descriptor, &mut handle),
                    0
                );
                emissions.push(handle);
            }
            let borrowed: Vec<_> = emissions.iter().map(|p| p.cast_const()).collect();
            let mut stream = null_mut();
            assert_eq!(
                liblrhsmm_rs_stream_create_f64(
                    native.weight,
                    borrowed.as_ptr(),
                    borrowed.len(),
                    &mut stream
                ),
                0
            );
            for handle in &mut emissions {
                assert_eq!(liblrhsmm_rs_gaussian_release_f64(handle), 0);
            }
            streams.push(stream);
        }
        let mut durations = Vec::new();
        for native in &model.durations {
            let descriptor = LiblrhsmmRsDurationParametersF64::from(native);
            let mut handle = null_mut();
            assert_eq!(
                liblrhsmm_rs_duration_create_f64(&descriptor, &mut handle),
                0
            );
            durations.push(handle);
        }
        let borrowed_streams: Vec<_> = streams.iter().map(|p| p.cast_const()).collect();
        let borrowed_durations: Vec<_> = durations.iter().map(|p| p.cast_const()).collect();
        let mut model_handle = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_create_f64(
                borrowed_streams.as_ptr(),
                streams.len(),
                borrowed_durations.as_ptr(),
                durations.len(),
                &mut model_handle
            ),
            0
        );
        for handle in &mut streams {
            assert_eq!(liblrhsmm_rs_stream_release_f64(handle), 0);
        }
        for handle in &mut durations {
            assert_eq!(liblrhsmm_rs_duration_release_f64(handle), 0);
        }
        let states: Vec<_> = segmentation
            .output_states
            .iter()
            .map(|row| row.as_ptr())
            .collect();
        let jumps: Vec<Vec<_>> = segmentation
            .outgoing
            .iter()
            .map(|row| {
                row.iter()
                    .map(|v| LiblrhsmmRsJumpF64 {
                        delta: v.delta,
                        probability: v.probability,
                    })
                    .collect()
            })
            .collect();
        let groups: Vec<_> = jumps
            .iter()
            .map(|row| LiblrhsmmRsJumpSliceF64 {
                jumps: row.as_ptr(),
                count: row.len(),
            })
            .collect();
        let mut seg = null_mut();
        assert_eq!(
            liblrhsmm_rs_segmentation_create_f64(
                2,
                2,
                segmentation.boundaries.as_ptr(),
                segmentation.duration_states.as_ptr(),
                states.as_ptr(),
                groups.as_ptr(),
                &mut seg
            ),
            0
        );
        (model_handle, seg)
    }
}

fn draw_f64(index: usize, maximum: i32) -> f64 {
    let values = [
        maximum / 8,
        maximum / 4,
        maximum / 2,
        maximum / 4 * 3,
        maximum / 8 * 7,
        maximum,
    ];
    values[index % 6] as f64 / maximum as f64
}
unsafe extern "C" fn draw_callback_f64(context: *mut c_void, output: *mut f64) -> u32 {
    unsafe {
        let source = &mut *context.cast::<Draws>();
        let index = source.calls;
        source.calls += 1;
        if source.fail_at == Some(index) {
            return 99;
        }
        output.write(draw_f64(index, source.maximum));
        0
    }
}
unsafe fn assert_sequence_f64(
    owner: *const LiblrhsmmRsSegmentationF64,
    expected: &SegmentationData<f64>,
) {
    unsafe {
        let mut values = vec![0; expected.boundaries.len()];
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_boundaries_f64(
                owner,
                0,
                values.as_mut_ptr(),
                values.len()
            ),
            0
        );
        assert_eq!(values, expected.boundaries);
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_duration_states_f64(
                owner,
                0,
                values.as_mut_ptr(),
                values.len()
            ),
            0
        );
        assert_eq!(values, expected.duration_states);
        for (stream, states) in expected.output_states.iter().enumerate() {
            assert_eq!(
                liblrhsmm_rs_segmentation_copy_output_states_f64(
                    owner,
                    stream,
                    0,
                    values.as_mut_ptr(),
                    values.len()
                ),
                0
            );
            assert_eq!(&values, states);
        }
        for (state, jumps) in expected.outgoing.iter().enumerate() {
            let mut count = 0;
            assert_eq!(
                liblrhsmm_rs_segmentation_outgoing_count_f64(owner, state, &mut count),
                0
            );
            assert_eq!(count, jumps.len());
            let mut actual = vec![
                LiblrhsmmRsJumpF64 {
                    delta: 0,
                    probability: 0.0
                };
                count
            ];
            assert_eq!(
                liblrhsmm_rs_segmentation_copy_outgoing_f64(
                    owner,
                    state,
                    0,
                    actual.as_mut_ptr(),
                    count
                ),
                0
            );
            for (a, e) in actual.iter().zip(jumps) {
                assert_eq!(a.delta, e.delta);
                assert_eq!(a.probability.to_bits(), e.probability.to_bits());
            }
        }
    }
}
unsafe fn read_observation_f64(owner: *const LiblrhsmmRsObservationF64) -> ObservationData<f64> {
    unsafe {
        let mut shape = LiblrhsmmRsObservationLayout {
            frames: 0,
            streams: 0,
        };
        assert_eq!(liblrhsmm_rs_observation_layout_f64(owner, &mut shape), 0);
        let mut streams = Vec::new();
        for stream in 0..shape.streams {
            let mut child = null_mut();
            assert_eq!(
                liblrhsmm_rs_observation_get_stream_f64(owner, stream, &mut child),
                0
            );
            let mut layout = LiblrhsmmRsObservationStreamLayout {
                frames: 0,
                dimensions: 0,
                values: 0,
            };
            assert_eq!(
                liblrhsmm_rs_observation_stream_layout_f64(child, &mut layout),
                0
            );
            assert_eq!(layout.frames, shape.frames);
            let mut values = vec![0.0; layout.values];
            assert_eq!(
                liblrhsmm_rs_observation_stream_copy_values_f64(
                    child,
                    0,
                    values.as_mut_ptr(),
                    values.len()
                ),
                0
            );
            streams.push(liblrhsmm_rs::data::ObservationStreamData {
                dimensions: layout.dimensions,
                values,
            });
            assert_eq!(liblrhsmm_rs_observation_stream_release_f64(&mut child), 0);
        }
        ObservationData {
            frames: shape.frames,
            streams,
        }
    }
}
fn assert_observation_f64(actual: &ObservationData<f64>, expected: &ObservationData<f64>) {
    assert_eq!(
        (actual.frames, actual.streams.len()),
        (expected.frames, expected.streams.len())
    );
    for (a, e) in actual.streams.iter().zip(&expected.streams) {
        assert_eq!(a.dimensions, e.dimensions);
        assert_eq!(
            a.values.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            e.values.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
        );
    }
}
#[test]
fn f64_scalar_sampling_and_selected_random_words_preserve_native_bits() {
    unsafe {
        let mut words = Words::new();
        let source = LiblrhsmmRsRandomSource {
            context: (&mut words as *mut Words).cast(),
            next_u32: None,
            next_u64: Some(next64),
        };
        let mut native = SmallRng::seed_from_u64(42);
        for _ in 0..32 {
            let mut value = 0.0_f64;
            assert_eq!(liblrhsmm_rs_uniform_sample_f64(&source, &mut value), 0);
            assert_eq!(
                value.to_bits(),
                liblrhsmm_rs::generation::uniform_with_precision::<f64, _>(&mut native).to_bits()
            );
        }
        assert_eq!(words.calls, 32);
        for (word, expected) in [(0, f64::EPSILON * 0.5), (u64::MAX, 1.0)] {
            words.fixed = Some(word);
            let mut value = 0.0;
            assert_eq!(liblrhsmm_rs_uniform_sample_f64(&source, &mut value), 0);
            assert_eq!(value, expected);
        }
        words.fail_at = Some(words.calls);
        let mut retained = 99.0;
        assert_eq!(liblrhsmm_rs_uniform_sample_f64(&source, &mut retained), 3);
        assert_eq!(retained, 99.0);
        let before = words.calls;
        assert_eq!(liblrhsmm_rs_uniform_sample_f64(&source, null_mut()), 1);
        assert_eq!(words.calls, before);
        let absent = LiblrhsmmRsRandomSource {
            context: source.context,
            next_u32: None,
            next_u64: None,
        };
        assert_eq!(liblrhsmm_rs_uniform_sample_f64(&absent, &mut retained), 1);
        assert_eq!(words.calls, before);
        assert_eq!(liblrhsmm_rs_uniform_sample_f64(null(), &mut retained), 1);
        for index in 0..6 {
            let radial = draw_f64(index, i32::MAX);
            let angular = draw_f64(index + 1, i32::MAX);
            let mut actual = 0.0_f64;
            assert_eq!(
                liblrhsmm_rs_gaussian_sample_f64(-0.75, 1.25, radial, angular, &mut actual),
                0
            );
            assert_eq!(
                actual.to_bits(),
                liblrhsmm_rs::generation::gaussian_with_precision(-0.75_f64, 1.25, radial, angular)
                    .unwrap()
                    .to_bits()
            );
            let mut component = 99;
            let weights = [0.25, 0.5, 0.1];
            assert_eq!(
                liblrhsmm_rs_choose_component_f64(weights.as_ptr(), 3, radial, &mut component),
                0
            );
            assert_eq!(
                component,
                liblrhsmm_rs::generation::choose_with_precision(&weights, radial).unwrap()
            );
        }
        let beyond = 1.0_f64 + f64::EPSILON;
        assert_eq!(
            liblrhsmm_rs_gaussian_sample_f64(beyond, 0.0, 0.5, 0.5, &mut retained),
            0
        );
        assert_eq!(retained.to_bits(), beyond.to_bits());
        let weights = [0.5 + f64::EPSILON, 0.5];
        let mut component = 99;
        assert_eq!(
            liblrhsmm_rs_choose_component_f64(weights.as_ptr(), 2, 0.5, &mut component),
            0
        );
        assert_eq!(component, 0);
        component = 99;
        assert_eq!(
            liblrhsmm_rs_choose_component_f64(null(), 0, 0.5, &mut component),
            3
        );
        assert_eq!(component, 99);
        assert_eq!(
            liblrhsmm_rs_choose_component_f64(null(), 1, 0.5, &mut component),
            1
        );
        retained = 99.0;
        assert_eq!(
            liblrhsmm_rs_gaussian_sample_f64(0.0, 1.0, 0.0, 0.5, &mut retained),
            3
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_sample_f64(0.0, -1.0, 0.5, 0.5, &mut retained),
            3
        );
        assert_eq!(retained, 99.0);
    }
}
#[test]
fn f64_uniform_generation_complete_native_and_original_c_corpora() {
    let fixtures = [
        include_str!("../../../tests/fixtures/c-double-generation-32767.txt"),
        include_str!("../../../tests/fixtures/c-double-generation-2147483647.txt"),
    ];
    for fixture in fixtures {
        let maximum = fixture
            .lines()
            .find_map(|line| line.strip_prefix("maximum "))
            .map_or(i32::MAX, |value| value.parse().unwrap());
        for scenario in 0..2 {
            let (model, mut expected_sequence) = sample::<f64>(scenario);
            let original = expected_sequence.clone();
            let mut draws = 0;
            let expected = model
                .generate_with_uniform(&mut expected_sequence, || {
                    let value = draw_f64(draws, maximum);
                    draws += 1;
                    value
                })
                .unwrap();
            unsafe {
                let (mut owner, mut sequence) = import_f64(&model, &original);
                let mut source = Draws::new(maximum);
                let descriptor = LiblrhsmmRsUniformSourceF64 {
                    context: (&mut source as *mut Draws).cast(),
                    draw: Some(draw_callback_f64),
                };
                let mut observation = null_mut();
                assert_eq!(
                    liblrhsmm_rs_model_generate_with_uniform_f64(
                        owner,
                        sequence,
                        &descriptor,
                        &mut observation
                    ),
                    0
                );
                assert_eq!(source.calls, draws);
                assert_sequence_f64(sequence, &expected_sequence);
                let actual = read_observation_f64(observation);
                assert_observation_f64(&actual, &expected);
                for line in fixture.lines() {
                    let fields: Vec<_> = line.split_whitespace().collect();
                    if fields[0] == "maximum" {
                        continue;
                    }
                    let index: usize = fields[1].parse().unwrap();
                    let (values, first) = match fields[0] {
                        "gaussian" if scenario == 0 => {
                            let mut value = 0.0;
                            assert_eq!(
                                liblrhsmm_rs_gaussian_sample_f64(
                                    -0.75,
                                    1.25,
                                    draw_f64(index, maximum),
                                    draw_f64(index + 1, maximum),
                                    &mut value
                                ),
                                0
                            );
                            (vec![value], 2)
                        }
                        "choice" if scenario == 0 => {
                            let weights = [0.25, 0.5, 0.1];
                            let mut choice = 99;
                            assert_eq!(
                                liblrhsmm_rs_choose_component_f64(
                                    weights.as_ptr(),
                                    3,
                                    draw_f64(index, maximum),
                                    &mut choice
                                ),
                                0
                            );
                            assert_eq!(choice, fields[2].parse::<usize>().unwrap());
                            continue;
                        }
                        "boundaries" if index == scenario => {
                            assert_eq!(
                                expected_sequence.boundaries,
                                fields[2..]
                                    .iter()
                                    .map(|v| v.parse::<i32>().unwrap())
                                    .collect::<Vec<_>>()
                            );
                            continue;
                        }
                        "draws" if index == scenario => {
                            assert_eq!(source.calls, fields[2].parse::<usize>().unwrap());
                            continue;
                        }
                        "frame" if index == scenario => {
                            let time = fields[2].parse::<usize>().unwrap();
                            (
                                (0..2)
                                    .flat_map(|stream| {
                                        actual.frame(stream, time).unwrap().iter().copied()
                                    })
                                    .collect(),
                                3,
                            )
                        }
                        _ => continue,
                    };
                    let reference: Vec<f64> =
                        fields[first..].iter().map(|v| v.parse().unwrap()).collect();
                    assert_eq!(values.len(), reference.len());
                    for (a, e) in values.into_iter().zip(reference) {
                        assert!((a - e).abs() / e.abs().max(1.0) <= 2e-14);
                    }
                }
                assert_eq!(liblrhsmm_rs_model_release_f64(&mut owner), 0);
                assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut sequence), 0);
                assert_observation_f64(&read_observation_f64(observation), &expected);
                assert_eq!(liblrhsmm_rs_observation_release_f64(&mut observation), 0);
            }
        }
    }
}
#[test]
fn f64_rng_generation_and_callback_failures_are_atomic() {
    let (model, original) = sample::<f64>(0);
    let mut expected_sequence = original.clone();
    let expected = model
        .generate(&mut expected_sequence, &mut SmallRng::seed_from_u64(42))
        .unwrap();
    unsafe {
        let (mut owner, mut sequence) = import_f64(&model, &original);
        let mut source = Words::new();
        let descriptor = LiblrhsmmRsRandomSource {
            context: (&mut source as *mut Words).cast(),
            next_u32: None,
            next_u64: Some(next64),
        };
        let mut observation = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_generate_f64(owner, sequence, &descriptor, &mut observation),
            0
        );
        assert_eq!(source.calls, 4 + 8 * expected.frames);
        assert_sequence_f64(sequence, &expected_sequence);
        assert_observation_f64(&read_observation_f64(observation), &expected);
        assert_eq!(liblrhsmm_rs_observation_release_f64(&mut observation), 0);
        assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut sequence), 0);
        for failure in [0, 1, 4, 5, 10] {
            let mut before = original.clone();
            before.boundaries.fill(999);
            let (mut unused_model, mut input) = import_f64(&model, &before);
            let mut source = Words::new();
            source.fail_at = Some(failure);
            let descriptor = LiblrhsmmRsRandomSource {
                context: (&mut source as *mut Words).cast(),
                next_u32: None,
                next_u64: Some(next64),
            };
            let sentinel = std::ptr::without_provenance_mut(0x1000);
            let mut retained = sentinel;
            assert_eq!(
                liblrhsmm_rs_model_generate_f64(owner, input, &descriptor, &mut retained),
                3
            );
            assert_eq!(retained, sentinel);
            assert_eq!(source.calls, failure + 1);
            assert_sequence_f64(input, &before);
            let mut draws = Draws::new(i32::MAX);
            draws.fail_at = Some(failure);
            let callback = LiblrhsmmRsUniformSourceF64 {
                context: (&mut draws as *mut Draws).cast(),
                draw: Some(draw_callback_f64),
            };
            assert_eq!(
                liblrhsmm_rs_model_generate_with_uniform_f64(
                    owner,
                    input,
                    &callback,
                    &mut retained
                ),
                3
            );
            assert_eq!(retained, sentinel);
            assert_eq!(draws.calls, failure + 1);
            assert_sequence_f64(input, &before);
            assert_eq!(
                liblrhsmm_rs_model_generate_with_uniform_f64(owner, input, &callback, null_mut()),
                1
            );
            assert_eq!(draws.calls, failure + 1);
            assert_eq!(
                liblrhsmm_rs_model_generate_f64(owner, input, &descriptor, null_mut()),
                1
            );
            assert_eq!(source.calls, failure + 1);
            assert_eq!(liblrhsmm_rs_model_release_f64(&mut unused_model), 0);
            assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut input), 0);
        }
        let mut invalid = model.clone();
        invalid.streams[1].mixtures[0].variances[0] = -1.0;
        let (mut bad, mut sequence) = import_f64(&invalid, &original);
        let mut source = Draws::new(i32::MAX);
        let callback = LiblrhsmmRsUniformSourceF64 {
            context: (&mut source as *mut Draws).cast(),
            draw: Some(draw_callback_f64),
        };
        let mut output = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_generate_with_uniform_f64(bad, sequence, &callback, &mut output),
            3
        );
        assert_eq!(source.calls, 0);
        assert!(output.is_null());
        assert_sequence_f64(sequence, &original);
        assert_eq!(liblrhsmm_rs_model_release_f64(&mut bad), 0);
        assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut sequence), 0);
        assert_eq!(liblrhsmm_rs_model_release_f64(&mut owner), 0);
    }
}
