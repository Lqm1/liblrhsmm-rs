use liblrhsmm_rs::data::{ObservationData, SegmentationData};
use liblrhsmm_rs::hsmm::HsmmSettings;
use liblrhsmm_rs::model::{
    DurationParameters, GaussianMixtureParameters, ModelParameters, StreamParameters,
};
use liblrhsmm_rs::serial::{self, DecodeLimits, ModelEncoding};
use liblrhsmm_rs::statistics::ModelSufficientStatistics;
use liblrhsmm_rs::update::UpdateSettings;

#[path = "common/estimation.rs"]
#[allow(dead_code)]
mod common;

#[test]
fn double_readers_and_writers_preserve_all_existing_c_schemas() {
    macro_rules! check {
        ($ty:ty, $file:literal) => {{
            let bytes = include_bytes!($file);
            let value = <$ty>::read_from(bytes.as_slice()).unwrap();
            let mut output = Vec::new();
            value.write_to(&mut output).unwrap();
            assert_eq!(output, bytes);
            for end in [0, bytes.len() / 2, bytes.len() - 1] {
                assert!(<$ty>::read_from(&bytes[..end]).is_err());
            }
        }};
    }
    check!(
        DurationParameters<f64>,
        "fixtures/c-standalone-duration.bin"
    );
    check!(
        GaussianMixtureParameters<f64>,
        "fixtures/c-standalone-gmm.bin"
    );
    check!(StreamParameters<f64>, "fixtures/c-standalone-stream.bin");
    check!(ModelParameters<f64>, "fixtures/current-c-model.hsmm");
    check!(ObservationData<f64>, "fixtures/c-observation.bin");
    check!(SegmentationData<f64>, "fixtures/c-segmentation.bin");
    let bytes = include_bytes!("fixtures/cmu-arctic-all-speakers.hsmm");
    let model = ModelParameters::<f64>::read_from(bytes.as_slice()).unwrap();
    let mut output = Vec::new();
    model
        .write_with_encoding(&mut output, ModelEncoding::WithoutVarianceFloors)
        .unwrap();
    assert_eq!(output, bytes);
    assert!(
        ModelParameters::<f64>::read_with_limits(
            bytes.as_slice(),
            DecodeLimits {
                max_array_entries: 3
            }
        )
        .is_err()
    );

    let bytes = include_bytes!("fixtures/c-observation-set.bin");
    let samples = serial::read_observation_set_with_precision::<f64>(bytes.as_slice()).unwrap();
    output.clear();
    serial::write_observation_set_with_precision(&samples, &mut output).unwrap();
    assert_eq!(output, bytes);
    let bytes = include_bytes!("fixtures/c-segmentation-set.bin");
    let samples = serial::read_segmentation_set_with_precision::<f64>(bytes.as_slice()).unwrap();
    output.clear();
    serial::write_segmentation_set_with_precision(&samples, &mut output).unwrap();
    assert_eq!(output, bytes);
    assert_eq!(samples[0].outgoing[0][0].probability, 0.25);
}

#[test]
fn double_wire_rounding_preserves_binary32_and_discards_only_extra_precision() {
    let duration = DurationParameters::<f64> {
        mean: 1.0 + 2.0_f64.powi(-25),
        variance: 1.0 + 3.0 * 2.0_f64.powi(-25),
        variance_floor: -0.0,
        ..Default::default()
    };
    let mut duration_bytes = Vec::new();
    duration.write_to(&mut duration_bytes).unwrap();
    assert_eq!(
        duration_bytes,
        include_bytes!("fixtures/c-double-wire-duration.bin")
    );
    let original = [
        1.0 + 2.0_f64.powi(-25),
        1.0 + 3.0 * 2.0_f64.powi(-25),
        -0.0,
        f64::from(f32::from_bits(1)),
        f64::MAX,
    ];
    let mut observation = ObservationData::<f64>::new(original.len(), &[1]).unwrap();
    observation.streams[0].values.copy_from_slice(&original);
    let mut bytes = Vec::new();
    observation.write_to(&mut bytes).unwrap();
    let double = ObservationData::<f64>::read_from(bytes.as_slice()).unwrap();
    let float = ObservationData::<f32>::read_from(bytes.as_slice()).unwrap();
    for (index, &source) in original.iter().enumerate() {
        let expected = source as f32;
        assert_eq!(float.streams[0].values[index].to_bits(), expected.to_bits());
        assert_eq!(
            double.streams[0].values[index].to_bits(),
            f64::from(expected).to_bits()
        );
    }
    assert_eq!(double.streams[0].values[0], 1.0);
    assert_eq!(double.streams[0].values[1], 1.0 + 2.0_f64.powi(-23));
    assert!(double.streams[0].values[4].is_infinite());
}

#[test]
fn binary32_codec_keeps_nonfinite_payloads_and_signed_zero_bits() {
    let bits = [
        0x8000_0000,
        1,
        0x7f80_0000,
        0xff80_0000,
        0x7fc1_2345,
        0x7f81_2345,
    ];
    let mut observation = ObservationData::<f32>::new(bits.len(), &[1]).unwrap();
    observation.streams[0].values = bits.into_iter().map(f32::from_bits).collect();
    let mut bytes = Vec::new();
    observation.write_to(&mut bytes).unwrap();
    let restored = ObservationData::<f32>::read_from(bytes.as_slice()).unwrap();
    assert_eq!(
        restored.streams[0]
            .values
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>(),
        bits
    );
}

#[test]
fn double_training_saves_reloads_and_infers_using_legacy_wire_precision() {
    let (mut model, observation, segmentation) = common::sample_with_precision::<f64>();
    for _ in 0..2 {
        let mut statistics = ModelSufficientStatistics::from_model(&model).unwrap();
        assert!(
            statistics
                .estimate_hsmm(&model, &observation, &segmentation, HsmmSettings::default())
                .unwrap()
                .is_finite()
        );
        model
            .update(&statistics, UpdateSettings::default())
            .unwrap();
        let mut bytes = Vec::new();
        model.write_to(&mut bytes).unwrap();
        model = ModelParameters::<f64>::read_from(bytes.as_slice()).unwrap();
        // The same saved model remains readable through the original float API.
        let float = ModelParameters::<f32>::read_from(bytes.as_slice()).unwrap();
        let mut float_bytes = Vec::new();
        float.write_to(&mut float_bytes).unwrap();
        assert_eq!(bytes, float_bytes);
    }
    let emissions = model
        .emission_log_probabilities(
            &observation,
            &segmentation,
            1.0,
            liblrhsmm_rs::emission::EmissionPruningOptions::None,
        )
        .unwrap();
    let path = model
        .viterbi_hsmm(&segmentation, &emissions, HsmmSettings::default())
        .unwrap();
    assert!(path.log_probability.is_finite());
    assert_eq!(path.occurrences.last().unwrap().end, observation.frames);
}
