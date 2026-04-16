# Boards and Streets

PQL simulates complete runouts, so it always has a concept of "current street" based on how many cards you provided in `board=`.

## Streets

| Street | Board size | Cards sampled each trial |
| --- | --- | --- |
| Preflop | 0 | flop + turn + river |
| Flop | 3 | turn + river |
| Turn | 4 | river |
| River | 5 | none — deterministic |

## Referencing Streets in Functions

Many functions take a street argument so you can ask about the board as it will look on a future street:

```sql
select avg(boardsuitcount(river))
from   hero='As9s', villain='*', board='2s3sJh', game='holdem'
```

Here `river` refers to the completed 5-card board, even though the simulation starts on the flop.

Typical street selectors accepted by board-aware functions:

- `flop` — the first three community cards
- `turn` — the fourth card
- `river` — the fifth card (or the complete 5-card board, depending on the function)

## Fixed Boards

If `board` is a full 5-card string, no cards are sampled. The query becomes a deterministic evaluation, useful for checking concrete spots.

## Dead Cards

Any card in `hero`, `villain`, or `board` is removed from the deck for the rest of the deal. This prevents impossible combinations from being generated.
