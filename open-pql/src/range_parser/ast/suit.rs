use super::{Error, LocInfo, ResultE};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Const {
    S,
    H,
    D,
    C,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Var {
    W,
    X,
    Y,
    Z,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Suit {
    Const(Const),
    Var(Var),
}

impl Suit {
    pub(crate) fn from_token(c: char, loc: LocInfo) -> ResultE<'static, Self> {
        match c {
            's' | 'S' => Ok(Self::Const(Const::S)),
            'h' | 'H' => Ok(Self::Const(Const::H)),
            'd' | 'D' => Ok(Self::Const(Const::D)),
            'c' | 'C' => Ok(Self::Const(Const::C)),

            'w' | 'W' => Ok(Self::Var(Var::W)),
            'x' | 'X' => Ok(Self::Var(Var::X)),
            'y' | 'Y' => Ok(Self::Var(Var::Y)),
            'z' | 'Z' => Ok(Self::Var(Var::Z)),

            _ => Err(Error::InvalidSuit(loc).into()),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::{
        super::{
            super::tests::parse_card as p, Suit, SuitConst as SC,
            SuitVar as SV, card::RangeCard as C,
        },
        *,
    };

    impl quickcheck::Arbitrary for Const {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            *g.choose(&[Self::S, Self::H, Self::D, Self::C]).unwrap()
        }
    }

    impl Const {
        pub fn to_src(&self) -> String {
            match self {
                Self::S => 's',
                Self::H => 'h',
                Self::D => 'd',
                Self::C => 'c',
            }
            .to_string()
        }
    }

    impl quickcheck::Arbitrary for Var {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            *g.choose(&[Self::W, Self::X, Self::Y, Self::Z]).unwrap()
        }
    }

    impl Var {
        pub fn to_src(&self) -> String {
            match self {
                Self::W => 'w',
                Self::X => 'x',
                Self::Y => 'y',
                Self::Z => 'z',
            }
            .to_string()
        }
    }

    #[test]
    fn test_card_suit_const() {
        assert_eq!(p("S"), Ok(C::AC(SC::S)));
        assert_eq!(p("H"), Ok(C::AC(SC::H)));
        assert_eq!(p("D"), Ok(C::AC(SC::D)));
        assert_eq!(p("C"), Ok(C::AC(SC::C)));

        assert_eq!(p("s"), Ok(C::AC(SC::S)));
        assert_eq!(p("h"), Ok(C::AC(SC::H)));
        assert_eq!(p("d"), Ok(C::AC(SC::D)));
        assert_eq!(p("c"), Ok(C::AC(SC::C)));
    }

    #[test]
    fn test_card_suit_var() {
        assert_eq!(p("W"), Ok(C::AV(SV::W)));
        assert_eq!(p("X"), Ok(C::AV(SV::X)));
        assert_eq!(p("Y"), Ok(C::AV(SV::Y)));
        assert_eq!(p("Z"), Ok(C::AV(SV::Z)));

        assert_eq!(p("w"), Ok(C::AV(SV::W)));
        assert_eq!(p("x"), Ok(C::AV(SV::X)));
        assert_eq!(p("y"), Ok(C::AV(SV::Y)));
        assert_eq!(p("z"), Ok(C::AV(SV::Z)));
    }

    #[test]
    fn test_from_token() {
        // unreachable
        assert![Suit::from_token('?', (0, 1)).is_err()];
    }
}
