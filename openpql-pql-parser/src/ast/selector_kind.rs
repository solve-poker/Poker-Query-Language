use super::fmt;

#[derive(Clone, Copy, PartialEq, Eq, derive_more::Debug)]
pub enum SelectorKind {
    #[debug("avg")]
    Avg,
    #[debug("count")]
    Count,
    #[debug("max")]
    Max,
    #[debug("min")]
    Min,
}

impl fmt::Display for SelectorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Avg => f.write_str("AVG"),
            Self::Count => f.write_str("COUNT"),
            Self::Max => f.write_str("MAX"),
            Self::Min => f.write_str("MIN"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(SelectorKind::Avg.to_string(), "AVG");
        assert_eq!(SelectorKind::Count.to_string(), "COUNT");
        assert_eq!(SelectorKind::Max.to_string(), "MAX");
        assert_eq!(SelectorKind::Min.to_string(), "MIN");
    }
}
