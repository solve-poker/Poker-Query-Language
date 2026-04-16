use crate::common::{assert_count_all, assert_count_none};

#[test]
fn equity_of_one_meets_threshold_of_half() {
    assert_count_all(
        "select count(min_equity(hero, river, 0.5)) \
         from game='holdem', hero='AhAs', villain='KhKs', board='2c3d4s7h8c'",
    );
}

#[test]
fn equity_of_one_meets_threshold_of_one() {
    assert_count_all(
        "select count(min_equity(hero, river, 1.0)) \
         from game='holdem', hero='AhAs', villain='KhKs', board='2c3d4s7h8c'",
    );
}

#[test]
fn equity_of_zero_fails_positive_threshold() {
    assert_count_none(
        "select count(min_equity(villain, river, 0.5)) \
         from game='holdem', hero='AhAs', villain='KhKs', board='2c3d4s7h8c'",
    );
}

#[test]
fn tied_equity_of_half_meets_threshold_of_half() {
    assert_count_all(
        "select count(min_equity(hero, river, 0.5)) \
         from game='holdem', hero='AhAd', villain='AcAs', board='2c3d4s7h8c'",
    );
}

#[test]
fn tied_equity_of_half_fails_threshold_above_half() {
    assert_count_none(
        "select count(min_equity(hero, river, 0.51)) \
         from game='holdem', hero='AhAd', villain='AcAs', board='2c3d4s7h8c'",
    );
}
