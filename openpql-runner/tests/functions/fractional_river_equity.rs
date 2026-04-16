use crate::common::run_ok;

#[test]
fn winner_fractional_equity_is_one() {
    let out = run_ok(
        "select avg(fractionalriverequity(hero)) \
         from game='holdem', hero='AhAs', villain='KhKs', board='2c3d4s7h8c'",
    );
    assert!(out.contains("AVG 0 = 1"), "stdout: {out}");
}

#[test]
fn loser_fractional_equity_is_zero() {
    let out = run_ok(
        "select avg(fractionalriverequity(villain)) \
         from game='holdem', hero='AhAs', villain='KhKs', board='2c3d4s7h8c'",
    );
    assert!(out.contains("AVG 0 = 0"), "stdout: {out}");
}

#[test]
fn two_way_tie_fractional_equity_is_half() {
    let out = run_ok(
        "select avg(fractionalriverequity(hero)) \
         from game='holdem', hero='AhAd', villain='AcAs', board='2c3d4s7h8c'",
    );
    assert!(out.contains("AVG 0 = 0.5"), "stdout: {out}");
}

#[test]
fn three_way_tie_fractional_equity_is_one_third() {
    let out = run_ok(
        "select avg(fractionalriverequity(hero)) \
         from game='holdem', hero='2c3c', villain='4d5d', v2='6h7h', board='AsKsQsJsTs'",
    );
    assert!(out.contains("AVG 0 = 0.333333"), "stdout: {out}");
}
