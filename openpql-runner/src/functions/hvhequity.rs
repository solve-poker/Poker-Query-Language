use super::*;

#[pqlfn(alias = "equity")]
pub fn hvhequity(ctx: &PQLFnContext, hero: PQLPlayer, street: PQLStreet) -> PQLEquity {
    match street {
        PQLStreet::Preflop => unreachable!(),
        PQLStreet::Flop => flop_equity(ctx, hero),
        PQLStreet::Turn => turn_equity(ctx, hero),
        PQLStreet::River => river_equity(ctx, hero),
    }
}

// TODO: tmp; optimize
pub fn turn_equity(ctx: &PQLFnContext, hero: PQLPlayer) -> PQLEquity {
    let turn = ctx.get_board(PQLStreet::Turn);

    let all = if ctx.game.is_shortdeck() {
        PQLCard::all::<true>()
    } else {
        PQLCard::all::<false>()
    };

    let mut res: Vec<PQLFraction> = vec![];

    let player_cards = ctx.get_c64_players();

    for &river in all {
        if turn.contains_card(river) || player_cards.contains_card(river) {
            continue;
        }

        let board = turn.with_river(river);

        let ratings: Vec<_> = PQLPlayer::iter(ctx.n_players)
            .map(|player| ctx.eval_rating(ctx.get_player_slice(player), board))
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
        let flop = ctx.get_board(PQLStreet::Flop);

        let mut res: Vec<PQLFraction> = vec![];

        let player_cards = ctx.get_c64_players();

        for turn_river in iter {
            let turn = turn_river[0];
            let river = turn_river[1];

            if flop.contains_card(turn)
                || flop.contains_card(river)
                || player_cards.contains_card(turn)
                || player_cards.contains_card(river)
            {
                continue;
            }

            let board = flop.with_turn(turn).with_river(river);

            let ratings: Vec<_> = PQLPlayer::iter(ctx.n_players)
                .map(|player| ctx.eval_rating(ctx.get_player_slice(player), board))
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
