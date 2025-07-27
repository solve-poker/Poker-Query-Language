use eval::get_card_count;

use super::*;

type OutsStraightFlush = PQLRankSet;
type OutsFlush = PQLRankSet;
type OutsStraight = PQLRankSet;
type OutsQuads = PQLRankSet;
type OutsFullhouse = PQLRankSet;
type OutsTrips = PQLRankSet;
type OutsTwopair = PQLRankSet;
type OutsPair = PQLRankSet;
type OutsHighcard = PQLRankSet;

type CardCount = (PQLRankSet, PQLRankSet, PQLRankSet, PQLRankSet);

#[allow(dead_code)]
// assume player has 2 cards and board has 3 or 4 cards
pub fn outs_omaha(player: Card64, board: Card64) -> OutsInfo {
    let (v0, v1, v2, v3, v4, v5) = decompose_omaha(player);

    let mut o0 = outs_holdem(v0, board);
    let o1 = outs_holdem(v1, board);
    let o2 = outs_holdem(v2, board);
    let o3 = outs_holdem(v3, board);
    let o4 = outs_holdem(v4, board);
    let o5 = outs_holdem(v5, board);

    o0.merge(&o1);
    o0.merge(&o2);
    o0.merge(&o3);
    o0.merge(&o4);
    o0.merge(&o5);

    o0
}

#[allow(dead_code)]
// assume player has 2 cards and board has 3 or 4 cards
pub fn outs_shortdeck(player: Card64, board: Card64) -> OutsInfo {
    let ht = eval_shortdeck7(player | board)
        .to_hand_type(PQLGame::ShortDeck)
        .ht;

    let mut outs_info = OutsInfo::default();

    let mut straightflush = PQLRankSet::default();
    let mut flush = PQLRankSet::default();
    let mut suit = None;

    for (ranks, s) in (player | board).iter_ranks() {
        if ranks.count() >= 4 {
            (straightflush, flush) = outs_flush::<true>(ranks);

            suit = Some(s);
            break;
        }
    }

    if !straightflush.is_empty() {
        outs_info.straightflush = Card64::from_u64(
            u64::from(straightflush.to_u16()) << (16 * suit.unwrap() as u8),
        );
    }

    if ht == HandType::StraightFlush {
        return outs_info;
    }

    let (has1, has2, has3, has4) =
        get_card_count_strict(player.to_u64() | board.to_u64());

    if has4.count() > 0 {
        outs_info.quads =
            Card64::from_ranks(outs_quads((has1, has2, has3, has4)));

        return outs_info;
    }

    if !flush.is_empty() {
        outs_info.flush = Card64::from_u64(
            u64::from(flush.to_u16()) << (16 * suit.unwrap() as u8),
        );
    }

    if ht == HandType::Flush {
        return outs_info;
    }

    if has3.count() > 0 && has2.count() > 0 {
        let (quads, fullhose) = outs_fullhouse((has1, has2, has3, has4));

        outs_info.quads = Card64::from_ranks(quads);
        outs_info.fullhouse = Card64::from_ranks(fullhose);

        return outs_info;
    }

    let straight = outs_straight::<true>(has1 | has2 | has3 | has4);

    if !straight.is_empty() {
        outs_info.straight = Card64::from_ranks(straight);
    }

    if ht == HandType::Straight {
        return outs_info;
    }

    if has3.count() > 0 {
        let (quads, fullhose, trips) = outs_trips((has1, has2, has3, has4));

        outs_info.quads = Card64::from_ranks(quads);
        outs_info.fullhouse = Card64::from_ranks(fullhose);
        outs_info.trips = Card64::from_ranks(trips);

        return outs_info;
    }

    if has2.count() > 1 {
        let (fullhose, twopair) = outs_twopair((has1, has2, has3, has4));

        outs_info.fullhouse = Card64::from_ranks(fullhose);
        outs_info.twopair = Card64::from_ranks(twopair);

        return outs_info;
    }

    if has2.count() > 0 {
        let (trips, twopair, pair) = outs_pair((has1, has2, has3, has4));

        outs_info.trips = Card64::from_ranks(trips);
        outs_info.twopair = Card64::from_ranks(twopair);
        outs_info.pair = Card64::from_ranks(pair);

        return outs_info;
    }

    let (pair, highcard) = outs_highcard((has1, has2, has3, has4));

    outs_info.pair = Card64::from_ranks(pair);
    outs_info.highcard = Card64::from_ranks(highcard);

    outs_info
}

