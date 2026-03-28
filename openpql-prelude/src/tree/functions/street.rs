//! Betting round (street) navigation and analysis utilities.
//!
//! [`AnnotatedAction`] -> `Street`

use crate::{
    Street,
    tree::{AnnotatedAction, functions::filter_count},
};

type ChanceCount = u8;

/// returns number of chances occured
fn count_chance(history: &[AnnotatedAction]) -> ChanceCount {
    filter_count(0, history, &|a: &AnnotatedAction| {
        matches!(a, AnnotatedAction::Chance(_))
    })
}

/// returns number of chances occured
pub fn current_street(history: &[AnnotatedAction]) -> Option<Street> {
    match count_chance(history) {
        1 => Some(Street::Preflop),
        2 => Some(Street::Flop),
        3 => Some(Street::Turn),
        4 => Some(Street::River),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{tree::tests::*, *};

    fn mk_history(n: usize) -> Vec<AnnotatedAction> {
        vec![PREFLOP; n]
            .into_iter()
            .chain([AnnotatedAction::Join(0, 0)])
            .collect()
    }

    #[test]
    fn test_current_street() {
        assert_eq!(current_street(&mk_history(0)), None);
        assert_eq!(current_street(&mk_history(1)), Some(Street::Preflop));
        assert_eq!(current_street(&mk_history(2)), Some(Street::Flop));
        assert_eq!(current_street(&mk_history(3)), Some(Street::Turn));
        assert_eq!(current_street(&mk_history(4)), Some(Street::River));
    }
}
