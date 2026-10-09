use liblrhsmm_rs::geometric::StateScoreTable;

#[test]
fn score_import_checks_shape_and_retains_all_scalar_bits() {
    let bits = [0x7fc01234_u32, 0x80000000, 0x7f800000, 1];
    let table = StateScoreTable::from_values(2, 2, bits.map(f32::from_bits).to_vec()).unwrap();
    assert_eq!(
        table
            .values()
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>(),
        bits
    );
    assert_eq!((table.frames(), table.states()), (2, 2));
    assert_eq!(table.get(2, 0), None);
    let bits = [
        0x7ff8000000001234_u64,
        0x8000000000000000,
        0x7ff0000000000000,
        0x3ff0000000000001,
    ];
    let table = StateScoreTable::from_values(2, 2, bits.map(f64::from_bits).to_vec()).unwrap();
    assert_eq!(
        table
            .values()
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>(),
        bits
    );
    for (frames, states, count) in [(0, 2, 0), (2, 0, 0), (2, 3, 4), (usize::MAX, 2, 0)] {
        assert!(StateScoreTable::from_values(frames, states, vec![0.0_f32; count]).is_err());
        assert!(StateScoreTable::from_values(frames, states, vec![0.0_f64; count]).is_err());
    }
}
