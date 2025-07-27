use super::*;

#[pqlfn(arg, rtn, eval)]
pub fn to_rank(s: &str) -> Result<Option<PQLRank>, RuntimeError> {
    s.parse::<PQLRank>().map_or_else(
        |_| Err(RuntimeError::ToRankParseFailed(s.into())),
        |r| Ok(Some(r)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_rank() {
        assert!(to_rank(" K ").is_ok());
        assert!(to_rank("Ks").is_err());
    }
}
