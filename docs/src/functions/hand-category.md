# Hand Categories

These functions classify a player's made hand on a given street.

## `flopHandCategory(player)`

Returns the general made-hand category a player has on the flop, such as "top pair", "set", "flush draw", etc.

```sql
select count(flopHandCategory(hero) = 'topPair') as pct_toppair
from   hero='AwKw', villain='*', board='As7d2c', game='holdem'
```

## `exactFlopHandCategory(player)`

A finer-grained version — distinguishes sub-categories (e.g. "top two" vs. "bottom two", "combo draw" vs. "flush draw only").

## `minFlopHandCategory(player)`

Returns the minimum hand category a player has, combined with one draw. Useful for "at least pair + flush draw" style filters.

## `pocketPair(player)`

Boolean — true if the player holds a pocket pair.

## `overpair(player)`

Boolean — true if the player holds a pocket pair strictly higher than the highest board card.

```sql
select count(overpair(hero)) as pct_overpair
from   hero='QQ+', villain='*', board='Jc7d2s', game='holdem'
```

## Compare With

- [Rank Utilities](./rank.md) — for lower-level questions about which ranks appear in a hand.
- [Outs](./outs.md) — for the number of outs to a specific made hand.
