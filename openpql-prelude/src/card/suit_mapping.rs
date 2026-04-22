use super::Suit;

/// Incremental suit-to-suit mapping for isomorphic normalization.
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
#[derive(Clone, Debug, Default)]
pub struct SuitMapping {
    map: [Option<Suit>; Suit::N_SUITS as usize],
    next_suit: Suit,
}

impl SuitMapping {
    /// Creates an empty mapping.
    pub const fn new() -> Self {
        Self {
            map: [None; Suit::N_SUITS as usize],
            next_suit: Suit::S,
        }
    }

    /// Returns the mapped suit, assigning the next canonical suit on first use.
    pub const fn map_suit(&mut self, suit: Suit) -> Suit {
        let idx = suit as usize;
        if let Some(iso_suit) = self.map[idx] {
            return iso_suit;
        }

        let iso_suit = self.next_suit;
        self.map[idx] = Some(iso_suit);
        self.advance_suit();

        iso_suit
    }

    /// Returns the number of mapped suits.
    pub fn len(&self) -> usize {
        self.map.iter().filter(|&&s| s.is_some()).count()
    }

    /// Returns `true` when no suits are mapped.
    pub fn is_empty(&self) -> bool {
        self.map.iter().all(|&s| s.is_none())
    }

    const fn advance_suit(&mut self) {
        self.next_suit = match self.next_suit {
            Suit::S => Suit::H,
            Suit::H => Suit::D,
            _ => Suit::C,
        };
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    const W: Suit = Suit::S;
    const X: Suit = Suit::H;
    const Y: Suit = Suit::D;
    const Z: Suit = Suit::C;

    #[test]
    fn test_suit_mapping() {
        let mut mapping = SuitMapping::new();

        assert_eq!(mapping.len(), 0);
        assert!(mapping.is_empty());

        let suits = [Suit::H, Suit::S, Suit::H, Suit::C, Suit::D];
        let mapped = suits.map(|suit| mapping.map_suit(suit));

        assert_eq!(mapped, [W, X, W, Y, Z]);

        assert_eq!(mapping.len(), 4);
    }
}
