use crate::common::{assert_count_all, run, run_ok, run_trials};

#[test]
fn where_tautology_keeps_all_trials() {
    let baseline = run_trials(
        "select count(pairedboard(flop)) \
         from game='holdem', board='AhAcKd'",
    );
    let with_where = run_trials(
        "select count(pairedboard(flop)) \
         from game='holdem', board='AhAcKd' \
         where 1 = 1",
    );
    assert_eq!(baseline, with_where);
}

#[test]
fn where_filters_to_matching_predicate() {
    assert_count_all(
        "select count(pairedboard(flop)) \
         from game='holdem', board='AhAcKd' \
         where pairedboard(flop)",
    );
}

#[test]
fn where_filters_out_on_contradictory_board() {
    let (_, err) = run("select count(pairedboard(flop)) \
         from game='holdem', board='AhAcKd' \
         where not pairedboard(flop)");
    assert!(err.contains("SamplingFailed"), "stderr: {err}");
}

#[test]
fn where_and_narrows_both_conditions() {
    assert_count_all(
        "select count(pairedboard(flop)) \
         from game='holdem', board='AhAcKd' \
         where pairedboard(flop) and 1 = 1",
    );
}

#[test]
fn where_or_keeps_when_either_true() {
    assert_count_all(
        "select count(pairedboard(flop)) \
         from game='holdem', board='AhAcKd' \
         where pairedboard(flop) or 1 = 2",
    );
}

#[test]
fn where_non_boolean_is_type_error() {
    let (_, err) = run("select count(pairedboard(flop)) \
         from game='holdem', board='AhAcKd' \
         where 1 + 1");
    assert!(err.contains("TypeError"), "stderr: {err}");
}

#[test]
fn where_keyword_is_case_insensitive() {
    let out = run_ok(
        "SELECT count(pairedboard(flop)) \
         FROM game='holdem', board='AhAcKd' \
         WHERE pairedboard(flop) AND NOT (1 = 2) OR 1 = 1",
    );
    assert!(out.contains("COUNT 0"));
}
