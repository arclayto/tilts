//! Regression specifications: which factors, which standard errors.

use serde::{Deserialize, Serialize};

/// Which factor series a regression uses.
///
/// Names follow Ken French's column headers so a [`FactorPanel`](crate::FactorPanel)
/// built from his files needs no renaming.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FactorSet {
    /// Market excess return only (`Mkt-RF`).
    Capm,
    /// Fama-French three-factor: `Mkt-RF`, `SMB`, `HML`.
    Ff3,
    /// Fama-French five-factor: `Mkt-RF`, `SMB`, `HML`, `RMW`, `CMA`.
    Ff5,
    /// Five-factor plus momentum (`Mom`). The common "six-factor" spec.
    Ff5Mom,
    /// Any named factors supplied by the caller, in regression order.
    Custom(Vec<String>),
}

impl FactorSet {
    /// Factor column names in regression order (intercept excluded).
    pub fn factor_names(&self) -> Vec<String> {
        let fixed: &[&str] = match self {
            Self::Capm => &["Mkt-RF"],
            Self::Ff3 => &["Mkt-RF", "SMB", "HML"],
            Self::Ff5 => &["Mkt-RF", "SMB", "HML", "RMW", "CMA"],
            Self::Ff5Mom => &["Mkt-RF", "SMB", "HML", "RMW", "CMA", "Mom"],
            Self::Custom(names) => return names.clone(),
        };
        fixed.iter().map(|s| (*s).to_owned()).collect()
    }

    /// Number of factors (regressors excluding the intercept).
    pub fn len(&self) -> usize {
        match self {
            Self::Capm => 1,
            Self::Ff3 => 3,
            Self::Ff5 => 5,
            Self::Ff5Mom => 6,
            Self::Custom(names) => names.len(),
        }
    }

    /// True for an empty `Custom` set.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// How coefficient standard errors are computed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SeMethod {
    /// Classical OLS standard errors. Assumes homoskedastic, serially uncorrelated
    /// errors — which monthly fund returns violate. Offered for comparison only.
    Ols,
    /// Newey-West (1987) HAC standard errors with a Bartlett kernel.
    ///
    /// `lags: None` selects the bandwidth with [`crate::hac::default_lags`].
    NeweyWest {
        /// Explicit lag truncation, or `None` for the data-driven default.
        lags: Option<usize>,
    },
}

impl Default for SeMethod {
    /// Newey-West with automatic bandwidth: the honest default.
    fn default() -> Self {
        Self::NeweyWest { lags: None }
    }
}

/// A complete regression specification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelSpec {
    /// Regressors.
    pub factors: FactorSet,
    /// Standard-error treatment.
    pub se: SeMethod,
    /// Whether to include an intercept (alpha). Almost always `true`.
    pub intercept: bool,
}

impl ModelSpec {
    /// A spec with the given factors, Newey-West errors, and an intercept.
    pub fn new(factors: FactorSet) -> Self {
        Self {
            factors,
            se: SeMethod::default(),
            intercept: true,
        }
    }

    /// Override the standard-error method.
    #[must_use]
    pub fn with_se(mut self, se: SeMethod) -> Self {
        self.se = se;
        self
    }

    /// Names of every coefficient this spec will produce, intercept first if present.
    pub fn coefficient_names(&self) -> Vec<String> {
        let mut names = Vec::with_capacity(self.factors.len() + 1);
        if self.intercept {
            names.push("alpha".to_owned());
        }
        names.extend(self.factors.factor_names());
        names
    }
}

impl Default for ModelSpec {
    /// Fama-French three-factor with Newey-West errors.
    fn default() -> Self {
        Self::new(FactorSet::Ff3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factor_names_match_french_headers() {
        assert_eq!(FactorSet::Ff3.factor_names(), ["Mkt-RF", "SMB", "HML"]);
        assert_eq!(FactorSet::Ff5Mom.len(), 6);
        assert_eq!(FactorSet::Ff5Mom.factor_names().last().map(String::as_str), Some("Mom"));
    }

    #[test]
    fn coefficient_names_put_alpha_first() {
        let spec = ModelSpec::new(FactorSet::Capm);
        assert_eq!(spec.coefficient_names(), ["alpha", "Mkt-RF"]);
        let no_alpha = ModelSpec {
            intercept: false,
            ..spec
        };
        assert_eq!(no_alpha.coefficient_names(), ["Mkt-RF"]);
    }

    #[test]
    fn default_se_is_newey_west_auto() {
        assert_eq!(SeMethod::default(), SeMethod::NeweyWest { lags: None });
    }
}
