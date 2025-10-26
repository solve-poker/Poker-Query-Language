use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VmBinOpCmp {
    Eq,
    Ge,
    Gt,
    Le,
    Lt,
}

fn compare_num(
    lhs: VmStackValue,
    rhs: VmStackValue,
) -> Result<Option<cmp::Ordering>, InternalError> {
    let lhs = PQLNumeric::try_from(lhs)?;
    let rhs = PQLNumeric::try_from(rhs)?;

    Ok(lhs.partial_compare(rhs))
}

#[inline]
fn compare_flop_cat(
    game: PQLGame,
    lhs: PQLFlopHandCategory,
    rhs: PQLFlopHandCategory,
) -> cmp::Ordering {
    if game.is_shortdeck() {
        lhs.compare::<true>(rhs)
    } else {
        lhs.compare::<false>(rhs)
    }
}

#[inline]
fn compare_handtype(
    game: PQLGame,
    lhs: PQLHandType,
    rhs: PQLHandType,
) -> cmp::Ordering {
    if game.is_shortdeck() {
        lhs.compare::<true>(rhs)
    } else {
        lhs.compare::<false>(rhs)
    }
}

impl VmBinOpCmp {
    const fn matches(self, order: Option<cmp::Ordering>) -> bool {
        use cmp::Ordering::{Equal, Greater, Less};

        if let Some(order) = order {
            matches!(
                (self, order),
                (Self::Eq, Equal)
                    | (Self::Lt, Less)
                    | (Self::Gt, Greater)
                    | (Self::Le, Less | Equal)
                    | (Self::Ge, Equal | Greater)
            )
        } else {
            false
        }
    }

    /// # Panics
    /// compiler check ensures no panics here
    pub fn execute(self, ctx: &mut VmExecContext) -> Result<(), PQLErrorKind> {
        let rhs = ctx.stack.pop().unwrap();
        let lhs = ctx.stack.pop().unwrap();
        let is_num = lhs.is_num();

        match (self, is_num) {
            (Self::Eq, true) => {
                ctx.stack.push(self.matches(compare_num(lhs, rhs)?).into());
            }
            (Self::Eq, false) => ctx.stack.push((lhs == rhs).into()),
            _ => {
                let order = Self::compare(ctx.fn_ctx.game, lhs, rhs)?;

                ctx.stack.push(self.matches(order).into());
            }
        }

        Ok(())
    }

    pub fn resolve_type(
        self,
        lhs_type: PQLType,
        rhs_type: PQLType,
    ) -> Result<PQLType, PQLErrorKind> {
        let both_num = lhs_type.is_num() && rhs_type.is_num();
        let same = lhs_type == rhs_type;
        let lhs_cmp = matches!(
            lhs_type,
            PQLType::FLOPHANDCATEGORY
                | PQLType::HANDTYPE
                | PQLType::HIRATING
                | PQLType::RANK
        );

        if match self {
            Self::Eq => both_num || same,
            _ => both_num || (same && lhs_cmp),
        } {
            Ok(PQLType::BOOLEAN)
        } else {
            Err(PQLErrorKind::ComparisonOperationUnsupported(
                lhs_type, rhs_type,
            ))
        }
    }

