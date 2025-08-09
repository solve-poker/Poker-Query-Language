use super::*;

#[pqlfn(arg, rtn, eval)]
pub fn to_card(s: &str) -> Result<PQLCard, RuntimeError> {
    s.parse()
        .map_or_else(|_| Err(RuntimeError::ToCardParseFailed(s.into())), Ok)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_card() {
        assert!(to_card(" 2H ").is_ok());
        assert!(to_card("2h?").is_err());
    }
}
