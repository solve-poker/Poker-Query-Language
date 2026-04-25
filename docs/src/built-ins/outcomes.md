# Outcomes

Functions describing what happened at showdown, plus a few small helpers used to build expressions.

## Showdown outcomes

### `winsHi(player)`

`TPlayer → TBoolean`

`true` if `player` wins the entire hi pot outright (no other player has an equal or better hi hand).

```sql
select count(winsHi(hero)) as heroWins
from   game='holdem', hero='AhKh', villain='QQ+', board='Ah9s2c'
```

### `tiesHi(player)`

`TPlayer → TBoolean`

`true` if `player` ties for the hi half of the pot with at least one other player.

### `scoops(player)`

`TPlayer → TBoolean`

`true` if `player` wins the entire pot. In a Hi-only game (which is what Open PQL currently supports), `scoops` and `winsHi` mean the same thing; the function is included for compatibility and forward-compatibility with split-pot games.

> The Lo counterparts (`winsLo`, `tiesLo`) are not yet implemented.

## Range membership

### `inRange(player, range)`

`TPlayer × TRange → TBoolean`

`true` if the cards dealt to `player` fall inside the given range. Useful when conditioning on a sub-range without changing the `from` clause:

```sql
select avg(equity(hero))
from   game='holdem', hero='**', villain='**'
where  inRange(hero, 'AwKw, AxKy, QQ+')
```

Range weights (e.g. `AK@10`) are ignored if present.

## String-to-value helpers

### `toCard(s)`

`TString → TCard`

Parses a single card from its string form, e.g. `toCard('As')`. Useful for comparing the result of `turnCard()` or `riverCard()` against a specific card.

### `toRank(s)`

`TString → TRank`

Parses a rank string (e.g. `toRank('A')`, `toRank('T')`). Useful for comparing the output of `maxRank`, `minRank`, etc., against a literal.

```sql
select count(maxRank(boardRanks(river)) = toRank('A')) as pct_ace_high_river
from   game='holdem', hero='*', villain='*'
```
