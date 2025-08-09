use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PQLPlayer(u8);

impl PQLPlayer {
    pub const fn to_usize(self) -> usize {
        self.0 as usize
    }

    pub fn position(name: &str, player_names: &[&str]) -> Option<Self> {
        // assume player_names have been trimmed as per grammar rules

        player_names
            .iter()
            .position(|s| {
                s.to_ascii_lowercase() == name.to_ascii_lowercase().trim()
            })
            .map(Into::into)
    }
}

impl PartialOrd for PQLPlayer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0 == other.0 {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
}

impl From<usize> for PQLPlayer {
    fn from(i: usize) -> Self {
        Self(i.to_le_bytes()[0])
    }
}

impl TryFrom<(&str, &[&str])> for PQLPlayer {
    type Error = ParseError;

    fn try_from((name, names): (&str, &[&str])) -> Result<Self, Self::Error> {
        Self::position(name, names)
            .map_or_else(|| Err(ParseError::InvalidPlayer(name.into())), Ok)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Arbitrary for PQLPlayer {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Self(u8::arbitrary(g))
        }
    }

    #[test]
    fn test_partial_cmp() {
        assert!(PQLPlayer(8) == PQLPlayer(8));
        assert!(PQLPlayer(8) <= PQLPlayer(8));
        assert!(
            !(PQLPlayer(0) <= PQLPlayer(1) || PQLPlayer(0) >= PQLPlayer(1))
        );
    }
}
