//! Return arithmetic: excess returns, portfolio aggregation, compounding.

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::panel::MonthlySeries;

/// How a multi-asset portfolio's weights evolve between months.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Rebalance {
    /// Weights reset to target every month. Matches how factor portfolios are
    /// usually analysed and is the conservative default.
    #[default]
    Monthly,
    /// Weights drift with returns from the initial allocation.
    BuyAndHold,
}

/// `returns - rf`, elementwise. Both inputs must cover identical months.
pub fn excess(returns: &MonthlySeries, rf: &MonthlySeries) -> Result<MonthlySeries> {
    if returns.periods() != rf.periods() {
        return Err(Error::Misaligned(
            "returns and risk-free series cover different months".into(),
        ));
    }
    let values = returns.values().iter().zip(rf.values()).map(|(r, f)| r - f).collect();
    MonthlySeries::new(returns.periods().to_vec(), values)
}

/// Combine asset return series into one portfolio series.
///
/// `weights` must sum to one (long-only is not required). All series must already
/// be aligned to the same months.
pub fn portfolio(weights: &[f64], assets: &[MonthlySeries], rebalance: Rebalance) -> Result<MonthlySeries> {
    let _ = (weights, assets, rebalance);
    todo!("M1: weighted monthly aggregation under each rebalance rule")
}

/// Geometric compounding of a return slice: `∏(1 + r) − 1`.
pub fn compound(returns: &[f64]) -> f64 {
    returns.iter().fold(1.0, |acc, r| acc * (1.0 + r)) - 1.0
}

/// Annualise a monthly return.
pub fn annualise_monthly(r: f64) -> f64 {
    (1.0 + r).powi(12) - 1.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::YearMonth;

    fn series(vals: &[f64]) -> MonthlySeries {
        let start = YearMonth::new(2024, 1).unwrap();
        let periods: Vec<_> = std::iter::successors(Some(start), |m| Some(m.next()))
            .take(vals.len())
            .collect();
        MonthlySeries::new(periods, vals.to_vec()).unwrap()
    }

    #[test]
    fn excess_subtracts_rf() {
        let ex = excess(&series(&[0.02, -0.01]), &series(&[0.004, 0.004])).unwrap();
        assert!((ex.values()[0] - 0.016).abs() < 1e-12);
        assert!((ex.values()[1] - -0.014).abs() < 1e-12);
    }

    #[test]
    fn excess_rejects_misaligned_months() {
        let rf = MonthlySeries::new(vec![YearMonth::new(2023, 12).unwrap()], vec![0.0]).unwrap();
        assert!(matches!(excess(&series(&[0.01]), &rf), Err(Error::Misaligned(_))));
    }

    #[test]
    fn compounding_and_annualising() {
        assert!((compound(&[0.1, 0.1]) - 0.21).abs() < 1e-12);
        assert!((annualise_monthly(0.01) - 0.126_825_030_131_969_7).abs() < 1e-12);
    }
}
