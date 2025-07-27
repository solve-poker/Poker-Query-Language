use super::transmute;

#[inline]
pub const fn normalize_u64(c: u64) -> [u8; 8] {
    // transmute is faster than calling to_le_bytes 4 times.
    unsafe {
        let [s, h, d, c]: [u16; 4] = transmute(c);

        let has4 = s & h & d & c;
        let has3 = s & h & d | s & h & c | s & d & c | h & d & c;

        let has2 = s & h | s & d | s & c | h & d | h & c | d & c;
        let has1 = s | h | d | c;

        transmute([has1, has2, has3, has4])
    }
}

const U16_D: u16 = 1 << 15;

#[inline]
pub const fn retain_leading_bit(v: u16) -> u16 {
    if v == 0 {
        0
    } else {
        U16_D >> v.leading_zeros()
    }
}

#[inline]
pub const fn retain_leading_2_bits(v: u16) -> u16 {
    let mem = retain_leading_bit(v);

    retain_leading_bit(v & !mem) | mem
}

#[inline]
pub const fn retain_leading_3_bits(v: u16) -> u16 {
    let mem = retain_leading_2_bits(v);

    retain_leading_bit(v & !mem) | mem
}

#[inline]
pub const fn retain_leading_4_bits(v: u16) -> u16 {
    let mem = retain_leading_3_bits(v);

    retain_leading_bit(v & !mem) | mem
}

#[inline]
pub const fn retain_leading_5_bits(v: u16) -> u16 {
    let mem = retain_leading_4_bits(v);

    retain_leading_bit(v & !mem) | mem
}
