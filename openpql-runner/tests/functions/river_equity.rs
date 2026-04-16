use crate::common::run_ok;

#[test]
fn winner_has_equity_one() {
    let out = run_ok(
        "select avg(riverequity(hero)) \
         from game='holdem', hero='AhAs', villain='KhKs', board='2c3d4s7h8c'",
    );
    assert!(out.contains("AVG 0 = 1"), "stdout: {out}");
}

#[test]
fn loser_has_equity_zero() {
    let out = run_ok(
        "select avg(riverequity(villain)) \
         from game='holdem', hero='AhAs', villain='KhKs', board='2c3d4s7h8c'",
    );
    assert!(out.contains("AVG 0 = 0"), "stdout: {out}");
}

#[test]
fn tied_hands_split_to_half() {
    let out = run_ok(
        "select avg(riverequity(hero)) \
         from game='holdem', hero='AhAd', villain='AcAs', board='2c3d4s7h8c'",
    );
    assert!(out.contains("AVG 0 = 0.5"), "stdout: {out}");
}

#[test]
fn three_way_tie_on_royal_flush_board() {
    let out = run_ok(
        "select avg(riverequity(hero)) \
         from game='holdem', hero='2c3c', villain='4d5d', v2='6h7h', board='AsKsQsJsTs'",
    );
    assert!(out.contains("AVG 0 = 0.333333"), "stdout: {out}");
}
