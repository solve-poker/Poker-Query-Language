# Query Structure

Every PQL query follows a `select … from …` shape, modeled after SQL.

```sql
select <selectors>
from   <bindings>
```

## Selectors

Selectors are the things you want to measure. Each selector can optionally be aliased with `as`:

```sql
select equity, avg(boardsuitcount(river)) as suits
from   ...
```

Selectors generally fall into three categories:

1. **Direct values** — e.g. `equity`, a scalar produced per trial.
2. **Aggregates** — e.g. `avg(expr)`, `count(expr)`, reducing across trials.
3. **Boolean predicates** — used inside aggregates to count probabilities.

## Bindings (the `from` clause)

Bindings declare the poker situation to simulate. The most common ones are:

| Binding | Meaning |
| --- | --- |
| `hero='…'` | Hero's exact cards or range |
| `villain='…'` | Villain's cards or range (multiple villains allowed) |
| `board='…'` | Community cards dealt so far |
| `game='holdem'` | Which poker variant to play |

See [From Clause](./from-clause.md) for the full list and [Range Notation](./ranges.md) for how the string values are parsed.

## Case Sensitivity

Keywords (`select`, `from`, `as`, function names) are case-insensitive. Card and suit literals follow standard notation: rank characters `2-9, T, J, Q, K, A` and suit characters `s, h, d, c`.

## Whitespace and Commas

Whitespace is free-form. Selectors and bindings are separated by commas. Trailing commas are tolerated.
