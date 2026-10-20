//! The holdings file a user hands to `tilts analyze`.
//!
//! Format, v1: a CSV with a header and two columns.
//!
//! ```csv
//! ticker,weight
//! VTI,0.60
//! VXUS,0.25
//! AVUV,0.15
//! ```
//!
//! Weights are normalised to sum to one, so dollar amounts work too. Broker-export
//! parsing (the painful part) is explicitly out of scope until there's a user
//! asking for a specific broker.

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

/// One line of a holdings file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Holding {
    /// Ticker as the price source expects it.
    pub ticker: String,
    /// Portfolio weight after normalisation; sums to one across the file.
    pub weight: f64,
}

/// A normalised, validated set of holdings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Holdings {
    positions: Vec<Holding>,
}

impl Holdings {
    /// Build from raw `(ticker, amount)` pairs. Amounts can be weights or dollars;
    /// they are rescaled to sum to one. Fails on empty input, non-positive totals,
    /// or duplicate tickers.
    pub fn from_pairs(pairs: impl IntoIterator<Item = (String, f64)>) -> Result<Self> {
        let raw: Vec<(String, f64)> = pairs.into_iter().collect();
        if raw.is_empty() {
            return Err(Error::Holdings("no positions".into()));
        }
        let mut seen = std::collections::HashSet::new();
        for (t, _) in &raw {
            if !seen.insert(t.to_uppercase()) {
                return Err(Error::Holdings(format!("duplicate ticker {t}")));
            }
        }
        let total: f64 = raw.iter().map(|(_, w)| w).sum();
        if total.is_nan() || total <= 0.0 {
            return Err(Error::Holdings(format!(
                "weights sum to {total}, need a positive total"
            )));
        }
        let positions = raw
            .into_iter()
            .map(|(ticker, w)| Holding {
                ticker: ticker.to_uppercase(),
                weight: w / total,
            })
            .collect();
        Ok(Self { positions })
    }

    /// Parse the CSV format described in the module docs.
    pub fn from_csv(text: &str) -> Result<Self> {
        let _ = text;
        todo!("M2: header-checked CSV parse → from_pairs")
    }

    /// The positions, weights summing to one.
    pub fn positions(&self) -> &[Holding] {
        &self.positions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalises_dollar_amounts_to_weights() {
        let h = Holdings::from_pairs([("vti".to_owned(), 600.0), ("VXUS".to_owned(), 400.0)]).unwrap();
        let p = h.positions();
        assert_eq!(p[0].ticker, "VTI");
        assert!((p[0].weight - 0.6).abs() < 1e-12);
        assert!((p[1].weight - 0.4).abs() < 1e-12);
    }

    #[test]
    fn rejects_duplicates_and_empty() {
        assert!(Holdings::from_pairs([("VTI".to_owned(), 1.0), ("vti".to_owned(), 1.0)]).is_err());
        assert!(Holdings::from_pairs(std::iter::empty()).is_err());
        assert!(Holdings::from_pairs([("VTI".to_owned(), 0.0)]).is_err());
    }
}
