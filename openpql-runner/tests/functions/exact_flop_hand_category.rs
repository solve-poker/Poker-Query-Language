use crate::common::{assert_count_all, assert_count_none};

#[test]
fn overpair_matches_exact_overpair() {
    assert_count_all(
        "select count(exactflophandcategory(hero, FLOPOVERPAIR)) \
         from game='holdem', hero='JsJh', board='7s8hTc'",
    );
}

#[test]
fn pocket_pair_hitting_set_matches_exact_set() {
    assert_count_all(
        "select count(exactflophandcategory(hero, FLOPSET)) \
         from game='holdem', hero='7h7d', board='7s6hTc'",
    );
}

#[test]
fn top_pair_does_not_match_exact_overpair() {
    assert_count_none(
        "select count(exactflophandcategory(hero, FLOPOVERPAIR)) \
         from game='holdem', hero='TsAh', board='7s8hTc'",
    );
}

#[test]
fn miss_matches_exact_nothing() {
    assert_count_all(
        "select count(exactflophandcategory(hero, FLOPNOTHING)) \
         from game='holdem', hero='4s2h', board='6s8hTc'",
    );
}

#[test]
fn top_pair_does_not_match_exact_straight() {
    assert_count_none(
        "select count(exactflophandcategory(hero, FLOPSTRAIGHT)) \
         from game='holdem', hero='TsAh', board='7s8hTc'",
    );
}
