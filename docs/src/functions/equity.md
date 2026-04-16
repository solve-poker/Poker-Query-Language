# Equity

Equity functions estimate each player's share of the pot at showdown, averaged over sampled runouts.

## `equity`

Hero's equity against all declared villains.

```sql
select equity
from   hero='AhKh', villain='QQ+', board='Ah9s2c', game='holdem'
```

## `hvhEquity(p1, p2)`

Head-to-head equity between two named players. Useful when several villains are declared and you want a specific matchup.

```sql
select hvhEquity(hero, villain)
from   hero='AA', villain='KK', board='', game='holdem'
```

## `minHvHEquity(hero, villain1, villain2, …)`

The minimum head-to-head equity hero has across each listed villain. Useful for worst-case analysis against multiple opponents.

## `riverEquity`

Equity evaluated strictly on the river — i.e. no further sampling because the board is fully known. Equivalent to `equity` when `board` is 5 cards.

## `fractionalRiverEquity`

Like `riverEquity`, but awards fractional pot shares for chopped pots rather than splitting the win 50/50 per trial.

## Tips

- Equity over a range is the combo-weighted average of per-combo equities.
- For preflop all-in spots, leave `board=''`.
- The runner uses Monte Carlo sampling; re-run to get fresh estimates.
