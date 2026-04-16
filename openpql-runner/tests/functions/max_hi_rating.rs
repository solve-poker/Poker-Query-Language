use crate::common::run_ok;

#[test]
fn max_hi_on_river_reflects_pair_of_aces() {
    let out = run_ok(
        "select max(maxhirating(river)) \
         from game='holdem', hero='AhAs', villain='KhKs', board='2c3d4s7h8c'",
    );
    assert!(out.contains("MAX 0 = PAIR(A"), "stdout: {out}");
}

#[test]
fn max_hi_on_flop_is_pair_of_aces_before_board_fills() {
    let out = run_ok(
        "select max(maxhirating(flop)) \
         from game='holdem', hero='AhAs', villain='KhKs', board='2c3d4s'",
    );
    assert!(out.contains("MAX 0 = PAIR(A"), "stdout: {out}");
}

#[test]
fn max_hi_on_turn_is_pair_of_aces() {
    let out = run_ok(
        "select max(maxhirating(turn)) \
         from game='holdem', hero='AhAs', villain='KhKs', board='2c3d4s7h'",
    );
    assert!(out.contains("MAX 0 = PAIR(A"), "stdout: {out}");
}

#[test]
fn max_hi_on_royal_flush_board_is_straight_flush() {
    let out = run_ok(
        "select max(maxhirating(river)) \
         from game='holdem', hero='2c3c', villain='4d5d', board='AsKsQsJsTs'",
    );
    assert!(out.contains("MAX 0 = STRAIGHT_FLUSH"), "stdout: {out}");
}

#[test]
fn min_of_max_hi_equals_max_of_max_hi_when_deterministic() {
    let min_out = run_ok(
        "select min(maxhirating(river)) \
         from game='holdem', hero='AhAs', villain='KhKs', board='2c3d4s7h8c'",
    );
    assert!(min_out.contains("MIN 0 = PAIR(A"), "stdout: {min_out}");
}
