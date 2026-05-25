use crate::{Card, CardCount, FlushingSuit, IsomorphicCard, Suit};

#[inline]
pub(super) const fn n_flush_suits(cards: &[IsomorphicCard]) -> CardCount {
    let (mut x, mut y, mut z) = (false, false, false);
    let mut idx = 0;

    while idx < cards.len() {
        match cards[idx].suit {
            FlushingSuit::X => x = true,
            FlushingSuit::Y => y = true,
            FlushingSuit::Z => z = true,
            FlushingSuit::N => {}
        }
        idx += 1;
    }

    x as CardCount + y as CardCount + z as CardCount
}

#[inline]
pub(super) const fn place_card(
    c: IsomorphicCard,
    next: CardCount,
) -> (Card, CardCount) {
    const fn take(suit: FlushingSuit, next: CardCount) -> (Suit, CardCount) {
        match suit {
            FlushingSuit::X => (Suit::S, next),
            FlushingSuit::Y => (Suit::H, next),
            FlushingSuit::Z => (Suit::D, next),
            FlushingSuit::N => {
                (Suit::ARR_ALL[(next % Suit::N_SUITS) as usize], next + 1)
            }
        }
    }

    let (s, next) = take(c.suit, next);

    (Card::new(c.rank, s), next)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_z_suit() {
        let z = IsomorphicCard::new(Rank::RA, FlushingSuit::Z);

        assert_eq!(n_flush_suits(&[z]), 1);
        assert_eq!(place_card(z, 0), (Card::new(Rank::RA, Suit::D), 0));
    }
}
