# tilts

Factor-exposure analysis for real portfolios. Hand `tilts` your holdings and it
regresses them on Fama-French factors and tells you what you actually own:
market beta, size, value, profitability, investment and momentum loadings, the
alpha left over — and, unlike most tools, **how uncertain every one of those
numbers is**.

```text
$ tilts analyze holdings.csv --model ff5mom --rolling 36

120 obs, 2015-01 to 2024-12, R² 0.981 (adj 0.980), NeweyWest { lags: Some(4) }
┌────────┬────────────┬───────┬───────┬────────────────┐
│        │ estimate   │ s.e.  │ t     │ 95% interval   │
├────────┼────────────┼───────┼───────┼────────────────┤
│ alpha  │ -0.41%/yr  │ 0.001 │ -0.38 │ [-2.5%, 1.7%]  │
│ Mkt-RF │ 0.982      │ 0.021 │ 46.8  │ [0.941, 1.023] │
│ SMB    │ 0.113      │ 0.044 │ 2.57  │ [0.027, 0.199] │
│ HML    │ 0.206      │ 0.051 │ 4.04  │ [0.106, 0.306] │
│ …      │            │       │       │                │
└────────┴────────────┴───────┴───────┴────────────────┘
```

(Illustrative output. The numbers are made up; the format is the target.)

> **Status: scaffold.** The crate layout, public types and CLI surface are in
> place and compile; the regressions themselves are `todo!()` until milestone M0
> below. Nothing is published to crates.io yet.

## Why another one

Factor regressions are a solved problem on paper and a mess in practice:

- The free web tools that did this well have moved behind paywalls.
- The ones that remain report point estimates with no error bars. A 36-month
  regression gives you a value loading of 0.2 ± 0.3; printing "0.2" is a lie.
- Everybody's hand-rolled notebook disagrees on standard errors, rebalancing,
  alignment and percent-vs-decimal, and nobody can tell whose is right.

`tilts` picks one set of conventions, documents each choice, validates against
an independent implementation in CI, and shows confidence intervals everywhere.

## Layout

| Crate | What it is | Depends on |
|---|---|---|
| [`tilts-core`](crates/tilts-core) | The math. OLS, Newey-West HAC errors, rolling fits, model specs, aligned panels. Plain `f64` and `nalgebra`; no I/O. | `nalgebra` |
| [`tilts-data`](crates/tilts-data) | The plumbing. Ken French's factor files, vendor price data behind a `PriceSource` trait, a Parquet cache, the holdings file. | `tilts-core`, `polars`, `ureq` |
| [`tilts`](crates/tilts) | The CLI. Argument parsing, orchestration, terminal rendering. | both |

The split is deliberate: `tilts-core` is meant to be reusable by things that are
not this tool (a future Python binding, an anomaly-replication CLI), so it must
never learn what a ticker or a vendor is.

## Roadmap

- **M0 — prove the math.** Single ticker on FF3, monthly. OLS + Newey-West
  implemented; French CSV parser with tests for its two traps (percent units,
  the annual section). Validated in CI against R's `lm` + `sandwich::NeweyWest`
  on a checked-in fixture. Sanity check: a total-market ETF gives β ≈ 1, α ≈ 0.
- **M1 — make it a portfolio.** Weighted holdings, FF5 + momentum, monthly vs
  buy-and-hold rebalancing, rolling windows, confidence intervals on everything.
- **M2 — make it a tool.** Holdings CSV, cache with staleness checks,
  international factor sets (developed, Europe, Japan, …), self-contained HTML
  report alongside the terminal table.
- **M3 — open the core.** PyO3 bindings for `tilts-core`; the sorts /
  Fama-MacBeth / GRS machinery that an academic workflow needs.

## Design decisions (and why)

- **Monthly only in v1.** French's canonical factors are monthly; daily
  regressions need asynchronous-trading-day handling that isn't worth it yet.
- **Newey-West by default,** with the `floor(4·(T/100)^(2/9))` bandwidth rule.
  Classical OLS errors are available with `--se ols` for comparison only.
- **Total return ≈ adjusted close.** Every free vendor gives adjusted close, not
  reported total return. The approximation is standard and documented on the
  `PriceSource` trait; don't let a user discover it by surprise.
- **Price vendors are a trait.** Tiingo (free key, stable API) is the default,
  Yahoo (no key, breaks a few times a year) the fallback. A vendor outage must
  never be an architecture problem.
- **Normal, not Student-t, intervals.** HAC errors have no exact small-sample
  distribution; `z = 1.96` is what the literature reports.
- **Alpha is printed annualised,** loadings raw. Those are the units people
  reason in.

## Holdings file

```csv
ticker,weight
VTI,0.60
VXUS,0.25
AVUV,0.15
```

Weights are normalised, so dollar amounts work. Broker exports are out of scope
until a specific broker is requested.

## Data sources

- Factors: [Ken French's data library](https://mba.tuck.dartmouth.edu/pages/faculty/ken.french/data_library.html), free, no key.
- Prices: [Tiingo](https://www.tiingo.com/) (set `TIINGO_API_KEY`) or Yahoo Finance (unofficial).

Everything downloaded is cached under the platform data directory
(`tilts cache path` prints it; `tilts cache clear` wipes it).

## Developing

```sh
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features
cargo fmt --all --check
```

MSRV is 1.95 (edition 2024), pinned by what the `polars` dependency graph
requires; expect it to move with Polars releases. Clippy runs with `pedantic`
on; the handful of pedantic lints we disagree with are listed in the workspace
`Cargo.toml`.

## License

Dual-licensed under MIT or Apache-2.0, at your option.
