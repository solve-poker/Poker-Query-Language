use super::*;

#[derive(Debug, Clone, Copy, From, PartialEq, Eq)]
pub struct VmStoreVarIdx(u8);

impl Add<u8> for VmStoreVarIdx {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl VmStoreVarIdx {
    pub const fn to_usize(self) -> usize {
        self.0 as usize
    }
}

impl TryFrom<usize> for VmStoreVarIdx {
    type Error = PQLError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value > u8::MAX as usize {
            return Err(PQLError::TooManyVariables);
        }

        Ok(Self(value.to_le_bytes()[0]))
    }
}

impl PartialOrd for VmStoreVarIdx {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0 == other.0 {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
}

#[cfg_attr(coverage_nightly, coverage(off))]
#[cfg(test)]
mod tests {
    use super::*;

    impl Arbitrary for VmStoreVarIdx {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Self(u8::arbitrary(g))
        }
    }

    #[test]
    fn test_add() {
        let v = VmStoreVarIdx(254);

        assert_eq!(v + 1, 255.into());
    }

    #[test]
    fn test_ord() {
        use VmStoreVarIdx as I;
        assert_eq!(Some(Ordering::Equal), I(0).partial_cmp(&I(0)));
        assert_eq!(None, I(1).partial_cmp(&I(0)));
    }

    #[test]
    fn test_cast() {
        assert_eq!(
            VmStoreVarIdx(100),
            VmStoreVarIdx::try_from(100usize).unwrap()
        );
        assert_eq!(100, VmStoreVarIdx(100).to_usize());
    }
}
