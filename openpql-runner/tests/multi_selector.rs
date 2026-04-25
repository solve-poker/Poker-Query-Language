mod common;

use common::run_ok;

/// Regression: prior to the loop-inversion fix, only the first selector
/// in a multi-selector `select` received any trials; later selectors
/// reported `COUNT = 0` / `AVG = NaN`. This deterministic game has hero
/// always winning and villain always losing, so a correctly-wired runner
/// must report a positive count for selector 0 and zero for selector 1.
#[test]
fn second_selector_receives_trials() {
    let out = run_ok(
        "select count(winshi(hero)), count(winshi(villain)) \
         from game='holdem', hero='AhAs', villain='KhKs', board='2c3d4s7h8c'",
    );

    let trials = out
        .lines()
        .find_map(|l| l.strip_suffix(" trials"))
        .and_then(|s| s.trim().parse::<usize>().ok())
        .unwrap_or_else(|| panic!("no '<n> trials' line:\n{out}"));

    let count_0 = out
        .lines()
        .find_map(|l| l.strip_prefix("COUNT 0 = "))
        .and_then(|s| s.trim().parse::<usize>().ok())
        .unwrap_or_else(|| panic!("no 'COUNT 0 = <n>' line:\n{out}"));

    let count_1 = out
        .lines()
        .find_map(|l| l.strip_prefix("COUNT 1 = "))
        .and_then(|s| s.trim().parse::<usize>().ok())
        .unwrap_or_else(|| panic!("no 'COUNT 1 = <n>' line:\n{out}"));

    assert_eq!(count_0, trials, "hero must win every trial:\n{out}");
    assert_eq!(count_1, 0, "villain must lose every trial:\n{out}");
}

#[test]
fn third_selector_receives_trials() {
    let out = run_ok(
        "select count(winshi(hero)), count(winshi(villain)), count(winshi(hero)) \
         from game='holdem', hero='AhAs', villain='KhKs', board='2c3d4s7h8c'",
    );

    let trials = out
        .lines()
        .find_map(|l| l.strip_suffix(" trials"))
        .and_then(|s| s.trim().parse::<usize>().ok())
        .unwrap();

    let count_2 = out
        .lines()
        .find_map(|l| l.strip_prefix("COUNT 2 = "))
        .and_then(|s| s.trim().parse::<usize>().ok())
        .unwrap_or_else(|| panic!("no 'COUNT 2 = <n>' line:\n{out}"));

    assert_eq!(count_2, trials, "third selector lost trials:\n{out}");
}
