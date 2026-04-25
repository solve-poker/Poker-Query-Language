# Equity

Equity functions estimate each player's share of the pot at showdown, averaged over sampled runouts.

## `equity(player, street)`

`TPlayer × TStreet → TEquity`

Alias for `hvhEquity`. Returns the player's hand-vs-hand equity on the given street, against the union of every other declared player's hand on that same street.

```sql
select avg(equity(hero))
from   game='holdem', hero='AhKh', villain='QQ+', board='Ah9s2c'
```

## `hvhEquity(player, street)`

`TPlayer × TStreet → TEquity`

Hand-vs-hand equity for a specific player on a given street. The board is fixed at the requested street and the remaining cards are sampled.

```sql
select avg(hvhEquity(hero, flop)),
       avg(hvhEquity(hero, turn))
from   game='holdem', hero='AhKh', villain='QcQd', board='Ah9s2c'
```

## `minHvHEquity(player, street, threshold)`  (alias `minEquity`)

`TPlayer × TStreet × TDouble → TBoolean`

Returns `true` when the player's hand-vs-hand equity on the given street is at least `threshold`. Convenient inside `where` clauses or `count(...)` selectors:

```sql
select count(minHvHEquity(hero, flop, 0.5)) as pct_favoured
from   game='holdem', hero='AhKh', villain='**'
```

## `riverEquity(player)`

`TPlayer → TEquity`

Equity computed strictly on the river — the board is fully known, so this is exact rather than sampled. Equivalent to `equity(player, river)` when the board is five cards.

```sql
select avg(riverEquity(hero))
from   game='holdem', hero='AwKw', villain='QQ-TT', board='2s3s7d'
```

## `fractionalRiverEquity(player)`

`TPlayer → TFraction`

Like `riverEquity`, but reports the player's exact pot share as a fraction (e.g. `1/2` for a chop, `1/3` for a three-way tie) instead of a real number.

```sql
select avg(fractionalRiverEquity(villain))
from   game='holdem', hero='AhKh', villain='QQ+', board='Ah9s2c'
```

## Tips

- Equity over a range is the combo-weighted average of per-combo equities.
- For preflop all-in spots, leave `board=''`.
- The runner uses Monte Carlo sampling; re-run the query to get a fresh estimate.
- For deterministic spots (5-card board), use `riverEquity` to skip sampling overhead.
