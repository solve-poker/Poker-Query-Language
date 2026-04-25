# Boards and Streets

PQL simulates complete runouts, so it always has a notion of the **current street** based on how many cards `board=` provides.

## Streets

| Street    | Board size | Sampled per trial |
| --------- | ---------- | ----------------- |
| `preflop` | 0          | flop + turn + river |
| `flop`    | 3          | turn + river |
| `turn`    | 4          | river |
| `river`   | 5          | nothing — fully deterministic |

Street identifiers (`preflop`, `flop`, `turn`, `river`) are case-insensitive bare keywords — not strings. They appear as arguments to many functions.

## Referencing Streets in Functions

Most board-aware functions take a street so you can ask about the board as it will look on a future card:

```sql
select avg(boardSuitCount(river))
from   game='holdem', hero='As9s', villain='*', board='2s3sJh'
```

`river` here means the completed five-card board, even though the simulation starts on the flop.

## Fixed Boards

If `board` is a full five-card string the runner samples no community cards — the query becomes a deterministic evaluation, useful for checking concrete spots:

```sql
select equity(hero)
from   game='holdem', hero='AhKh', villain='QcQd', board='Qs9h2c7d3d'
```

## Board Patterns

A partial board pattern (with suit variables or `*`) randomises the unspecified parts:

```sql
board='Aw9x2y'    -- any rainbow A-9-2 flop
```

See [Range Notation](./ranges.md) for the full pattern syntax.

## Dead Cards

Any card mentioned in a player range, in the `board`, or in `dead='…'` is removed from the deck for the rest of the deal. This prevents impossible combinations from being generated.

## Game-Specific Notes

- **Hold'em / Short Deck**: 5-card community board, all from the same deck the players draw from.
- **Omaha**: same 5-card community board; players must use **exactly two** of their four hole cards.
- **Short Deck**: 36-card deck, so fewer possible boards and faster enumeration on river-locked queries.
