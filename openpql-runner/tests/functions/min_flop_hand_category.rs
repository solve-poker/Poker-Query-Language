use crate::common::{assert_count_all, assert_count_none};

#[test]
fn overpair_meets_min_top_pair() {
    assert_count_all(
        "select count(minflophandcategory(hero, FLOPTOPPAIR)) \
         from game='holdem', hero='JsJh', board='7s8hTc'",
    );
}

#[test]
fn overpair_meets_min_overpair() {
    assert_count_all(
        "select count(minflophandcategory(hero, FLOPOVERPAIR)) \
         from game='holdem', hero='JsJh', board='7s8hTc'",
    );
}

#[test]
fn top_pair_does_not_meet_min_overpair() {
    assert_count_none(
        "select count(minflophandcategory(hero, FLOPOVERPAIR)) \
         from game='holdem', hero='TsAh', board='7s8hTc'",
    );
}

#[test]
fn nothing_does_not_meet_min_top_pair() {
    assert_count_none(
        "select count(minflophandcategory(hero, FLOPTOPPAIR)) \
         from game='holdem', hero='4s2h', board='6s8hTc'",
    );
}

#[test]
fn straight_meets_min_top_two() {
    assert_count_all(
        "select count(minflophandcategory(hero, FLOPTOPTWO)) \
         from game='holdem', hero='8d9s', board='7s6hTc'",
    );
}
