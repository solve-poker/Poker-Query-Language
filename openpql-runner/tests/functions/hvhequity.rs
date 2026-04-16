use crate::common::run_ok;

#[test]
fn aces_win_on_river_equity_is_one() {
    let out = run_ok(
        "select avg(equity(hero, river)) \
         from game='holdem', hero='AhAs', villain='KhKs', board='2c3d4s7h8c'",
    );
    assert!(out.contains("AVG 0 = 1"), "stdout: {out}");
}

#[test]
fn losing_hand_on_river_equity_is_zero() {
    let out = run_ok(
        "select avg(equity(villain, river)) \
         from game='holdem', hero='AhAs', villain='KhKs', board='2c3d4s7h8c'",
    );
    assert!(out.contains("AVG 0 = 0"), "stdout: {out}");
}

#[test]
fn tied_hands_on_river_equity_is_half() {
    let out = run_ok(
        "select avg(equity(hero, river)) \
         from game='holdem', hero='AhAd', villain='AcAs', board='2c3d4s7h8c'",
    );
    assert!(out.contains("AVG 0 = 0.5"), "stdout: {out}");
}

#[test]
fn flop_equity_enumerates_runouts_deterministically() {
    let out = run_ok(
        "select avg(equity(hero, flop)) \
         from game='holdem', hero='AhAs', villain='KhKs', board='2c3d4s'",
    );
    assert!(out.contains("AVG 0 = 0.914965986394559"), "stdout: {out}");
}

#[test]
fn turn_equity_enumerates_rivers_deterministically() {
    let out = run_ok(
        "select avg(equity(hero, turn)) \
         from game='holdem', hero='AhAs', villain='KhKs', board='2c3d4s7h'",
    );
    assert!(out.contains("AVG 0 = 0.9583333333333325"), "stdout: {out}");
}
