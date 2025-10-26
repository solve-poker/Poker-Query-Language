use super::*;

#[derive(Clone, Copy, derive_more::Debug, PartialEq, Eq, Display)]
#[display("{} / {}", self.num, self.den)]
#[debug("{self}")]
pub struct PQLFraction {
    num: FractionInner,
    den: FractionInner,
}

impl PQLFraction {
    pub const fn new(num: FractionInner, den: FractionInner) -> Self {
        Self { num, den }
    }

    pub const fn zero() -> Self {
        Self::new(0, 1)
    }

    #[allow(clippy::cast_lossless)]
    pub const fn to_double(self) -> PQLDouble {
        (self.num as PQLDouble) / (self.den as PQLDouble)
    }

    /// # Panics
    /// won't panic since `n_winners` ≤ 10
    pub(crate) fn pot_share(n_winners: usize) -> Self {
        let den = FractionInner::try_from(n_winners).unwrap();

        Self::new(1, den)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::*;

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
