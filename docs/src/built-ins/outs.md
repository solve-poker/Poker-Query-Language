# Outs

Outs functions check whether a player has live cards to a particular target on the next street.

> Open PQL currently exposes the predicate-style outs functions only. The original PQL also defined `nutHiOuts`, `outsToHandType`, and `minOutsToHandType` (returning counts); those are not yet implemented.

## `nutHi(player, street)`

`TPlayer × TStreet → TBoolean`

`true` if the player currently holds the nut hi hand on the given street. Considers known dead cards when judging which hands are still possible.

```sql
select count(nutHi(hero, flop)) as pct_flopped_nuts
from   game='holdem', hero='AwKw', villain='*', board=''
```

## `nutHiForHandType(player, street)`

`TPlayer × TStreet → TBoolean`

`true` if the player has the **best possible hand for their hand type** on the given street. For instance, with `hero='AsKh'` and a flop of `AdTd2d`, the player has `pair` and is the best possible pair (top pair, top kicker), so `nutHiForHandType(hero, flop)` is `true`.

This is useful for distinguishing "made the absolute nuts" from "made the best version of a weaker class".

## Notes

- "Unseen" deck for these functions excludes every declared player's holding, the board, and any `dead='…'` cards.
- For multi-street planning (e.g. equity on the turn given hero held a draw on the flop), combine `nutHi` with a `where` clause and an aggregate selector.
