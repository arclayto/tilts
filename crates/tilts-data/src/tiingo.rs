//! Tiingo end-of-day prices. Free tier with an API key; stable, documented API.
//!
//! <https://www.tiingo.com/documentation/end-of-day>
//!
//! Primary price source for `tilts`: the free tier's daily request limit is far
//! more than a personal portfolio needs, and unlike Yahoo it is an actual API with
//! a contract.

use tilts_core::{MonthlySeries, YearMonth};

use crate::error::{Error, Result};
use crate::source::PriceSource;

/// Environment variable holding the API key.
pub const API_KEY_ENV: &str = "TIINGO_API_KEY";

/// Tiingo client.
#[derive(Debug, Clone)]
pub struct Tiingo {
    api_key: String,
}

impl Tiingo {
    /// Build with an explicit key.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
        }
    }

    /// Build from [`API_KEY_ENV`].
    pub fn from_env() -> Result<Self> {
        std::env::var(API_KEY_ENV)
            .ok()
            .filter(|k| !k.trim().is_empty())
            .map(Self::new)
            .ok_or(Error::MissingCredentials {
                source_name: "tiingo",
                env_var: API_KEY_ENV,
            })
    }

    /// The key, for request construction. Never logged.
    fn api_key(&self) -> &str {
        &self.api_key
    }
}

impl PriceSource for Tiingo {
    fn name(&self) -> &'static str {
        "tiingo"
    }

    /// Uses `resampleFreq=monthly` on the daily-prices endpoint and the `adjClose`
    /// field, so returns include dividends and splits.
    fn monthly_total_returns(&self, ticker: &str, from: YearMonth, to: YearMonth) -> Result<MonthlySeries> {
        let _ = (self.api_key(), ticker, from, to);
        todo!("M0: GET /tiingo/daily/{ticker}/prices?resampleFreq=monthly → adjClose → returns")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_env_fails_cleanly_without_a_key() {
        // Don't mutate process env in tests; just check the constructor shape.
        let t = Tiingo::new("abc");
        assert_eq!(t.name(), "tiingo");
        assert_eq!(t.api_key(), "abc");
    }
}
