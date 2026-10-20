//! The one call most users make: regress an excess-return series on a factor panel.

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::model::{ModelSpec, SeMethod};
use crate::panel::{FactorPanel, MonthlySeries};
use crate::time::YearMonth;

/// One estimated coefficient with its uncertainty.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Coefficient {
    /// Point estimate.
    pub estimate: f64,
    /// Standard error under the fit's [`SeMethod`].
    pub se: f64,
    /// `estimate / se`.
    pub t_stat: f64,
}

impl Coefficient {
    /// A two-sided confidence interval using a normal approximation.
    ///
    /// `z` is the critical value (1.96 for 95%). We use the normal rather than
    /// Student-t because HAC standard errors have no exact small-sample
    /// distribution anyway; this is standard practice and we say so in the docs.
    pub fn interval(&self, z: f64) -> (f64, f64) {
        (self.estimate - z * self.se, self.estimate + z * self.se)
    }
}

/// A fitted factor regression.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FactorFit {
    /// Coefficient names, `alpha` first if the spec had an intercept, then factors
    /// in regression order.
    pub names: Vec<String>,
    /// Estimates aligned with `names`.
    pub coefficients: Vec<Coefficient>,
    /// Coefficient of determination.
    pub r2: f64,
    /// Adjusted R².
    pub adj_r2: f64,
    /// Observations used after alignment.
    pub n_obs: usize,
    /// First and last month in the aligned sample.
    pub span: (YearMonth, YearMonth),
    /// How standard errors were computed (with the resolved lag count for Newey-West).
    pub se_method: SeMethod,
    /// Residuals, aligned with the sample months. Needed for GRS and diagnostics later.
    pub residuals: Vec<f64>,
}

impl FactorFit {
    /// Look up a coefficient by name (`"alpha"`, `"Mkt-RF"`, …).
    pub fn coefficient(&self, name: &str) -> Option<&Coefficient> {
        self.names.iter().position(|n| n == name).map(|i| &self.coefficients[i])
    }

    /// The intercept, if the model had one.
    pub fn alpha(&self) -> Option<&Coefficient> {
        self.coefficient("alpha")
    }
}

/// Regress `excess_returns` on the factors named in `spec`, after aligning the two
/// on their common months.
///
/// `excess_returns` must already be in excess of the risk-free rate (see
/// [`crate::returns::excess`]); the panel's `RF` column, if any, is ignored.
pub fn fit(excess_returns: &MonthlySeries, factors: &FactorPanel, spec: &ModelSpec) -> Result<FactorFit> {
    let _ = (excess_returns, factors, spec);
    todo!("M0: align → design matrix → ols → se by spec.se → FactorFit")
}
