use liblrhsmm_rs::PreparationOptions;

#[path = "common/hsmm.rs"]
mod common;

#[test]
fn every_hsmm_viterbi_score_matches_instrumented_c() {
    let fixture = include_str!("fixtures/corrected-c-viterbi-scores.txt");
    assert_eq!(fixture.lines().count(), 384);
    for family in 0..2 {
        for scenario in 0..4 {
            let (model, segmentation, emissions, mut options) = common::sample(scenario);
            if family == 1 {
                options.state_radius = [0.25, 0.5, 1.75, 2.5][scenario];
            }
            let path = model
                .viterbi_hsmm(&segmentation, &emissions, options)
                .unwrap();
            let prepared = model
                .prepare(PreparationOptions {
                    duration_cache_length: 2,
                    temperature: options.temperature,
                    duration_weight: options.duration_weight,
                })
                .unwrap();
            assert_eq!(
                prepared
                    .viterbi_hsmm(&segmentation, &emissions, options)
                    .unwrap(),
                path
            );
            let mut records = 0;
            for line in fixture.lines() {
                let fields: Vec<_> = line.split_whitespace().collect();
                assert_eq!(fields[0], "vscore");
                if fields[1].parse::<usize>().unwrap() != family
                    || fields[2].parse::<usize>().unwrap() != scenario
                {
                    continue;
                }
                let index: usize = fields[3].parse().unwrap();
                let begin: f32 = fields[4].parse().unwrap();
                let end: f32 = fields[5].parse().unwrap();
                assert_eq!(path.scores.begin.values()[index], begin, "begin: {line}");
                assert_eq!(path.scores.end.values()[index], end, "end: {line}");
                records += 1;
            }
            assert_eq!(records, 48);
            assert_eq!(path.log_probability, path.scores.end.get(11, 3).unwrap());
        }
    }
}
