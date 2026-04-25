# Preflop Equity vs a Range

A common analysis: "how does my hand fare against a given range of opponent hands, before any community cards are dealt?"

## Suited AK vs a Tight Open Range

A 5%-ish opening range might be `QQ+, AwQw+, AxKy`. Hero's equity holding suited AK against that range:

```sql
select avg(equity(hero))
from   game='holdem',
       hero='AhKh',
       villain='QQ+, AwQw+, AxKy',
       board=''
```

Run it:

```bash
opql --run "select avg(equity(hero)) from game='holdem', hero='AhKh', villain='QQ+, AwQw+, AxKy', board=''"
```

Expect equity in the mid-to-high 40s — AK flips with a tight opener.

## Hero on a Range

You can also give hero a range to estimate range-vs-range equity:

```sql
select avg(equity(hero))
from   game='holdem',
       hero='99+, AwJw+',
       villain='QQ+, AxKy',
       board=''
```

The result is the combo-weighted average of hero's range equity against villain's range.

## Conditional Equity

Add a `where` clause to ask a conditional question — e.g. "given hero is dealt a specific sub-range, what's the equity?":

```sql
select avg(equity(hero))
from   game='holdem', hero='**', villain='**', board=''
where  inRange(hero, 'AwKw, AwQw, AwJw')
```

## Adding a Board

To see how equity shifts postflop, add a `board='…'`. See [Postflop Analysis](./postflop.md).
