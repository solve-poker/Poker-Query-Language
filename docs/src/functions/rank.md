# Rank Utilities

Rank utilities let you reason about which ranks (2-A) appear in a hand or on the board.

## Set-returning

| Function | Meaning |
| --- | --- |
| `boardRanks(street)` | Ranks on the board at the given street |
| `handRanks(player)` | Ranks in a player's hole cards |
| `duplicatedBoardRanks(street)` | Ranks appearing more than once on the board |
| `duplicatedHandRanks(player)` | Ranks appearing more than once in a hand |
| `intersectingHandRanks(player, street)` | Hand ranks that also appear on the board |
| `nonIntersectingHandRanks(player, street)` | Hand ranks that do not appear on the board |

## Scalar

| Function | Meaning |
| --- | --- |
| `maxRank(ranks)` | Highest rank in the set |
| `minRank(ranks)` | Lowest rank in the set |
| `nthRank(ranks, n)` | The n-th highest rank (1-indexed) |
| `rankCount(ranks)` | Cardinality of the rank set |
| `handBoardIntersections(player, street)` | How many hand ranks appear on the board |

## Predicate

| Function | Meaning |
| --- | --- |
| `hasTopBoardRank(player, street)` | Player has at least one card matching the top board rank |
| `hasSecondBoardRank(player, street)` | Player has at least one card matching the second-highest board rank |

## Example

Frequency hero flops top pair:

```sql
select count(hasTopBoardRank(hero, flop)) as pct_toppair
from   hero='AxKy', villain='*', board='', game='holdem'
```
