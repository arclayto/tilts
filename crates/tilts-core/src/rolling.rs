//! Rolling-window regressions, for showing how exposures drift over time.

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::model::ModelSpec;
use crate::panel::{FactorPanel, MonthlySeries};
use crate::regression::FactorFit;
use crate::time::YearMonth;

/// Window geometry for [`rolling_fit`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollingSpec {
    /// Observations per window. 36 and 60 months are the conventional choices.
    pub window: usize,
    /// Months to advance between windows. `1` gives the smoothest picture.
    pub step: usize,
}

impl Default for RollingSpec {
    /// 36-month windows, advanced one month at a time.
    fn default() -> Self {
        Self { window: 36, step: 1 }
    }
}

/// One window's result, keyed by the window's last month.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RollingPoint {
    /// Last month in the window.
    pub end: YearMonth,
    /// The regression over that window.
    pub fit: FactorFit,
}

/// Fit `spec` over every window described by `rolling`.
///
/// Each window is a full [`crate::fit()`], so Newey-West lags are resolved per window
/// (which, with `lags: None`, means every window uses the same bandwidth).
pub fn rolling_fit(
    excess_returns: &MonthlySeries,
    factors: &FactorPanel,
    spec: &ModelSpec,
    rolling: RollingSpec,
) -> Result<Vec<RollingPoint>> {
    let _ = (excess_returns, factors, spec, rolling);
    todo!("M1: slide windows over the aligned sample and collect fits")
}
