use crate::PQLDouble;

type FractionInner = i32;

/// Exact rational number used for equity pot shares.
#[derive(
    Clone, Copy, derive_more::Debug, PartialEq, Eq, derive_more::Display,
)]
#[display("{} / {}", self.num, self.den)]
#[debug("{self}")]
pub struct PQLFraction {
    num: FractionInner,
    den: FractionInner,
}

impl PQLFraction {
    /// Builds a fraction from a numerator and denominator.
    pub const fn new(num: FractionInner, den: FractionInner) -> Self {
        Self { num, den }
    }

    /// Returns `0 / 1`.
    pub const fn zero() -> Self {
        Self::new(0, 1)
    }

    /// Converts the fraction to a `PQLDouble`.
    #[allow(clippy::cast_lossless)]
    pub const fn to_double(self) -> PQLDouble {
        (self.num as PQLDouble) / (self.den as PQLDouble)
    }

    /// Returns a hero's share of the pot when tied with `n_winners` players.
    ///
    /// # Panics
    /// won't panic since `n_winners` ≤ 10
    pub fn pot_share(n_winners: usize) -> Self {
        let den = FractionInner::try_from(n_winners).unwrap();

        Self::new(1, den)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        let f = PQLFraction::zero();
        assert_eq!(f.num, 0);
        assert_eq!(f.den, 1);
    }

    #[test]
    fn test_potshare() {
        let f = PQLFraction::pot_share(10);
        assert_eq!(f.num, 1);
        assert_eq!(f.den, 10);
    }
}
