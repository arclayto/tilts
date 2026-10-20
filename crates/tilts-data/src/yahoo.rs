//! Yahoo Finance, via the unofficial chart endpoint. No key, no contract.
//!
//! This is the fallback for users who don't want to sign up for anything. It breaks
//! a few times a year when Yahoo changes something; when it does, the fix belongs
//! here and nowhere else. Never make it the default.

use tilts_core::{MonthlySeries, YearMonth};

use crate::error::Result;
use crate::source::PriceSource;

/// Yahoo client.
#[derive(Debug, Clone, Default)]
pub struct Yahoo;

impl PriceSource for Yahoo {
    fn name(&self) -> &'static str {
        "yahoo"
    }

    /// Uses the `adjclose` series from the v8 chart endpoint at `interval=1mo`.
    fn monthly_total_returns(&self, ticker: &str, from: YearMonth, to: YearMonth) -> Result<MonthlySeries> {
        let _ = (ticker, from, to);
        todo!("M1: GET query1.finance.yahoo.com/v8/finance/chart/{ticker}?interval=1mo → adjclose → returns")
    }
}