    pub fn compare(
        game: PQLGame,
        lhs: VmStackValue,
        rhs: VmStackValue,
    ) -> Result<Option<cmp::Ordering>, InternalError> {
        use VmStackValue::{
            Count, Double, FlopCategory, Frac, HandType, Long, Rank, Rating,
        };

        match (lhs, rhs) {
            (Rank(lhs), Rank(rhs)) => Ok(Some(lhs.cmp(&rhs))),
            (Rating(lhs), Rating(rhs)) => Ok(Some(lhs.cmp(&rhs))),
            (HandType(lhs), HandType(rhs)) => {
                Ok(Some(compare_handtype(game, lhs, rhs)))
            }
            (FlopCategory(lhs), FlopCategory(rhs)) => {
                Ok(Some(compare_flop_cat(game, lhs, rhs)))
            }
            _ => compare_num(lhs, rhs),
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    fn assert_cmp_inner<O, I, T>(sd: bool, vals: I, op: O, expected: bool)
    where
        VmBinOp: From<O>,
        I: IntoIterator<Item = T>,
        VmStackValue: From<T>,
    {
        let mut ctx = VmExecContext::default();
        if sd {
            ctx.fn_ctx.game = PQLGame::ShortDeck;
        }
        for v in vals {
            ctx.stack.push(v.into());
        }

        let ins = VmInstruction::BinOp(op.into());
        ins.execute(&mut ctx).unwrap();

        assert_eq!(ctx.stack.pop().unwrap(), expected.into());
    }

    fn assert_cmp<O, I, T>(vals: I, op: O, expected: bool)
    where
        VmBinOp: From<O>,
        I: IntoIterator<Item = T>,
        VmStackValue: From<T>,
    {
        assert_cmp_inner(false, vals, op, expected);
    }

    fn assert_cmp_sd<O, I, T>(vals: I, op: O, expected: bool)
    where
        VmBinOp: From<O>,
        I: IntoIterator<Item = T>,
        VmStackValue: From<T>,
    {
        assert_cmp_inner(true, vals, op, expected);
    }

    #[test]
    fn test_num_lt() {
        assert_cmp([1.0, 2.0], VmBinOpCmp::Lt, true);
        assert_cmp([2.0, 1.0], VmBinOpCmp::Lt, false);
        assert_cmp(
            [PQLCardCount::from(1), PQLCardCount::from(2)],
            VmBinOpCmp::Lt,
            true,
        );
    }

    #[test]
    fn test_num_le() {
        assert_cmp([1.0, 2.0], VmBinOpCmp::Le, true);
        assert_cmp([2.0, 2.0], VmBinOpCmp::Le, true);
        assert_cmp([3.0, 2.0], VmBinOpCmp::Le, false);
    }

    #[test]
    fn test_num_gt() {
        assert_cmp([2.0, 1.0], VmBinOpCmp::Gt, true);
        assert_cmp([1.0, 2.0], VmBinOpCmp::Gt, false);
    }

    #[test]
    fn test_num_ge() {
        assert_cmp([1.0, 2.0], VmBinOpCmp::Ge, false);
        assert_cmp([2.0, 2.0], VmBinOpCmp::Ge, true);
        assert_cmp([3.0, 2.0], VmBinOpCmp::Ge, true);
    }

    #[test]
    fn test_num_eq() {
        assert_cmp([1.0, 2.0], VmBinOpCmp::Eq, false);
        assert_cmp([1.0, 1.0], VmBinOpCmp::Eq, true);
        assert_cmp(
            [PQLCardCount::from(1), PQLCardCount::from(1)],
            VmBinOpCmp::Eq,
            true,
        );
    }

    #[test]
    fn test_cmp() {
        assert_cmp([PQLRank::RK, PQLRank::RA], VmBinOpCmp::Lt, true);
        assert_cmp(
            [PQLFlopHandCategory::Nothing, PQLFlopHandCategory::Pocket12],
            VmBinOpCmp::Ge,
            false,
        );
        assert_cmp(
            [PQLHandType::Flush, PQLHandType::FullHouse],
            VmBinOpCmp::Lt,
            true,
        );
        assert_cmp(
            [mk_rating("AsKhQdJcTs"), mk_rating("AsKsQsJsTs")],
            VmBinOpCmp::Lt,
            true,
        );
    }

    #[test]
    fn test_eq() {
        assert_cmp([PQLStreet::Flop, PQLStreet::Flop], VmBinOpCmp::Eq, true);
    }

    #[test]
    fn test_shortdeck() {
        assert_cmp_sd(
            [PQLHandType::Flush, PQLHandType::FullHouse],
            VmBinOpCmp::Gt,
            true,
        );

        assert_cmp_sd(
            [PQLFlopHandCategory::Flush, PQLFlopHandCategory::FullHouse],
            VmBinOpCmp::Gt,
            true,
        );
    }

    fn assert_err<O, I, E>(vals: I, op: O, expected: E)
    where
        VmBinOp: From<O>,
        I: IntoIterator<Item = VmStackValue>,
        PQLErrorKind: From<E>,
    {
        let mut ctx = VmExecContext::default();
        for v in vals {
            ctx.stack.push(v);
        }

        let ins = VmInstruction::BinOp(op.into());

        assert_eq!(ins.execute(&mut ctx).unwrap_err(), expected.into());
    }

    #[test]
    fn test_internal() {
        assert_err(
            [sval!(@street flop), sval!(@street flop)],
            VmBinOpCmp::Lt,
            InternalError::NonNumericStackValue,
        );

        assert_err(
            [sval!(@long 1), sval!(@street flop)],
            VmBinOpCmp::Lt,
            InternalError::NonNumericStackValue,
        );

        assert_err(
            [sval!(@long 1), sval!(@street flop)],
            VmBinOpCmp::Eq,
            InternalError::NonNumericStackValue,
        );

        assert_cmp([PQLDouble::NAN, PQLDouble::NAN], VmBinOpCmp::Eq, false);
    }
}
