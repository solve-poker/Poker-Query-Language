# Range Notation

Ranges describe sets of starting hands. Open PQL uses a **generic, variable-based** notation (not the classic `AKs`/`AKo` shorthand).

## Suit Variables

Suits are written as `w`, `x`, `y`, `z`. Same letter = same suit. Different letters = different suits.

- `AwKw` — Ace and King, **same suit** (suited AK)
- `AxKy` — Ace and King, **different suits** (offsuit AK)
- `AK` — Any AK (suited or offsuit)

Concrete suits (`s`, `h`, `d`, `c`) lock to a specific card:

- `AsKh` — Exactly the Ace of spades and King of hearts

## Atoms

| Notation | Meaning |
| --- | --- |
| `AsKh` | Exact two cards |
| `AwKw` | Suited AK |
| `AxKy` | Offsuit AK |
| `AK` | Any AK |
| `TT` | Any pocket tens |
| `*` | Any two cards |

## Spans

| Notation | Meaning |
| --- | --- |
| `QQ+` | Pocket pairs QQ or better |
| `88-55` | Pocket pairs from 88 down to 55 |
| `AwJw+` | Suited aces from AJ up |
| `KwQw-KwTw` | Suited kings from KQ down to KT |

## Lists

`[2,4,6,8,T]A` expands to `A2, A4, A6, A8, AT`.

## Combining

Combine atoms with commas in a single quoted string:

```
AA, KK, AwKw, 77-55
```

## Conflicts with the Board

Combos that collide with known cards (other players' holdings or the board) are excluded automatically during simulation. You don't need to subtract blockers by hand.

## Related Crate

The grammar lives in [`openpql-range-parser`](../reference/api.md). Invalid ranges surface as parse errors at query evaluation time.
