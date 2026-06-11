//! End-to-end smoke test for wasm32-unknown-unknown, where trials must
//! run without spawning threads.
//!
//! Run with `wasm-pack test --node openpql-runner --test wasm`.
#![cfg(all(target_arch = "wasm32", target_os = "unknown"))]

use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn test_runs_query_end_to_end() {
    let src = "select count(winshi(hero)) \
               from game='holdem', hero='AsKh', villain='2c3d', board='AhAdAc'";

    let mut out = Vec::new();
    let mut err = Vec::new();

    opql::PQLRunner::run(src, Some(100), None, &mut out, &mut err).unwrap();

    let out = String::from_utf8(out).unwrap();
    let err = String::from_utf8(err).unwrap();

    assert!(err.is_empty(), "unexpected stderr: {err}");
    assert!(out.contains("COUNT 0 = 100"), "unexpected stdout: {out}");
    assert!(out.contains("100 trials"), "unexpected stdout: {out}");
}
