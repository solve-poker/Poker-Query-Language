use super::*;

/// Card index representation (0-51)
///
/// Maps each card to a unique index from 0 to 51, where:
/// - Index = rank * 4 + suit
/// - Ranks are ordered 0-12 (2 through Ace)
/// - Suits are ordered 0-3 (Spades, Hearts, Diamonds, Clubs)
///
/// # Examples
///
/// ```
/// use open_pql::{Card, CardIdx, Rank::*, Suit::*};
///
/// let idx = CardIdx::from(Card::new(R2, S)); // First card (index 0)
/// assert_eq!(idx.to_u8(), 0);
/// assert_eq!(idx.to_card(), Card::new(R2, S));
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CardIdx(u8);

impl CardIdx {
    /// Converts to a u8 value
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Card, CardIdx, Rank::*, Suit::*};
    ///
    /// let idx = CardIdx::from(Card::new(R2, S));
    /// assert_eq!(idx.to_u8(), 0);
    /// ```
    #[must_use]
    pub const fn to_u8(self) -> u8 {
        self.0
    }

    /// Converts to a usize value
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Card, CardIdx, Rank::*, Suit::*};
    ///
    /// let idx = CardIdx::from(Card::new(R3, H));
    /// assert_eq!(idx.to_usize(), 5); // rank=1, suit=1 -> 1*4+1 = 5
    /// ```
    #[must_use]
    pub const fn to_usize(self) -> usize {
        self.0 as usize
    }

    /// Converts to a Card
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Card, CardIdx, Rank::*, Suit::*};
    ///
    /// let idx = CardIdx::from(Card::new(RA, C));
    /// let card = idx.to_card();
    /// assert_eq!(card.rank, RA);
    /// assert_eq!(card.suit, C);
    /// ```
    #[must_use]
    pub fn to_card(self) -> Card {
        let rank = self.0 / 4;
        let suit = self.0 % 4;
        Card::from_indices(RankIdx::new(rank), SuitIdx::new(suit))
    }
}

impl From<Card> for CardIdx {
    fn from(card: Card) -> Self {
        let rank_idx = card.rank as u8;
        let suit_idx = card.suit as u8;
        Self(rank_idx * 4 + suit_idx)
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
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
