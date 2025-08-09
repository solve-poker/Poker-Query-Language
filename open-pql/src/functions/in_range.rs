use super::*;

#[pqlfn(arg, rtn, eval)]
pub fn in_range(hand: &Hand, range: &mut PQLRange) -> PQLBoolean {
    range.is_satisfied(hand)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_in_range() {
        let mut c = PQLRange::from_src("AA", PQLGame::default()).unwrap();

        in_range(&cards!("AsAh"), &mut c);
    }
}
