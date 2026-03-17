use std::{any, hash::Hash};

use derive_more::{Deref, Index, IntoIterator};
use quickcheck::{Arbitrary, Gen};
use rustc_hash::FxHashSet;

#[derive(Clone, Debug, Index, IntoIterator, Deref)]
pub struct Distinct<const N: usize, T>(pub Vec<T>);

impl<const N: usize, T: Arbitrary + Eq + Hash + Clone> Arbitrary
    for Distinct<N, T>
{
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn arbitrary(g: &mut Gen) -> Self {
        let mut set = FxHashSet::default();
        let max_attempts = N.saturating_mul(1000).max(10000);

        for _ in 0..max_attempts {
            if set.len() == N {
                return Self(set.into_iter().collect());
            }

            set.insert(T::arbitrary(g));
        }

        panic!("Failed to generate {N} {}", any::type_name::<T>());
    }
}
