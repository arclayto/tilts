//! Heteroskedasticity- and autocorrelation-consistent (HAC) covariance.
//!
//! Monthly fund and factor returns are heteroskedastic and mildly autocorrelated, so
//! classical OLS standard errors understate uncertainty. Newey-West (1987) is the
//! field's default fix and the one every published factor regression reports.

use nalgebra::{DMatrix, DVector};

use crate::error::Result;

/// Newey-West bandwidth rule of thumb: `floor(4 · (T/100)^(2/9))`.
///
/// This is the Newey-West (1994) rule used as the default in `EViews` and in most
/// textbook treatments. Another common rule, `floor(0.75 · T^(1/3))`, gives similar
/// values; we pick one and document it. Stata's `newey` has no default and makes
/// you choose, which is arguably more honest but not what a CLI user wants.
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
pub fn default_lags(n_obs: usize) -> usize {
    (4.0 * (n_obs as f64 / 100.0).powf(2.0 / 9.0)).floor() as usize
}

/// Newey-West covariance matrix of the OLS coefficients.
///
/// `x` is the design matrix (intercept included), `residuals` the OLS residuals,
/// `lags` the Bartlett-kernel truncation. Returns the `k × k` coefficient covariance;
/// standard errors are the square roots of its diagonal.
pub fn newey_west_cov(x: &DMatrix<f64>, residuals: &DVector<f64>, lags: usize) -> Result<DMatrix<f64>> {
    let _ = (x, residuals, lags);
    todo!("M0: (XᵀX)⁻¹ S (XᵀX)⁻¹ with Bartlett-weighted autocovariances")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_lags_matches_textbook_values() {
        assert_eq!(default_lags(100), 4);
        assert_eq!(default_lags(36), 3);
        assert_eq!(default_lags(500), 5);
    }
}
