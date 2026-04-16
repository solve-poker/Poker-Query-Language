//! Shared helpers for integration tests.
//!
//! Each integration-test binary gets its own copy of this module, so
//! unused helpers are allowed.
//!
//! # Important runner quirks to know about
//!
//! 1. **One selector per `select` statement.** `PQLRunner::run` has a bug
//!    where multi-selector queries only execute the first selector's N
//!    trials, leaving later selectors with zero samples (`AVG = NaN`,
//!    `COUNT = 0` regardless of reality). Always write queries as one
//!    selector per statement, joined with `;` if you need several.
//!
//! 2. **Board ranges are predicates, not positional specs.** Even a fully
//!    specified 5-card string like `'AhKhQhJhTh'` does not pin each card
//!    to a fixed slot; the sampler just checks the predicate. Use
//!    aggregations (`count`, `avg` on suit/rank-count) rather than
//!    slot-equality assertions.
//!
//! 3. **Debug builds run 100 trials per statement, release runs 60000.**
//!    Write assertions that hold in both (e.g. a deterministic query that
//!    always returns the same value, regardless of trial count).
#![allow(dead_code)]

use opql::PQLRunner;

/// Run `src` and return `(stdout, stderr)` as strings.
pub fn run(src: &str) -> (String, String) {
    let mut out = Vec::new();
    let mut err = Vec::new();
    PQLRunner::run(src, &mut out, &mut err).unwrap();
    (
        String::from_utf8(out).unwrap(),
        String::from_utf8(err).unwrap(),
    )
}

/// Run `src`, assert stderr is empty, return stdout.
pub fn run_ok(src: &str) -> String {
    let (out, err) = run(src);
    assert!(err.is_empty(), "unexpected stderr: {err}");
    out
}

/// Run `src`, assert stdout is empty, return stderr.
pub fn run_err(src: &str) -> String {
    let (out, err) = run(src);
    assert!(out.is_empty(), "unexpected stdout: {out}");
    err
}

/// Run a single-selector `count(...)` query and return the integer count.
pub fn run_count(src: &str) -> usize {
    let out = run_ok(src);
    out.lines()
        .find_map(|l| l.strip_prefix("COUNT 0 = "))
        .and_then(|s| s.trim().parse::<usize>().ok())
        .unwrap_or_else(|| panic!("no 'COUNT 0 = <n>' line in stdout:\n{out}"))
}

/// Run a single-selector query and return the number of successful trials.
pub fn run_trials(src: &str) -> usize {
    let out = run_ok(src);
    out.lines()
        .find_map(|l| l.strip_suffix(" trials"))
        .and_then(|s| s.trim().parse::<usize>().ok())
        .unwrap_or_else(|| panic!("no '<n> trials' line in stdout:\n{out}"))
}

/// Assert a single-selector `count(...)` query's count equals the trial
/// count (i.e. the predicate held in every trial).
pub fn assert_count_all(src: &str) {
    let out = run_ok(src);
    let trials = out
        .lines()
        .find_map(|l| l.strip_suffix(" trials"))
        .and_then(|s| s.trim().parse::<usize>().ok())
        .unwrap_or_else(|| panic!("no '<n> trials' line in stdout:\n{out}"));
    let count = out
        .lines()
        .find_map(|l| l.strip_prefix("COUNT 0 = "))
        .and_then(|s| s.trim().parse::<usize>().ok())
        .unwrap_or_else(|| panic!("no 'COUNT 0 = <n>' line in stdout:\n{out}"));
    assert_eq!(
        count, trials,
        "expected count == trials ({trials}), got {count}:\n{out}"
    );
}

/// Assert a single-selector `count(...)` query's count is zero.
pub fn assert_count_none(src: &str) {
    assert_eq!(run_count(src), 0);
}
