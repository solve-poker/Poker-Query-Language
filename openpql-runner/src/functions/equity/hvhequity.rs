use super::*;

#[pqlfn(alias = "equity")]
pub fn hvhequity(
    ctx: &PQLFnContext,
    hero: PQLPlayer,
    street: PQLStreet,
) -> PQLEquity {
    match street {
        PQLStreet::Flop => flop_equity(ctx, hero),
        PQLStreet::Turn => turn_equity(ctx, hero),
        PQLStreet::River => river_equity(ctx, hero),
    }
}

// TODO: tmp; optimize
pub fn turn_equity(ctx: &PQLFnContext, hero: PQLPlayer) -> PQLEquity {
    let b64 = ctx.get_c64_board(PQLStreet::Turn);

    let all = if ctx.game.is_shortdeck() {
        PQLCard::all::<true>()
    } else {
        PQLCard::all::<false>()
    };

    let mut res: Vec<PQLFraction> = vec![];

    for river in all {
        if b64.contains_card(*river) {
            continue;
        }

        let mut board = b64;
        board.set(*river);

        let ratings: Vec<_> = PQLPlayer::iter(ctx.n_players)
            .map(|player| {
                ctx.game.eval_rating(ctx.get_c64_player(player), board)
            })
            .collect();

        let max = *ratings.iter().max().unwrap();

        if ratings[usize::from(hero)] == max {
            let n_winners = ratings.iter().filter(|&r| *r == max).count();

            res.push(PQLFraction::new(
                1,
                FractionInner::try_from(n_winners).unwrap(),
            ));
        } else {
            res.push(PQLFraction::zero());
        }
    }

    avg(&res)
}

// TODO: tmp; optimize
pub fn flop_equity(ctx: &PQLFnContext, hero: PQLPlayer) -> PQLEquity {
    fn inner<I: Iterator<Item = HandN<2>>>(
        ctx: &PQLFnContext,
        hero: PQLPlayer,
        iter: I,
    ) -> PQLEquity {
        let b64 = ctx.get_c64_board(PQLStreet::Flop);

        let mut res: Vec<PQLFraction> = vec![];

        for turn_river in iter {
            let turn = turn_river[0];
            let river = turn_river[1];

            if b64.contains_card(turn) || b64.contains_card(river) {
                continue;
            }

            let mut board = b64;
            board.set(turn);
            board.set(river);

            let ratings: Vec<_> = PQLPlayer::iter(ctx.n_players)
                .map(|player| {
                    ctx.game.eval_rating(ctx.get_c64_player(player), board)
                })
                .collect();

            let max = *ratings.iter().max().unwrap();

            if ratings[usize::from(hero)] == max {
                let n_winners = ratings.iter().filter(|&r| *r == max).count();

                res.push(PQLFraction::new(
                    1,
                    FractionInner::try_from(n_winners).unwrap(),
                ));
            } else {
                res.push(PQLFraction::zero());
            }
        }

        avg(&res)
    }

    if ctx.game.is_shortdeck() {
        inner(ctx, hero, HandN::<2>::iter_all::<true>())
    } else {
        inner(ctx, hero, HandN::<2>::iter_all::<false>())
    }
}

fn avg(vs: &[PQLFraction]) -> PQLEquity {
    let mut sum = 0.0;
    let mut count = 0.0;

    for v in vs {
        sum += v.to_double();
        count += 1.0;
    }

    sum / count
}
