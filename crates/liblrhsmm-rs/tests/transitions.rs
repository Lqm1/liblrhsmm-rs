use liblrhsmm_rs::Segmentation;

#[test]
fn preserves_c_jump_order_weights_and_repeated_builds() {
    let mut segmentation = Segmentation::new(0, 4);
    for (source, delta, probability) in [
        (0, 2, 0.2),
        (1, 0, 0.3),
        (2, -2, 0.4),
        (3, -2, 0.5),
        (0, -100, 0.1),
        (3, 100, 0.1),
    ] {
        segmentation.add_jump(source, delta, probability).unwrap();
    }
    for state in 0..4 {
        segmentation
            .add_jump(state, 1, 0.6 + state as f32 * 0.1)
            .unwrap();
    }
    let graph = segmentation.build_transitions().unwrap();
    let fixture = include_str!("../../../tests/fixtures/c-transitions.txt");
    assert_eq!(fixture.lines().count(), 16);
    for line in fixture.lines() {
        let mut fields = line.split_whitespace();
        let _pass: usize = fields.next().unwrap().parse().unwrap();
        let state: usize = fields.next().unwrap().parse().unwrap();
        let index: usize = fields.next().unwrap().parse().unwrap();
        let delta: i32 = fields.next().unwrap().parse().unwrap();
        let probability: f32 = fields.next().unwrap().parse().unwrap();
        assert_eq!(graph.incoming()[state][index].delta, delta);
        assert_eq!(graph.incoming()[state][index].probability, probability);
    }
    assert_eq!(
        graph.incoming(),
        segmentation.build_transitions().unwrap().incoming()
    );
    assert_eq!(graph.outgoing(), segmentation.outgoing);
}

#[test]
fn rejects_invalid_weights_and_ignores_out_of_bounds_offsets_safely() {
    let mut segmentation = Segmentation::new(0, 2);
    let before = segmentation.clone();
    assert!(segmentation.add_jump(0, 0, f32::NAN).is_err());
    assert!(segmentation.add_jump(2, 0, 1.0).is_err());
    assert_eq!(segmentation, before);
    segmentation.add_jump(0, i32::MIN, 0.5).unwrap();
    segmentation.add_jump(1, i32::MAX, 0.5).unwrap();
    let graph = segmentation.build_transitions().unwrap();
    assert_eq!(
        graph.incoming().iter().map(Vec::len).collect::<Vec<_>>(),
        [1, 1]
    );
    drop(graph);
    segmentation.outgoing[0][0].probability = -1.0;
    assert!(segmentation.build_transitions().is_err());
}
