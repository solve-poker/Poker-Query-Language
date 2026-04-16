use crate::common::{assert_count_all, assert_count_none};

#[test]
fn river_royal_flush_rating_equals_rate_hi_hand() {
    assert_count_all(
        "select count(hirating(hero, river) = ratehihand('AhKhQhJhTh')) \
         from game='holdem', hero='AhKh', board='QhJhTh2s3d'",
    );
}

#[test]
fn flop_trips_rating_equals_rate_hi_hand() {
    assert_count_all(
        "select count(hirating(hero, flop) = ratehihand('AcAsAhKdQc')) \
         from game='holdem', hero='AcAs', board='AhKdQc'",
    );
}

#[test]
fn royal_flush_rating_is_greater_than_high_card() {
    assert_count_all(
        "select count(hirating(hero, river) > ratehihand('Ah7d5c3s2h')) \
         from game='holdem', hero='AhKh', board='QhJhTh2s3d'",
    );
}

#[test]
fn trips_rating_does_not_equal_royal_flush() {
    assert_count_none(
        "select count(hirating(hero, flop) = ratehihand('AhKhQhJhTh')) \
         from game='holdem', hero='AcAs', board='AhKdQc'",
    );
}

#[test]
fn high_card_rating_is_less_than_pair() {
    assert_count_all(
        "select count(hirating(hero, flop) < ratehihand('2h2d3c4s5h')) \
         from game='holdem', hero='Ah2d', board='Kc9s4h'",
    );
}
