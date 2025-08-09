use super::*;

/// Card index representation (0-51)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CardIdx(u8);

impl CardIdx {
    /// Converts to a u8 value
    pub const fn to_u8(self) -> u8 {
        self.0
    }

    /// Converts to a usize value
    pub const fn to_usize(self) -> usize {
        self.0 as usize
    }

    /// Converts to a Card
    pub fn to_card(self) -> Card {
        let rank = self.0 / 4;
        let suit = self.0 % 4;
        Card::from_indices(rank, suit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_to_card() {
        for i in 0..N_CARDS {
            assert_eq!(CardIdx(i).to_card(), Card::ARR_ALL[i as usize]);
        }
    }
}
