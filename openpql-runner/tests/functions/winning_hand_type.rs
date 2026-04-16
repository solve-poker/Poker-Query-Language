use crate::common::run_ok;

#[test]
fn pair_on_dry_board_wins_as_pair() {
    let out = run_ok(
        "select max(winninghandtype()) \
         from game='holdem', hero='AhAs', villain='KhKs', board='2c3d4s7h8c'",
    );
    assert!(out.contains("MAX 0 = PAIR"), "stdout: {out}");
}

#[test]
fn royal_flush_board_wins_as_straight_flush() {
    let out = run_ok(
        "select max(winninghandtype()) \
         from game='holdem', hero='2c3c', villain='4d5d', board='AsKsQsJsTs'",
    );
    assert!(out.contains("MAX 0 = STRAIGHT_FLUSH"), "stdout: {out}");
}

#[test]
fn quads_on_board_win_as_four_of_a_kind() {
    let out = run_ok(
        "select max(winninghandtype()) \
         from game='holdem', hero='2c3c', villain='5d6d', board='AhAdAcAs7h'",
    );
    assert!(out.contains("MAX 0 = QUADS"), "stdout: {out}");
}

#[test]
fn high_card_only_when_no_pair_possible() {
    let out = run_ok(
        "select max(winninghandtype()) \
         from game='holdem', hero='2c3d', villain='4s5h', board='7d9sJcKhAd'",
    );
    assert!(out.contains("MAX 0 = HIGH_CARD"), "stdout: {out}");
}

#[test]
fn min_selector_reports_same_deterministic_type() {
    let out = run_ok(
        "select min(winninghandtype()) \
         from game='holdem', hero='AhAs', villain='KhKs', board='2c3d4s7h8c'",
    );
    assert!(out.contains("MIN 0 = PAIR"), "stdout: {out}");
}
