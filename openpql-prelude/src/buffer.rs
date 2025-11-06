use std::io;

use super::{Card, HandN};

pub trait BufferWrite {
    fn write_to<W: io::Write>(self, w: &mut W) -> io::Result<()>;
}

// TODO: refactor
mod hand_n {
    use super::{BufferWrite, Card, HandN, io};

    const CARD_BITS: usize = 6;

    const fn card_to_bits(card: Card) -> u8 {
        const N_SUITS: u8 = 4;
        let r = card.rank as u8;
        let s = card.suit as u8;

        r * N_SUITS + s
    }

    fn hand_2_to_bits(hand: HandN<2>) -> u16 {
        let mut res = 0;

        for (i, &card) in hand.iter().enumerate() {
            let bits = u16::from(card_to_bits(card));
            res |= bits << (i * CARD_BITS);
        }

        res
    }

    fn hand_n_to_bits<const N: usize>(hand: HandN<N>) -> u32 {
        assert!(N >= 3 && N <= 5);
        let mut res = 0;

        for (i, &card) in hand.iter().enumerate() {
            let bits = u32::from(card_to_bits(card));
            res |= bits << (i * CARD_BITS);
        }

        res
    }

    macro_rules! impl_buf_write_hand {
        ($kind:ty, $proc:expr) => {
            impl BufferWrite for $kind {
                fn write_to<W: io::Write>(self, w: &mut W) -> io::Result<()> {
                    w.write_all(&$proc(self).to_le_bytes())
                }
            }
        };
    }

    impl_buf_write_hand!(HandN<2>, hand_2_to_bits);
    impl_buf_write_hand!(HandN<3>, hand_n_to_bits);
    impl_buf_write_hand!(HandN<4>, hand_n_to_bits);
    impl_buf_write_hand!(HandN<5>, hand_n_to_bits);
}
