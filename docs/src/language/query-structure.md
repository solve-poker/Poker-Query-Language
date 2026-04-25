# Query Structure

Every PQL query follows a `select … from … [where …]` shape, modeled after SQL.

```sql
select <selectors>
from   <bindings>
where  <predicate>          -- optional
```

A query produces one report row per selector, computed over a fixed number of Monte Carlo trials.

## Selectors

Selectors describe what you want to measure. Each selector is an aggregate over an inner expression:

```sql
select avg(equity(hero)) as heroEv,
       count(wins(hero))  as heroWinPct,
       max(handType(hero, river))
from   ...
```

PQL supports four aggregate selectors: `avg`, `count`, `max`, `min`. See [Selectors](./selectors.md) for the full semantics. Each selector can be aliased with `as` to give the report column a readable name.

## Bindings (the `from` clause)

Bindings declare the poker situation to simulate. They are written as `key='value'` pairs, separated by commas:

```sql
from game='holdem', hero='AhKh', villain='QQ+', board='Ah9s2c'
```

Reserved keys are `game`, `board`, `dead`. Any other key is treated as a player name. See [From Clause](./from-clause.md) for the full list.

## Filtering with `where`

A `where` clause filters trials before they reach the selectors. The expression must be a boolean and may reference any function, player, or street:

```sql
select avg(equity(hero))
from   game='holdem', hero='AhKh', villain='**'
where  hasTopBoardRank(hero, flop)
```

See [Where Clause](./where-clause.md).

## Identifiers and Case

Keywords (`select`, `from`, `where`, `as`, `and`, `or`, `not`) and function names are **case-insensitive**. The book uses lowercase for keywords and camelCase for function names by convention.

Card characters follow standard notation: ranks `2-9 T J Q K A` and suits `s h d c`.

## Operators

PQL inherits a subset of SQL operators:

| Class | Operators |
| --- | --- |
| Comparison | `=`, `<`, `<=`, `>`, `>=` |
| Arithmetic | `+`, `-`, `*`, `/` |
| Boolean | `and`, `or`, `not` |
| Grouping | `(`, `)` |

Not currently implemented: `<>` / `!=`, `||`, `IN`, `CASE … WHEN`, line comments (`-- …`), block comments (`/* … */`).

## Whitespace and Commas

Whitespace is free-form. Selectors and bindings are comma-separated. Trailing commas are accepted.

## Multiple Statements

A `;` separates independent statements. Each statement is parsed and run in turn, with its own report:

```sql
select equity(hero) from game='holdem', hero='AA', villain='KK';
select equity(hero) from game='holdem', hero='AA', villain='QQ';
```
