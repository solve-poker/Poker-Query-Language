use crate::common::{assert_count_all, assert_count_none};

#[test]
fn royal_flush_meets_min_lower_straight_flush() {
    assert_count_all(
        "select count(minhirating(hero, river, ratehihand('9h8h7h6h5h'))) \
         from game='holdem', hero='AhKh', board='QhJhTh2s3d'",
    );
}

#[test]
fn royal_flush_meets_own_rating() {
    assert_count_all(
        "select count(minhirating(hero, river, ratehihand('AhKhQhJhTh'))) \
         from game='holdem', hero='AhKh', board='QhJhTh2s3d'",
    );
}

#[test]
fn trips_meets_min_high_card_rating() {
    assert_count_all(
        "select count(minhirating(hero, flop, ratehihand('2h3d4s7c9h'))) \
         from game='holdem', hero='AcAs', board='AhKdQc'",
    );
}

#[test]
fn trips_does_not_meet_min_royal_flush() {
    assert_count_none(
        "select count(minhirating(hero, flop, ratehihand('AhKhQhJhTh'))) \
         from game='holdem', hero='AcAs', board='AhKdQc'",
    );
}

#[test]
fn high_card_does_not_meet_min_pair() {
    assert_count_none(
        "select count(minhirating(hero, flop, ratehihand('2h2d3c4s5h'))) \
         from game='holdem', hero='Ah3d', board='Kc9s7h'",
    );
}
