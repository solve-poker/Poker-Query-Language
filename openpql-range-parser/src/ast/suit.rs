use super::{Display, Error, LocInfo, ResultE};

pub type SuitConst = openpql_prelude::Suit;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Display)]
pub enum SuitVar {
    #[display("w")]
    W,
    #[display("x")]
    X,
    #[display("y")]
    Y,
    #[display("z")]
    Z,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Display)]
pub enum CardSuit {
    #[display("{_0}")]
    Const(SuitConst),
    #[display("{_0}")]
    Var(SuitVar),
}

impl CardSuit {
    pub(crate) fn from_token(c: char, loc: LocInfo) -> ResultE<'static, Self> {
        match c {
            's' | 'S' => Ok(Self::Const(SuitConst::S)),
            'h' | 'H' => Ok(Self::Const(SuitConst::H)),
            'd' | 'D' => Ok(Self::Const(SuitConst::D)),
            'c' | 'C' => Ok(Self::Const(SuitConst::C)),

            'w' | 'W' => Ok(Self::Var(SuitVar::W)),
            'x' | 'X' => Ok(Self::Var(SuitVar::X)),
            'y' | 'Y' => Ok(Self::Var(SuitVar::Y)),
            'z' | 'Z' => Ok(Self::Var(SuitVar::Z)),

            _ => Err(Error::InvalidSuit(loc).into()),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::*;

    impl Arbitrary for SuitVar {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            *g.choose(&[Self::W, Self::X, Self::Y, Self::Z]).unwrap()
        }
    }

    #[test]
    fn test_card_suit_const() {
        assert_range_card("S", "s");
        assert_range_card("H", "h");
        assert_range_card("D", "d");
        assert_range_card("C", "c");

        assert_range_card("s", "s");
        assert_range_card("h", "h");
        assert_range_card("d", "d");
        assert_range_card("c", "c");
    }

    #[test]
    fn test_card_suit_var() {
        assert_range_card("W", "w");
        assert_range_card("X", "x");
        assert_range_card("Y", "y");
        assert_range_card("Z", "z");

        assert_range_card("w", "w");
        assert_range_card("x", "x");
        assert_range_card("y", "y");
        assert_range_card("z", "z");
    }

    #[test]
    fn test_from_token() {
        // unreachable
        assert![CardSuit::from_token('?', (0, 1)).is_err()];
    }
}
