use super::*;

/// a struct that decodes i16 to human readable Hand Ranking
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct HandRatingView {
    pub(crate) order: HandTypeOrd,
    pub(crate) rating: PQLHiRating,
}

impl HandRatingView {
    const fn new(order: HandTypeOrd, value: PQLHiRating) -> Self {
        Self {
            order,
            rating: value,
        }
    }

    fn get_hand_type_and_hi_lo_ranks(self) -> (HandType, Rank16, Rank16) {
        let [lo, hi] = self.rating.to_i16().to_le_bytes();
        let ht = self.order.masks_to_kind(hi);
        let layout = ht.to_layout();
        let (low, high) = layout.masks_to_ranks(lo, hi);

        (ht, Rank16::from_u16(high), Rank16::from_u16(low))
    }
}

impl fmt::Debug for HandRatingView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (ht, high, low) = self.get_hand_type_and_hi_lo_ranks();
        let (high, low) = (high.to_u16(), low.to_u16());

        if low > 0 {
            f.write_str(&format!(
                "HandRatingView<{:?}>({:?}, {}, {})",
                &self.order,
                ht,
                u16_to_rank_str(high),
                u16_to_rank_str(low),
            ))
        } else {
            f.write_str(&format!(
                "HandRatingView<{:?}>({:?}, {})",
                &self.order,
                ht,
                u16_to_rank_str(high),
            ))
        }
    }
}

impl From<(PQLGame, PQLHiRating)> for HandRatingView {
    fn from((g, r): (PQLGame, PQLHiRating)) -> Self {
        Self::new(g.into(), r)
    }
}

#[cfg(test)]
impl From<(PQLGame, HandType, Rank16, Rank16)> for HandRatingView {
    fn from((g, ht, h, l): (PQLGame, HandType, Rank16, Rank16)) -> Self {
        let ord: HandTypeOrd = g.into();
        let k = ord.kind_to_masks(ht);

        let (lo, hi) = ht.to_layout().ranks_to_masks(l.to_u16(), h.to_u16());
        Self::new(ord, PQLHiRating::new(i16::from_le_bytes([lo, hi | k])))
    }
}

#[cfg(test)]
mod tests {
    use self::HandType::*;
    use super::*;

    fn mk_view(g: PQLGame, ht: HandType, h: &str, l: &str) -> HandRatingView {
        (g, ht, r16!(h), r16!(l)).into()
    }

    #[test]
    fn test_debug() {
        let game = PQLGame::Holdem;
        let view_name = "HandRatingView";
        let ord_name = "Standard";

        assert_eq!(
            format!("{:?}", mk_view(game, Pair, "4", "23T")),
            format!("{view_name}<{ord_name}>(Pair, 4, 23T)"),
        );

        assert_eq!(
            format!("{:?}", mk_view(game, StraightFlush, "K", "")),
            format!("{view_name}<{ord_name}>(StraightFlush, K)"),
        );
    }

    #[test]
    fn test_royalflush() {
        let game = PQLGame::Holdem;

        assert_eq!(
            mk_view(game, StraightFlush, "A", ""),
            mk_view(game, StraightFlush, "A", ""),
        );
    }
}
