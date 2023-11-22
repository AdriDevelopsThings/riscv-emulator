/// converts a signed 12 bit integer to an u64 integer that can be interpreted as an i64
pub fn i12_to_u64(x: u16) -> u64 {
    if x & 0x0800 != 0 {
        (x as i64 - 0x1000) as u64
    } else {
        x as u64
    }
}

pub fn i12_to_u64_unsigned(x: u16) -> u64 {
    (x & 0x7FF) as u64
}

pub fn i20_to_u64(x: u32) -> u64 {
    if x & 0x080000 != 0 {
        (x as i64 - 0x100000) as u64
    } else {
        x as u64
    }
}
