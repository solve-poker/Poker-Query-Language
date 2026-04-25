# Range Notation

Ranges describe sets of starting hands. They appear on the right-hand side of any player binding, on the `board=` binding, and inside functions like `inRange` and `boardInRange`.

> **Heads-up:** Open PQL implements only the **generic** range syntax (variable-suit notation). The **classic** shorthand (`AKs`, `AKo`, `99+`) used by some other tools is **not yet implemented**.

The grammar lives in the `openpql-range-parser` crate.

## Concrete Cards

Use a rank (`2-9 T J Q K A`) followed by a concrete suit (`s h d c`) to fix a single card:

```text
As           -- Ace of spades
AsKh         -- Ace of spades + King of hearts
```

A two-card hand for Hold'em or Short Deck is two such cards juxtaposed; an Omaha hand is four cards.

## Suit Variables

Suits can be left abstract using **suit variables** `w x y z`. The same letter means the same (still-unspecified) suit, different letters mean **different** suits.

| Notation | Meaning |
| -------- | ------- |
| `AwKw`   | A and K of the same suit (suited AK) |
| `AxKy`   | A and K of different suits (offsuit AK) |
| `AK`     | Any AK (suited or offsuit) |
| `TT`     | Any pocket tens |
| `**`     | Any two cards (Hold'em) |
| `*`      | Wildcard rank (matches any rank) |

`*` may also stand in for an entire wildcard card — e.g. `A*` means "an Ace plus any card".

## Spans

A `+` extends the leading shape upward; a `-` between two shapes denotes an inclusive interval going **downward** from the larger to the smaller.

| Notation         | Meaning |
| ---------------- | ------- |
| `QQ+`            | Pocket pairs QQ or better |
| `88-55`          | Pocket pairs 88 down to 55 |
| `AwJw+`          | Suited aces from AJ up |
| `KwQw-KwTw`      | Suited kings from KQ down to KT |

## Lists

`[a, b, c]` is an alternation that fills one slot. Combine with another card for products:

```text
[2,4,6,8,T]A     -- A2, A4, A6, A8, AT
A[2,3,4]         -- A2, A3, A4
```

You may also write `[span]` to embed a span literal, e.g. `[QQ+]`.

## Combining Many Atoms

Comma-separated terms in the same string union into a single range:

```text
AA, KK, AwKw, 77-55
```

## Boards

The `board=` binding accepts the same range syntax. A wholly concrete value (`'Ah9s2c'`) pins the flop; a partial pattern leaves some cards generic:

```text
board='Aw9x2y'   -- any rainbow flop with ranks A, 9, 2
```

## Conflicts and Blockers

Combos that collide with already-known cards (other players' holdings, the board, or `dead` cards) are excluded automatically during sampling. You don't need to subtract blockers by hand.

## Errors

A malformed range surfaces as a parse error at query evaluation time, with the offending span in the error report.
