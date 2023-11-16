pub fn parse_two_complement_64(number: u64) -> i64 {
    if number >> 63 & 1 == 0 {
        -(!number as i64 + 1)
    } else {
        number as i64
    }
}

pub fn into_two_complement_64(number: i64) -> u64 {
    number as u64
}
