//! Ordinary least squares on `nalgebra` matrices.
//!
//! This module is deliberately dumb: point estimates, residuals, and the pieces
//! ([`OlsFit::xtx_inv`]) that standard-error methods in [`crate::hac`] need. It
//! knows nothing about factor names.

use nalgebra::{DMatrix, DVector};

use crate::error::Result;

/// Output of a plain OLS fit, before any standard-error treatment.
#[derive(Debug, Clone, PartialEq)]
pub struct OlsFit {
    /// Coefficients, one per column of the design matrix.
    pub beta: DVector<f64>,
    /// `y - X beta`.
    pub residuals: DVector<f64>,
    /// `(XᵀX)⁻¹`, kept because every sandwich estimator needs it.
    pub xtx_inv: DMatrix<f64>,
    /// Coefficient of determination.
    pub r2: f64,
    /// `r2` adjusted for the number of regressors.
    pub adj_r2: f64,
    /// Observations.
    pub n_obs: usize,
    /// Regressors, including the intercept column if present.
    pub n_params: usize,
}

/// Fit `y ≈ X β` by least squares.
///
/// `x` must already contain an intercept column if one is wanted. Fails with
/// [`Error::TooFewObservations`](crate::Error::TooFewObservations) when `n ≤ k`
/// and [`Error::RankDeficient`](crate::Error::RankDeficient) when `XᵀX` is singular.
pub fn ols(y: &DVector<f64>, x: &DMatrix<f64>) -> Result<OlsFit> {
    let _ = (y, x);
    todo!("M0: QR-based OLS with rank check")
}

/// Classical (homoskedastic) standard errors from an [`OlsFit`].
pub fn classical_se(fit: &OlsFit) -> DVector<f64> {
    let _ = fit;
    todo!("M0: sqrt(diag(sigma² (XᵀX)⁻¹))")
}

/// Prepend a column of ones to `x`.
pub fn with_intercept(x: &DMatrix<f64>) -> DMatrix<f64> {
    let ones = DMatrix::from_element(x.nrows(), 1, 1.0);
    let mut out = DMatrix::zeros(x.nrows(), x.ncols() + 1);
    out.column_mut(0).copy_from(&ones.column(0));
    out.columns_mut(1, x.ncols()).copy_from(x);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_intercept_prepends_ones() {
        let x = DMatrix::from_row_slice(2, 1, &[5.0, 7.0]);
        let xi = with_intercept(&x);
        assert_eq!(xi, DMatrix::from_row_slice(2, 2, &[1.0, 5.0, 1.0, 7.0]));
    }
}
