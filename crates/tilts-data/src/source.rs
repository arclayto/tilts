//! The vendor-agnostic interface for security price data.

use tilts_core::{MonthlySeries, YearMonth};

use crate::error::Result;

/// A provider of monthly total returns for tickers.
///
/// Implementations live in [`crate::tiingo`] and [`crate::yahoo`]. The CLI picks one
/// from `--source` and wraps it in the cache; nothing else in the codebase should
/// know which vendor is behind a series.
pub trait PriceSource {
    /// Short, stable identifier (`"tiingo"`, `"yahoo"`). Used in cache keys and output.
    fn name(&self) -> &'static str;

    /// Monthly total returns for `ticker`, inclusive of `from..=to`, as decimals.
    ///
    /// "Total return" means dividends reinvested. Every vendor we support only
    /// offers an *adjusted close*, so this is the standard retail approximation:
    /// `adj_close[t] / adj_close[t-1] - 1` on month-end observations. It is close to,
    /// but not exactly, the fund's reported total return. Implementations must
    /// document any further deviation.
    ///
    /// Months with no data (fund not yet launched, delisted) are simply absent from
    /// the result; callers align on the intersection.
    fn monthly_total_returns(&self, ticker: &str, from: YearMonth, to: YearMonth) -> Result<MonthlySeries>;
}
