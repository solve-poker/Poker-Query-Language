use crate::common::{assert_count_all, assert_count_none};

#[test]
fn hero_with_top_flop_rank() {
    // Flop AhKdQc → top = A. Hero holds an Ace.
    assert_count_all(
        "select count(hastopboardrank(hero, flop)) \
         from game='holdem', hero='As2c', board='AhKdQc'",
    );
}

#[test]
fn hero_without_top_flop_rank() {
    // Hero only has 2 and 3 — no Ace.
    assert_count_none(
        "select count(hastopboardrank(hero, flop)) \
         from game='holdem', hero='2s3c', board='AhKdQc'",
    );
}

#[test]
fn hero_with_top_river_rank() {
    // Board AhKdQc2s3h → top = A. Hero holds As.
    assert_count_all(
        "select count(hastopboardrank(hero, river)) \
         from game='holdem', hero='As5c', board='AhKdQc2s3h'",
    );
}

#[test]
fn hero_with_second_but_not_top_rank() {
    // Hero has K (2nd) but not A (top) → false.
    assert_count_none(
        "select count(hastopboardrank(hero, flop)) \
         from game='holdem', hero='Ks2c', board='AhKdQc'",
    );
}