#[allow(dead_code)]
// assume player has 2 cards and board has 3 or 4 cards
pub fn outs_holdem(player: Card64, board: Card64) -> OutsInfo {
    let ht = eval_holdem7(player | board)
        .to_hand_type(PQLGame::Holdem)
        .ht;

    let mut outs_info = OutsInfo::default();

    let mut straightflush = PQLRankSet::default();
    let mut flush = PQLRankSet::default();
    let mut suit = None;

    for (ranks, s) in (player | board).iter_ranks() {
        if ranks.count() >= 4 {
            (straightflush, flush) = outs_flush::<false>(ranks);

            suit = Some(s);
            break;
        }
    }

    if !straightflush.is_empty() {
        outs_info.straightflush = Card64::from_u64(
            u64::from(straightflush.to_u16()) << (16 * suit.unwrap() as u8),
        );
    }

    if ht == HandType::StraightFlush {
        return outs_info;
    }

    let (has1, has2, has3, has4) =
        get_card_count_strict(player.to_u64() | board.to_u64());

    if has4.count() > 0 {
        outs_info.quads =
            Card64::from_ranks(outs_quads((has1, has2, has3, has4)));

        return outs_info;
    }

    if has3.count() > 0 && has2.count() > 0 {
        let (quads, fullhose) = outs_fullhouse((has1, has2, has3, has4));

        outs_info.quads = Card64::from_ranks(quads);
        outs_info.fullhouse = Card64::from_ranks(fullhose);

        return outs_info;
    }

    if !flush.is_empty() {
        outs_info.flush = Card64::from_u64(
            u64::from(flush.to_u16()) << (16 * suit.unwrap() as u8),
        );
    }

    if ht == HandType::Flush {
        return outs_info;
    }

    let straight = outs_straight::<false>(has1 | has2 | has3 | has4);

    if !straight.is_empty() {
        outs_info.straight = Card64::from_ranks(straight);
    }

    if ht == HandType::Straight {
        return outs_info;
    }

    if has3.count() > 0 {
        let (quads, fullhose, trips) = outs_trips((has1, has2, has3, has4));

        outs_info.quads = Card64::from_ranks(quads);
        outs_info.fullhouse = Card64::from_ranks(fullhose);
        outs_info.trips = Card64::from_ranks(trips);

        return outs_info;
    }

    if has2.count() > 1 {
        let (fullhose, twopair) = outs_twopair((has1, has2, has3, has4));

        outs_info.fullhouse = Card64::from_ranks(fullhose);
        outs_info.twopair = Card64::from_ranks(twopair);

        return outs_info;
    }

    if has2.count() > 0 {
        let (trips, twopair, pair) = outs_pair((has1, has2, has3, has4));

        outs_info.trips = Card64::from_ranks(trips);
        outs_info.twopair = Card64::from_ranks(twopair);
        outs_info.pair = Card64::from_ranks(pair);

        return outs_info;
    }

    let (pair, highcard) = outs_highcard((has1, has2, has3, has4));

    outs_info.pair = Card64::from_ranks(pair);
    outs_info.highcard = Card64::from_ranks(highcard);

    outs_info
}

#[inline]
pub fn outs_quads((has1, has2, has3, has4): CardCount) -> OutsQuads {
    PQLRankSet::higher_of(has1 | has2 | has3) & !has4
}

#[inline]
pub fn outs_fullhouse(
    (has1, has2, has3, _): CardCount,
) -> (OutsQuads, OutsFullhouse) {
    // Two Cases: AAABBB or AAABBC
    let mut res = PQLRankSet::default();

    if has3.count() == 1 {
        if has1 > has2 {
            res |= has1;
        }

        if has2 > has3 {
            res |= has2;
        }
    }

    (has3, res)
}

