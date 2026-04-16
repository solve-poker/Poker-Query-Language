# Preflop Equity vs a Range

A common analysis is "how does my hand fare against a given range of opponent hands, before any community cards are dealt?"

## Suited AK vs Top 5% of Hands

A 5% opener might be `QQ+, AwQw+, AxKy`. Let's measure suited AK's equity against that range preflop.

```sql
select equity
from   hero='AhKh',
       villain='QQ+, AwQw+, AxKy',
       board='',
       game='holdem'
```

Run it:

```bash
opql --run "select equity from hero='AhKh', villain='QQ+, AwQw+, AxKy', board='', game='holdem'"
```

Expect an equity in the mid-to-high 40s — AK flips with a tight range.

## Sweeping a Range for Hero

You can also give hero a range to see a range-vs-range heatmap figure:

```sql
select equity
from   hero='99+, AwJw+',
       villain='QQ+, AxKy',
       board='',
       game='holdem'
```

The returned number is the combo-weighted average of hero's range equity against villain's range.

## Narrowing Further

Add a board to see how equity shifts postflop — see the next tutorial, [Postflop Analysis](./postflop.md).
