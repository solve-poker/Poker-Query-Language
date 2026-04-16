# Outs

Outs functions count how many cards in the remaining deck would improve a player's hand to a specific target.

## `nutHi(player, street)`

The number of unseen cards that would give `player` the nuts on the next street.

```sql
select avg(nutHi(hero, turn))
from   hero='AhKh', villain='*', board='Qh7h2c', game='holdem'
```

## `nutHiForHandType(player, street, handType)`

Same as `nutHi`, but restricted to a specific made-hand target (e.g. "nut flush", "nut straight"). Useful when you care about a particular draw category.

## Notes

- "Unseen" means: not in `hero`, `villain`, or the known board.
- For multi-villain spots, unseen still excludes every declared player's holding.
- Outs are computed on the exact game being played, so short-deck and Omaha use their respective deck and hand rules.