#[inline]
pub fn outs_trips(
    (has1, _, has3, _): CardCount,
) -> (OutsQuads, OutsFullhouse, OutsTrips) {
    (has3, has1, better_trips(has1, has3))
}

#[inline]
pub fn outs_twopair(
    (has1, has2, _, _): CardCount,
) -> (OutsFullhouse, OutsTwopair) {
    (has2, better_twopair(has1, has2))
}

#[inline]
pub fn outs_pair(
    (has1, has2, _, _): CardCount,
) -> (OutsTrips, OutsTwopair, OutsPair) {
    (has2, has1, better_pair(has1, has1 | has2))
}

#[inline]
pub fn outs_highcard((has1, _, _, _): CardCount) -> (OutsPair, OutsHighcard) {
    (has1, better_highcard(has1))
}

#[inline]
pub fn outs_straight<const S: bool>(has1: PQLRankSet) -> OutsStraight {
    let mut res = 0;
    let has1 = has1.to_u16();

    let arr = if S {
        &ARR_STRAIGHT_SHORT as &[_]
    } else {
        &ARR_STRAIGHT as &[_]
    };

    for straight in arr {
        if has1 & *straight == *straight {
            break;
        }

        if (has1 & straight).count_ones() == 4 {
            res |= straight & !has1;
        }
    }

    PQLRankSet::from_u16(res)
}

#[inline]
pub fn outs_flush<const S: bool>(
    has1: PQLRankSet,
) -> (OutsStraightFlush, OutsFlush) {
    let straightflush = outs_straight::<S>(has1);

    let fifth_rank = has1
        .nth_rank(5)
        .map_or_else(PQLRankSet::default, Into::into);

    let flush = PQLRankSet::higher_of(fifth_rank) & !(straightflush | has1);

    (straightflush, flush)
}

#[derive(Debug, Clone, Default)]
pub struct OutsInfo {
    pub highcard: Card64,
    pub pair: Card64,
    pub twopair: Card64,
    pub trips: Card64,
    pub fullhouse: Card64,
    pub quads: Card64,
    pub flush: Card64,
    pub straight: Card64,
    pub straightflush: Card64,
}

impl OutsInfo {
    pub fn outs_of(&self, ht: HandType) -> Card64 {
        (match ht {
            HandType::HighCard => self.highcard,
            HandType::Pair => self.pair,
            HandType::TwoPair => self.twopair,
            HandType::Trips => self.trips,
            HandType::Straight => self.straight,
            HandType::Flush => self.flush,
            HandType::FullHouse => self.fullhouse,
            HandType::Quads => self.quads,
            HandType::StraightFlush => self.straightflush,
        }) & !self.higher_of(ht)
    }

    fn higher_of(&self, ht: HandType) -> Card64 {
        match ht {
            HandType::HighCard => {
                self.pair
                    | self.twopair
                    | self.trips
                    | self.straight
                    | self.flush
                    | self.fullhouse
                    | self.quads
                    | self.straightflush
            }
            HandType::Pair => {
                self.twopair
                    | self.trips
                    | self.straight
                    | self.flush
                    | self.fullhouse
                    | self.quads
                    | self.straightflush
            }
            HandType::TwoPair => {
                self.trips
                    | self.straight
                    | self.flush
                    | self.fullhouse
                    | self.quads
                    | self.straightflush
            }
            HandType::Trips => {
                self.straight
                    | self.flush
                    | self.fullhouse
                    | self.quads
                    | self.straightflush
            }
            HandType::Straight => {
                self.flush | self.fullhouse | self.quads | self.straightflush
            }
            HandType::Flush => self.fullhouse | self.quads | self.straightflush,
            HandType::FullHouse => self.quads | self.straightflush,
            HandType::Quads => self.straightflush,
            HandType::StraightFlush => Card64::default(),
        }
    }

    fn merge(&mut self, other: &Self) {
        self.highcard |= other.highcard;
        self.pair |= other.pair;
        self.twopair |= other.twopair;
        self.trips |= other.trips;
        self.straight |= other.straight;
        self.flush |= other.flush;
        self.fullhouse |= other.fullhouse;
        self.quads |= other.quads;
        self.straightflush |= other.straightflush;
    }
}

