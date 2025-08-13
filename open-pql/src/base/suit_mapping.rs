use super::Suit;

/// Maps original suits to isomorphic suits for normalization
#[derive(Clone, Debug, Default)]
pub struct SuitMapping {
    map: [Option<Suit>; 4],
    next_suit: Suit,
}

impl SuitMapping {
    /// Creates a new empty suit mapping
    pub const fn new() -> Self {
        Self {
            map: [None; 4],
            next_suit: Suit::S,
        }
    }

    /// Maps a suit to its isomorphic equivalent, creating a new mapping if needed
    pub const fn map_suit(&mut self, suit: Suit) -> Suit {
        let idx = suit as usize;
        if let Some(iso_suit) = self.map[idx] {
            return iso_suit;
        }

        let iso_suit = self.next_suit;
        self.map[idx] = Some(iso_suit);
        self.next_suit = match self.next_suit {
            Suit::S => Suit::H,
            Suit::H => Suit::D,
            Suit::D | Suit::C => Suit::C,
        };

        iso_suit
    }

    /// Returns the number of suits that have been mapped
    pub fn len(&self) -> usize {
        self.map.iter().filter(|&&s| s.is_some()).count()
    }

    /// Returns true if no suits have been mapped
    pub fn is_empty(&self) -> bool {
        self.map.iter().all(|&s| s.is_none())
    }

    /// Clears all mappings
    pub fn clear(&mut self) {
        self.map = [None; 4];
        self.next_suit = Suit::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    const W: Suit = Suit::S;
    const X: Suit = Suit::H;
    const Y: Suit = Suit::D;
    const Z: Suit = Suit::C;

    #[test]
    fn test_suit_mapping() {
        let mut mapping = SuitMapping::new();

        let s0 = mapping.map_suit(Suit::H);
        assert_eq!(s0, W);

        let s1 = mapping.map_suit(Suit::S);
        assert_eq!(s1, X);

        let s0_again = mapping.map_suit(Suit::H);
        assert_eq!(s0_again, W);

        let s2 = mapping.map_suit(Suit::C);
        assert_eq!(s2, Y);

        let s3 = mapping.map_suit(Suit::D);
        assert_eq!(s3, Z);

        assert_eq!(mapping.len(), 4);

        mapping.clear();
        assert_eq!(mapping.len(), 0);
        assert!(mapping.is_empty());
    }
}
