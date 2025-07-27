use super::*;

/// A enum that handles strength of hand type for different games
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum HandTypeOrd {
    #[default]
    Standard,
    Shortdeck,
}

const OFFSET_HANDTYPE: u8 = 5;

impl HandTypeOrd {
    pub(crate) const fn i16_to_hand_type(self, i: HandRatingInt) -> HandType {
        self.masks_to_kind(i.to_le_bytes()[1])
    }

    pub(crate) const fn masks_to_kind(self, byte_hi: u8) -> HandType {
        match self {
            Self::Standard => masks_to_kind_holdem(byte_hi),
            Self::Shortdeck => masks_to_kind_shortdeck(byte_hi),
        }
    }
}

const fn masks_to_kind_holdem(i: u8) -> HandType {
    match i >> OFFSET_HANDTYPE {
        0b100 => HandType::HighCard,
        0b101 => HandType::Pair,
        0b110 => HandType::TwoPair,
        0b111 => HandType::Trips,
        0b000 => HandType::Straight,
        0b001 => HandType::Flush,
        0b010 => HandType::FullHouse,
        _ => {
            if i == 0b0110_0000 {
                HandType::Quads
            } else {
                HandType::StraightFlush
            }
        }
    }
}

const fn masks_to_kind_shortdeck(i: u8) -> HandType {
    match i >> OFFSET_HANDTYPE {
        0b001 => HandType::FullHouse,
        0b010 => HandType::Flush,
        _ => masks_to_kind_holdem(i),
    }
}

impl From<PQLGame> for HandTypeOrd {
    fn from(game: PQLGame) -> Self {
        match game {
            PQLGame::Holdem | PQLGame::Omaha => Self::Standard,
            PQLGame::ShortDeck => Self::Shortdeck,
        }
    }
}

#[cfg(test)]
mod tests {
    use self::HandType::*;
    use super::*;

    impl Arbitrary for HandTypeOrd {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            *g.choose(&[Self::Standard, Self::Shortdeck]).unwrap()
        }
    }

    impl HandTypeOrd {
        pub(crate) const fn kind_to_masks(self, ht: HandType) -> u8 {
            match self {
                Self::Standard => kind_to_masks_holdem(ht),
                Self::Shortdeck => kind_to_masks_shortdeck(ht),
            }
        }
    }

    const fn kind_to_masks_shortdeck(t: HandType) -> u8 {
        match t {
            FullHouse => 0b001 << OFFSET_HANDTYPE,
            Flush => 0b010 << OFFSET_HANDTYPE,
            _ => kind_to_masks_holdem(t),
        }
    }

    const fn kind_to_masks_holdem(t: HandType) -> u8 {
        match t {
            HighCard => 0b100 << OFFSET_HANDTYPE,
            Pair => 0b101 << OFFSET_HANDTYPE,
            TwoPair => 0b110 << OFFSET_HANDTYPE,
            Trips => 0b111 << OFFSET_HANDTYPE,
            Straight => 0b000 << OFFSET_HANDTYPE,
            Flush => 0b001 << OFFSET_HANDTYPE,
            FullHouse => 0b010 << OFFSET_HANDTYPE,
            Quads | StraightFlush => 0b011 << OFFSET_HANDTYPE,
        }
    }

    #[test]
    fn test_holdem_masks() {
        let to_masks = |v| HandTypeOrd::Standard.kind_to_masks(v);
        let to_kind = |v| HandTypeOrd::Standard.masks_to_kind(v);

        assert_eq!(0b100, to_masks(HighCard) >> 5);
        assert_eq!(0b101, to_masks(Pair) >> 5);
        assert_eq!(0b110, to_masks(TwoPair) >> 5);
        assert_eq!(0b111, to_masks(Trips) >> 5);
        assert_eq!(0b000, to_masks(Straight) >> 5);
        assert_eq!(0b001, to_masks(Flush) >> 5);
        assert_eq!(0b010, to_masks(FullHouse) >> 5);
        assert_eq!(0b011, to_masks(Quads) >> 5);
        assert_eq!(0b011, to_masks(StraightFlush) >> 5);
        assert_eq!(0b011, to_masks(StraightFlush) >> 5);

        assert_eq!(to_kind(0b1000_0000), HighCard);
        assert_eq!(to_kind(0b1010_0000), Pair);
        assert_eq!(to_kind(0b1100_0000), TwoPair);
        assert_eq!(to_kind(0b1110_0000), Trips);
        assert_eq!(to_kind(0b0000_0000), Straight);
        assert_eq!(to_kind(0b0010_0000), Flush);
        assert_eq!(to_kind(0b0100_0000), FullHouse);
        assert_eq!(to_kind(0b0110_0000), Quads);
        assert_eq!(to_kind(0b0110_0010), StraightFlush);
        assert_eq!(to_kind(0b0111_1000), StraightFlush);
    }

    #[test]
    fn test_shortdeck_masks() {
        let to_masks = |v| HandTypeOrd::Shortdeck.kind_to_masks(v);
        let to_kind = |v| HandTypeOrd::Shortdeck.masks_to_kind(v);

        assert_eq!(0b100, to_masks(HighCard) >> 5);
        assert_eq!(0b101, to_masks(Pair) >> 5);
        assert_eq!(0b110, to_masks(TwoPair) >> 5);
        assert_eq!(0b111, to_masks(Trips) >> 5);
        assert_eq!(0b000, to_masks(Straight) >> 5);
        assert_eq!(0b011, to_masks(Quads) >> 5);
        assert_eq!(0b011, to_masks(StraightFlush) >> 5);
        assert_eq!(0b011, to_masks(StraightFlush) >> 5);

        assert_eq!(to_kind(0b1000_0000), HighCard);
        assert_eq!(to_kind(0b1010_0000), Pair);
        assert_eq!(to_kind(0b1100_0000), TwoPair);
        assert_eq!(to_kind(0b1110_0000), Trips);
        assert_eq!(to_kind(0b0000_0000), Straight);
        assert_eq!(to_kind(0b0110_0000), Quads);
        assert_eq!(to_kind(0b0110_0010), StraightFlush);
        assert_eq!(to_kind(0b0111_1000), StraightFlush);

        assert_eq!(0b001, to_masks(FullHouse) >> 5);
        assert_eq!(0b010, to_masks(Flush) >> 5);

        assert_eq!(to_kind(0b0010_0000), FullHouse);
        assert_eq!(to_kind(0b0100_0000), Flush);
    }
}
