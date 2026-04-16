use crate::common::{assert_count_all, assert_count_none};

#[test]
fn hero_holds_second_flop_rank() {
    // Flop AhKdQc → 2nd rank is K. Hero holds Ks.
    assert_count_all(
        "select count(hassecondboardrank(hero, flop)) \
         from game='holdem', hero='Ks2c', board='AhKdQc'",
    );
}

#[test]
fn hero_holding_only_top_rank_does_not_have_second() {
    // Hero has A (top), not K (2nd).
    assert_count_none(
        "select count(hassecondboardrank(hero, flop)) \
         from game='holdem', hero='As2c', board='AhKdQc'",
    );
}

#[test]
fn hero_holding_only_third_rank_does_not_have_second() {
    // Hero has Q (3rd), not K (2nd).
    assert_count_none(
        "select count(hassecondboardrank(hero, flop)) \
         from game='holdem', hero='Qs2c', board='AhKdQc'",
    );
}

#[test]
fn hero_holds_second_river_rank() {
    // Board AhKdQc2s3h → ranks = {A,K,Q,3,2}, 2nd = K. Hero has K.
    assert_count_all(
        "select count(hassecondboardrank(hero, river)) \
         from game='holdem', hero='Ks5c', board='AhKdQc2s3h'",
    );
}
