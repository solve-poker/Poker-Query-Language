use std::{
    arch::x86_64::__m256i,
    fmt::{Debug, Formatter, Result},
    mem,
};

use crate::Rank16;

pub trait View16: Debug {
    fn from_bytes(bytes: [u8; 2]) -> Self;
}

impl View16 for Rank16 {
    fn from_bytes(bytes: [u8; 2]) -> Self {
        Self::from_u16(u16::from_le_bytes(bytes))
    }
}

pub struct Bits16(u16);

impl Debug for Bits16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut flags = [false; 16];
        for (i, b) in flags.iter_mut().enumerate() {
            *b = 1 << i & self.0 != 0;
        }

        let chars = flags.map(|b| if b { '1' } else { '_' });

        f.write_str(&format!(
            "{} {}",
            chars[..8].iter().collect::<String>(),
            chars[8..].iter().collect::<String>(),
        ))
    }
}

impl View16 for Bits16 {
    fn from_bytes(bytes: [u8; 2]) -> Self {
        Self(u16::from_le_bytes(bytes))
    }
}

pub fn dbg_mm256<T: View16>(v: __m256i) -> Vec<(usize, String)> {
    unsafe {
        let bytes: [u8; 16 * 2] = mem::transmute(v);

        (0..16)
            .map(|i| {
                let arr = [bytes[i * 2], bytes[i * 2 + 1]];

                (i, format!("{:?}", T::from_bytes(arr)))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_debug() {
        unsafe {
            let ranks = [
                r16!("2"),
                r16!("3"),
                r16!("4"),
                r16!("5"),
                r16!("6"),
                r16!("7"),
                r16!("8"),
                r16!("9"),
                r16!("T"),
                r16!("J"),
                r16!("Q"),
                r16!("K"),
                r16!("A"),
                Rank16::empty(),
                Rank16::empty(),
                Rank16::empty(),
            ];

            let arr: [u16; 16] = ranks.map(Rank16::to_u16);

            let n: __m256i = mem::transmute(arr);

            let out = dbg_mm256::<Rank16>(n);

            for i in 0..16 {
                assert_eq!(out[i].0, i);
                assert_eq!(out[i].1, format!("{:?}", ranks[i]));
            }

            let out = dbg_mm256::<Bits16>(n);

            assert_eq!(out[0].1, "1_______ ________");
            assert_eq!(out[8].1, "________ 1_______");
        }
    }
}
