use crate::common::{assert_count_all, assert_count_none};

#[test]
fn pocket_pair_flop_is_paired() {
    assert_count_all(
        "select count(pairedboard(flop)) \
         from game='holdem', board='AhAcKd'",
    );
}

#[test]
fn pocket_pair_river_is_paired() {
    assert_count_all(
        "select count(pairedboard(river)) \
         from game='holdem', board='AhAcKdQsJh'",
    );
}

#[test]
fn unpaired_flop_is_not_paired() {
    assert_count_none(
        "select count(pairedboard(flop)) \
         from game='holdem', board='AhKdQc'",
    );
}

#[test]
fn unpaired_river_is_not_paired() {
    assert_count_none(
        "select count(pairedboard(river)) \
         from game='holdem', board='AhKdQc2s3h'",
    );
}
