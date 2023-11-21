use crate::utils::{i12_to_u64, u64_to_i12};

#[test]
fn test_i12_complement() {
    let pairs: Vec<(i64, u16)> = vec![
        (0, 0b000000000000),
        (200, 0b000011001000),
        (2047, 0b011111111111),
        (-1, 0b111111111111),
        (-200, 0b111100111000),
        (-2048, 0b100000000000),
        (-12, 0b111111110100),
    ];

    for (big, small) in pairs {
        println!("testing {} => {}", big, small);
        let new_small = u64_to_i12(big as u64);
        assert_eq!(new_small, small);
        let new_big: u64 = i12_to_u64(small);
        assert_eq!(new_big, big as u64);
    }
}
