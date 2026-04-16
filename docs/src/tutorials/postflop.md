# Postflop Analysis

Once a flop, turn, or river is on the table you can ask more specific questions — frequencies of made hands, equity by street, and so on.

## Set on the Flop

How often does hero flop a set with a pocket pair?

```sql
select count(flopHandCategory(hero) = 'set') as pct_set
from   hero='TT', villain='*', board='', game='holdem'
```

`board=''` means we sample every possible flop. `count(predicate)` returns the probability the predicate holds.

## Equity Given a Draw

Hero has a flush draw on the flop; what's the showdown equity?

```sql
select equity
from   hero='AhKh', villain='AA-JJ', board='Qh7h2c', game='holdem'
```

## Turn Improvement

Conditional probability: given hero holds overcards on the flop, how often does the turn bring a pair?

```sql
select count(hasTopBoardRank(hero, turn)) as pct_pair_turn
from   hero='AxKy', villain='*', board='8s5d2c', game='holdem'
```

## Combining Filters

Combine board texture with hand categories — for example, overpair equity on non-paired boards:

```sql
select avg(riverEquity)
from   hero='QQ', villain='*', board='Jc7d2s', game='holdem'
```

From here, add `count(overpair(hero))` or `count(pairedBoard(river))` selectors to explore the distribution.
