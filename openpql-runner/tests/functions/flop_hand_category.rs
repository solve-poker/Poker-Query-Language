use crate::common::{assert_count_all, assert_count_none};

#[test]
fn ten_with_ace_on_t_high_flop_is_top_pair() {
    assert_count_all(
        "select count(flophandcategory(hero) = FLOPTOPPAIR) \
         from game='holdem', hero='TsAh', board='7s8hTc'",
    );
}

#[test]
fn pocket_jacks_above_ten_high_flop_is_overpair() {
    assert_count_all(
        "select count(flophandcategory(hero) = FLOPOVERPAIR) \
         from game='holdem', hero='JsJh', board='7s8hTc'",
    );
}

#[test]
fn connectors_filling_straight_on_flop() {
    assert_count_all(
        "select count(flophandcategory(hero) = FLOPSTRAIGHT) \
         from game='holdem', hero='8d9s', board='7s6hTc'",
    );
}

#[test]
fn suited_connectors_making_flop_flush() {
    assert_count_all(
        "select count(flophandcategory(hero) = FLOPFLUSH) \
         from game='holdem', hero='8s9s', board='7s6s2s'",
    );
}

#[test]
fn unpaired_miss_is_not_top_pair() {
    assert_count_none(
        "select count(flophandcategory(hero) = FLOPTOPPAIR) \
         from game='holdem', hero='4s2h', board='6s8hTc'",
    );
}
