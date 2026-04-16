use super::*;

#[pqlfn]
pub fn flushing_board(ctx: &PQLFnContext, street: PQLStreet) -> PQLBoolean {
    core::flushing_board(PQLBoard::from(ctx.get_board_slice(street)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    fn f(s: &str, street: PQLStreet) -> bool {
        let ctx = TestPQLFnContext::from_board(&cards!(s));
        let ctx = ctx.as_ctx();

        flushing_board(&ctx, street)
    }

    #[test]
    fn test_flushing_board() {
        assert!(f("2s 3s 4s 5h 6d", PQLStreet::Flop));
        assert!(f("2s 3s 4s 5h 6d", PQLStreet::Turn));
        assert!(f("2s 3s 4s 5h 6d", PQLStreet::River));
        assert!(!f("2s 3s 4h 5h 6d", PQLStreet::Flop));
        assert!(!f("2s 3s 4h 5h 6d", PQLStreet::Turn));
        assert!(!f("2s 3s 4h 5h 6d", PQLStreet::River));

        assert!(f("2s 3s 4h 5s 6d", PQLStreet::Turn));
        assert!(f("2s 3s 4h 5s 6d", PQLStreet::River));
        assert!(!f("2s 3s 4h 5h 6d", PQLStreet::Turn));
        assert!(!f("2s 3s 4h 5h 6d", PQLStreet::River));

        assert!(f("2s 3h 4d 5s 6s", PQLStreet::River));
        assert!(!f("2s 3h 4d 5s 6h", PQLStreet::River));
    }
}
