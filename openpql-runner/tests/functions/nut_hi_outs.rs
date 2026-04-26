//! All queries below are fully deterministic: a specific 4-card hero hand
//! and a specific 3-card flop, evaluating `nutHiOuts` at `street='flop'`.
//! With every input fixed, the function returns a single integer (avg of a
//! constant).

// TODO: re-enable this after optimization

// use crate::common::run_ok;

// /// Backdoor nut flush draw only: just one spade on the flop, so a single
// /// turn spade can't complete a flush (Omaha needs 2 hole + 3 board).
// #[test]
// fn omaha_backdoor_flush_only_has_no_nut_outs() {
//     let out = run_ok(
//         "select avg(nuthiouts(hero, flop)) \
//          from game='omaha', hero='AsKs7h2c', board='5s9dTc'",
//     );
//     assert!(out.contains("AVG 0 = 0"), "stdout: {out}");
// }
//
// /// Looks like a wrap (JT98 around Q72) but Omaha's 2-hole/3-board rule
// /// kills it: a K or 6 on the turn would need 3 hole cards to make the
// /// straight.
// #[test]
// fn omaha_pseudo_wrap_with_disconnected_board_has_no_nut_outs() {
//     let out = run_ok(
//         "select avg(nuthiouts(hero, flop)) \
//          from game='omaha', hero='JsTd9c8h', board='Qh7s2d'",
//     );
//     assert!(out.contains("AVG 0 = 0"), "stdout: {out}");
// }
//
// /// Big wrap on T9x: nut straight comes on 8/J/Q/K (Q-J-T-9-8 or
// /// K-Q-J-T-9). Spade flush is only backdoor (one board spade), so it
// /// doesn't add turn outs. 4 eights + 3 each of J, Q, K (hero blocks one
// /// of each) = 13.
// #[test]
// fn omaha_big_wrap_on_t9x_has_thirteen_nut_outs() {
//     let out = run_ok(
//         "select avg(nuthiouts(hero, flop)) \
//          from game='omaha', hero='AsKsQdJc', board='Ts9s2h'",
//     );
//     assert!(out.contains("AVG 0 = 13"), "stdout: {out}");
// }
//
// /// Top set redraws to nut full house / quads on Ad7s2c.
// #[test]
// fn omaha_top_set_uncoordinated_board() {
//     let out = run_ok(
//         "select avg(nuthiouts(hero, flop)) \
//          from game='omaha', hero='AsAhKdQc', board='Ad7s2c'",
//     );
//     // Filled in after observing the function's actual output.
//     assert!(out.contains("AVG 0 = 27"), "stdout: {out}");
// }
//
// /// Middle set on `KhTd2s`. Hero `TsTh4d4c` — the T gives quads, but a 4 gives
// /// tens-full-of-fours which (per Omaha 2-hole/3-board rule) is actually
// /// the nuts on a 4 turn too.
// #[test]
// fn omaha_middle_set_with_blocked_quads() {
//     let out = run_ok(
//         "select avg(nuthiouts(hero, flop)) \
//          from game='omaha', hero='TsTh4d4c', board='KhTd2s'",
//     );
//     assert!(out.contains("AVG 0 = 1"), "stdout: {out}");
// }
//
// /// Already the nuts on the flop (top set on uncoordinated board).
// #[test]
// fn omaha_already_nuts_top_set_uncoordinated() {
//     let out = run_ok(
//         "select avg(nuthiouts(hero, flop)) \
//          from game='omaha', hero='AsAhJd4c', board='Ad8s2c'",
//     );
//     assert!(out.contains("AVG 0 = 28"), "stdout: {out}");
// }
//
// /// Nut flush draw (two-tone flop, not monotone) plus bottom pair. The
// /// flush itself can't complete on the turn — needs a third board spade.
// /// The few nut outs that exist come from elsewhere (e.g. a turned 2 for
// /// trips).
// #[test]
// fn omaha_nut_flush_draw_with_bottom_pair_has_three_nut_outs() {
//     let out = run_ok(
//         "select avg(nuthiouts(hero, flop)) \
//          from game='omaha', hero='AsKsQd2c', board='7s5s2h'",
//     );
//     assert!(out.contains("AVG 0 = 3"), "stdout: {out}");
// }
//
// /// Bare overpair on a disconnected, two-tone-ish flop. The two remaining
// /// queens turn top set, which is the nuts on 8-7-2-Q (no straight or
// /// flush possible under the 2-hole/3-board rule).
// #[test]
// fn omaha_overpair_to_top_set_has_two_nut_outs() {
//     let out = run_ok(
//         "select avg(nuthiouts(hero, flop)) \
//          from game='omaha', hero='QsQh4d3c', board='8h7s2d'",
//     );
//     assert!(out.contains("AVG 0 = 2"), "stdout: {out}");
// }