#[inline]
pub const fn get_card_count_strict(
    c: u64,
) -> (PQLRankSet, PQLRankSet, PQLRankSet, PQLRankSet) {
    let (has1, has2, has3, has4) = get_card_count(c);

    (
        PQLRankSet::from_u16(has1 & !(has2 | has3 | has4)),
        PQLRankSet::from_u16(has2 & !(has3 | has4)),
        PQLRankSet::from_u16(has3 & !has4),
        PQLRankSet::from_u16(has4),
    )
}

#[inline]
fn better_trips(has1: PQLRankSet, has3: PQLRankSet) -> PQLRankSet {
    let second_kicker: PQLRankSet = has1.nth_rank(2).unwrap().into();

    PQLRankSet::higher_of(second_kicker) & !(has1 | has3)
}

#[inline]
fn better_twopair(has1: PQLRankSet, has2: PQLRankSet) -> PQLRankSet {
    let second_pair: PQLRankSet = has2.nth_rank(2).unwrap().into();

    let better_pair = PQLRankSet::higher_of(second_pair) & has1;
    let better_kicker = PQLRankSet::higher_of(has1);

    (better_pair | better_kicker) & !has2
}

#[inline]
fn better_pair(has1: PQLRankSet, nonkicker: PQLRankSet) -> PQLRankSet {
    let third_kicker: PQLRankSet = has1.nth_rank(3).unwrap().into();

    PQLRankSet::higher_of(third_kicker) & !nonkicker
}

#[inline]
fn better_highcard(has1: PQLRankSet) -> PQLRankSet {
    let fifth_kicker: PQLRankSet = has1.nth_rank(5).unwrap().into();

    PQLRankSet::higher_of(fifth_kicker) & !has1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_input(s: &str) -> (PQLRankSet, PQLRankSet, PQLRankSet, PQLRankSet) {
        let mut s = s.split('|');

        let p = c64!(s.next().unwrap());
        let b = c64!(s.next().unwrap());

        get_card_count_strict((p | b).to_u64())
    }

    #[test]
    fn test_outs_quads() {
        let f = |s| outs_quads(to_input(s));

        assert_eq!(f("2s2h | 2d2cQh"), r16!("KA"));
        assert_eq!(f("QsQh | QdQcTh"), r16!("JKA"));
    }

    #[test]
    fn test_outs_fullhouse() {
        let f = |s| outs_fullhouse(to_input(s));

        assert_eq!(f("2s2h | 2d3s3h"), (r16!("2"), r16!("3")));
        assert_eq!(f("TsTh | 3s3h3dQs"), (r16!("3"), r16!("TQ")));
    }

    #[test]
    fn test_outs_trips() {
        let f = |s| outs_trips(to_input(s));

        assert_eq!(f("2s2h | 2dQhAh"), (r16!("2"), r16!("AQ"), r16!("K")));
    }

    #[test]
    fn test_outs_twopair() {
        let f = |s| outs_twopair(to_input(s));

        assert_eq!(f("2s2h | AsAh5d"), (r16!("2A"), r16!("56789TJQK")));
    }

    #[test]
    fn test_outs_pair() {
        let f = |s| outs_pair(to_input(s));

        assert_eq!(
            f("2s2h | AhJh5d"),
            (r16!("2"), r16!("AJ5"), r16!("6789TQK"))
        );
    }

    #[test]
    fn test_outs_highcard() {
        let f = |s| outs_highcard(to_input(s));

        assert_eq!(f("2s3s | AhJh5d"), (r16!("235JA"), r16!("46789TQK")));
    }

    #[test]
    fn test_outs_straight() {
        let f = |s| outs_straight::<false>(to_input(s).0);

        assert_eq!(f("2s3h | 4d5cKh"), r16!("A6"));
        assert_eq!(f("2s3h | 4d5cAh"), r16!("6"));
    }

    #[test]
    fn test_outs_flush() {
        let f = |s: &str| outs_flush::<false>(r16!(s));

        assert_eq!(f("2348"), (r16!(""), r16!("5679TJQKA")));
        assert_eq!(f("A2348"), (r16!("5"), r16!("679TJQK")));
    }
}
