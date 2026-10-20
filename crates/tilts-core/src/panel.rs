//! Aligned monthly data: a single series, and a panel of named factor columns.
//!
//! Alignment is the whole game in factor regressions. Both types validate on
//! construction that periods are strictly increasing, so downstream code can index
//! by position without re-checking.

use crate::error::{Error, Result};
use crate::time::YearMonth;

/// One value per month, e.g. a fund's monthly total return or a factor series.
///
/// Returns are stored as decimals (`0.0123` for 1.23%), never percentages.
#[derive(Debug, Clone, PartialEq)]
pub struct MonthlySeries {
    periods: Vec<YearMonth>,
    values: Vec<f64>,
}

impl MonthlySeries {
    /// Build a series, checking that lengths match and periods strictly increase.
    pub fn new(periods: Vec<YearMonth>, values: Vec<f64>) -> Result<Self> {
        if periods.len() != values.len() {
            return Err(Error::LengthMismatch {
                left: periods.len(),
                right: values.len(),
            });
        }
        check_strictly_increasing(&periods)?;
        Ok(Self { periods, values })
    }

    /// The months covered, in order.
    pub fn periods(&self) -> &[YearMonth] {
        &self.periods
    }

    /// The values, aligned with [`periods`](Self::periods).
    pub fn values(&self) -> &[f64] {
        &self.values
    }

    /// Number of observations.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// True if there are no observations.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// First and last month, or `None` if empty.
    pub fn span(&self) -> Option<(YearMonth, YearMonth)> {
        Some((*self.periods.first()?, *self.periods.last()?))
    }
}

/// Several factor series sharing one set of periods.
///
/// Column order is the regression order. Build one from French's files via
/// `tilts-data`, or by hand with [`FactorPanel::new`].
#[derive(Debug, Clone, PartialEq)]
pub struct FactorPanel {
    periods: Vec<YearMonth>,
    names: Vec<String>,
    columns: Vec<Vec<f64>>,
}

impl FactorPanel {
    /// Build a panel from named columns that all share `periods`.
    pub fn new(periods: Vec<YearMonth>, columns: Vec<(String, Vec<f64>)>) -> Result<Self> {
        check_strictly_increasing(&periods)?;
        let mut names = Vec::with_capacity(columns.len());
        let mut cols = Vec::with_capacity(columns.len());
        for (name, values) in columns {
            if values.len() != periods.len() {
                return Err(Error::LengthMismatch {
                    left: periods.len(),
                    right: values.len(),
                });
            }
            names.push(name);
            cols.push(values);
        }
        Ok(Self {
            periods,
            names,
            columns: cols,
        })
    }

    /// The months covered, in order.
    pub fn periods(&self) -> &[YearMonth] {
        &self.periods
    }

    /// Factor names in column order.
    pub fn names(&self) -> &[String] {
        &self.names
    }

    /// Number of observations (rows).
    pub fn len(&self) -> usize {
        self.periods.len()
    }

    /// True if there are no observations.
    pub fn is_empty(&self) -> bool {
        self.periods.is_empty()
    }

    /// A single factor's values by name.
    pub fn column(&self, name: &str) -> Result<&[f64]> {
        self.names
            .iter()
            .position(|n| n == name)
            .map(|i| self.columns[i].as_slice())
            .ok_or_else(|| Error::MissingFactor(name.to_owned()))
    }

    /// The panel restricted to the months both it and `other` cover, in order.
    ///
    /// This is the inner join every regression starts with. Returns the trimmed panel
    /// and the trimmed series.
    pub fn align_with(&self, other: &MonthlySeries) -> Result<(FactorPanel, MonthlySeries)> {
        let _ = other;
        todo!("M0: inner-join on periods")
    }
}

fn check_strictly_increasing(periods: &[YearMonth]) -> Result<()> {
    if let Some((a, b)) = periods.iter().zip(periods.iter().skip(1)).find(|(a, b)| a >= b) {
        return Err(Error::Misaligned(format!("{a} is not before {b}")));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn months(n: usize) -> Vec<YearMonth> {
        YearMonth::new(2020, 1)
            .unwrap()
            .range_inclusive(YearMonth::new(2020, 12).unwrap())
            .take(n)
            .collect()
    }

    #[test]
    fn series_rejects_length_mismatch_and_disorder() {
        assert!(matches!(
            MonthlySeries::new(months(3), vec![0.0; 2]),
            Err(Error::LengthMismatch { left: 3, right: 2 })
        ));
        let mut p = months(3);
        p.swap(0, 1);
        assert!(matches!(MonthlySeries::new(p, vec![0.0; 3]), Err(Error::Misaligned(_))));
    }

    #[test]
    fn panel_looks_up_columns_by_name() {
        let panel = FactorPanel::new(
            months(2),
            vec![("Mkt-RF".into(), vec![0.01, 0.02]), ("SMB".into(), vec![0.0, -0.01])],
        )
        .unwrap();
        assert_eq!(panel.column("SMB").unwrap(), &[0.0, -0.01]);
        assert!(matches!(panel.column("HML"), Err(Error::MissingFactor(_))));
        assert_eq!(panel.names(), ["Mkt-RF", "SMB"]);
    }
}
