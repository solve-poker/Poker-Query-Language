use super::*;

#[pqlfn]
pub fn rainbow_board(ctx: &PQLFnContext, street: PQLStreet) -> PQLBoolean {
    // River will always be false
    board_suit_count(ctx, street) == street.board_card_count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_rainbow_board_flop(ctx: TestPQLFnContext) -> TestResult {
        let ctx = ctx.as_ctx();

        let street = PQLStreet::Flop;
        let expected = count_suits(ctx.get_board_slice(street)) == 3;
        let res = rainbow_board(&ctx, street);

        TestResult::from_bool(res == expected)
    }

    #[quickcheck]
    fn test_rainbow_board_turn(ctx: TestPQLFnContext) -> TestResult {
        let ctx = ctx.as_ctx();

        let street = PQLStreet::Turn;
        let expected = count_suits(ctx.get_board_slice(street)) == 4;
        let res = rainbow_board(&ctx, street);

        TestResult::from_bool(res == expected)
    }

    #[quickcheck]
    fn test_rainbow_board_river(ctx: TestPQLFnContext) -> TestResult {
        TestResult::from_bool(!rainbow_board(&ctx.as_ctx(), PQLStreet::River))
    }
}
