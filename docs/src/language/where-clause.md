# Where Clause

The `where` clause is an optional boolean predicate that filters trials before they reach the selectors. It is evaluated **after** sampling and **before** the selector expression. Trials that fail the predicate are discarded and not counted in the aggregate.

```sql
select avg(equity(hero))
from   game='holdem', hero='AhKh', villain='**'
where  hasTopBoardRank(hero, flop)
```

This computes hero's average equity *conditional on flopping top pair*.

## What it can contain

The predicate is an arbitrary boolean expression:

- Function calls returning `TBoolean` — e.g. `nutHi(hero, flop)`, `pocketPair(hero)`.
- Comparisons — e.g. `equity(hero) > 0.5`, `handType(hero, flop) = flush`.
- Boolean composition — `and`, `or`, `not`, plus `(` `)` for grouping.

```sql
where equity(villain) > equity(hero)
  and minHandType(hero, turn, pair)
```

## When the where clause helps

A `where` clause is useful when you want a conditional probability or expected value. For example:

```sql
-- How often does hero win when villain hits the flop?
select count(wins(hero)) as heroWinsGivenVillainHits
from   game='holdem', hero='AwAx', villain='**'
where  not (handType(villain, flop) = highcard)
```

If you need the **unconditional** probability or expectation, omit `where`.

## Filtering Cost

A trial that fails the `where` predicate is wasted work — the runner counts it as a failed sample but does not feed it into selectors. If your `where` clause matches very rarely, the simulation may take a long time to gather enough successful trials. Tighten the `from` clause (e.g. fix more cards) when possible.
