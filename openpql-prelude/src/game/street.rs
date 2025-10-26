use super::{Board, Card64, CardCount, Display, FromStr, ParseError};

#[derive(
    Debug, Clone, PartialEq, Eq, Copy, PartialOrd, Ord, Default, Display,
)]
pub enum Street {
    #[default]
    Preflop = 0,
    Flop,
    Turn,
    River,
}

impl Street {
    // won't truncate since n ≤ 5
    #[allow(clippy::cast_possible_truncation)]
    pub const fn board_card_count(self) -> CardCount {
        (match self {
            Self::Preflop => Board::N_PREFLOP,
            Self::Flop => Board::N_FLOP,
            Self::Turn => Board::N_TURN,
            Self::River => Board::N_RIVER,
        }) as CardCount
    }
}

impl FromStr for Street {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().trim() {
            "preflop" => Ok(Self::Preflop),
            "flop" => Ok(Self::Flop),
            "turn" => Ok(Self::Turn),
            "river" => Ok(Self::River),

            _ => Err(ParseError::InvalidStreet(s.into())),
        }
    }
}

impl From<(Board, Street)> for Card64 {
    fn from((board, street): (Board, Street)) -> Self {
        match street {
            Street::Preflop => Self::EMPTY,
            Street::Flop => board.to_c64_flop(),
            Street::Turn => board.to_c64_flop() | board.to_c64_turn(),
            Street::River => {
                board.to_c64_flop() | board.to_c64_turn() | board.to_c64_river()
            }
        }
    }
}

#[cfg(any(test, feature = "quickcheck"))]
impl quickcheck::Arbitrary for Street {
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        #[allow(unused)]
        const fn completeness_check(e: Street) {
            match e {
                Street::Preflop
                | Street::Flop
                | Street::Turn
                | Street::River => (),
            }
        }
        *g.choose(&[Self::Preflop, Self::Flop, Self::Turn, Self::River])
            .unwrap()
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
pub mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_to_card64_with_board() {
        assert_eq!(
            Card64::from((board!("As Kh Qd Jc Ts"), Street::Preflop)),
            c64!("")
        );

        assert_eq!(
            Card64::from((board!("As Kh Qd Jc Ts"), Street::Flop)),
            c64!("As Kh Qd")
        );

        assert_eq!(
            Card64::from((board!("As Kh Qd Jc Ts"), Street::Turn)),
            c64!("As Kh Qd Jc")
        );

        assert_eq!(
            Card64::from((board!("As Kh Qd Jc Ts"), Street::River)),
            c64!("As Kh Qd Jc Ts")
        );

        assert_eq!(
            Card64::from((board!("As Kh Qd"), Street::River)),
            c64!("As Kh Qd")
        );

        assert!(Card64::from((board!(""), Street::River)).is_empty());
    }

    #[quickcheck]
    fn test_n_board(street: Street) {
        let n = match street {
            Street::Preflop => 0,
            Street::Flop => 3,
            Street::Turn => 4,
            Street::River => 5,
        };

        assert_eq!(street.board_card_count(), n);
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Ok(Street::Preflop), "preFlop".parse());
        assert_eq!(Ok(Street::Flop), "Flop".parse());
        assert_eq!(Ok(Street::Turn), "tUrn".parse());
        assert_eq!(Ok(Street::River), "riVer".parse());

        assert_eq!(Ok(Street::Flop), " flop ".parse(), "should trim");

        assert!("invalid".parse::<Street>().is_err());
    }
}
