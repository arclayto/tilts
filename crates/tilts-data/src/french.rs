//! Ken French's data library: the canonical, free source of Fama-French factors.
//!
//! <https://mba.tuck.dartmouth.edu/pages/faculty/ken.french/data_library.html>
//!
//! Each dataset is a zip containing one CSV. The CSVs are *not* clean: a free-text
//! preamble, then monthly rows as `YYYYMM,Mkt-RF,SMB,HML,RF` in **percent**, then a
//! blank line and an annual section with the same headers. Parsing must stop at the
//! blank line and divide by 100. These two traps account for most wrong factor
//! regressions on the internet, so they get tests.

use serde::{Deserialize, Serialize};
use tilts_core::FactorPanel;

use crate::cache::Cache;
use crate::error::{Error, Result};

/// Base URL for every file in the library.
pub const BASE_URL: &str = "https://mba.tuck.dartmouth.edu/pages/faculty/ken.french/ftp/";

/// Which factor file to fetch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Dataset {
    /// Three factors (`Mkt-RF`, `SMB`, `HML`) plus `RF`.
    Ff3,
    /// Five factors (`Mkt-RF`, `SMB`, `HML`, `RMW`, `CMA`) plus `RF`.
    Ff5,
    /// Momentum (`Mom`) on its own; joined to FF5 for the six-factor spec.
    Momentum,
}

/// Which market the factors are built from.
///
/// French publishes US factors from 1926 and international sets from 1990.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Region {
    /// United States (CRSP universe).
    Us,
    /// All developed markets.
    Developed,
    /// Developed markets excluding the US.
    DevelopedExUs,
    /// Europe.
    Europe,
    /// Japan.
    Japan,
    /// Asia Pacific excluding Japan.
    AsiaPacificExJapan,
    /// North America (US + Canada).
    NorthAmerica,
    /// Emerging markets. French publishes only the five-factor set here.
    Emerging,
}

/// Observation frequency. Only `Monthly` is supported by the rest of `tilts` in v1;
/// `Daily` exists so the URL mapping is complete.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Frequency {
    /// Month-end observations.
    Monthly,
    /// Trading-day observations.
    Daily,
}

/// File name within [`BASE_URL`] for a dataset, or an error if French doesn't
/// publish that combination.
///
/// US names are verified. International names follow French's naming pattern and
/// are checked against the live site in M0; treat any failure there as a mapping
/// bug, not a network problem.
pub fn file_name(dataset: Dataset, region: Region, freq: Frequency) -> Result<String> {
    use Dataset::{Ff3, Ff5, Momentum};
    use Frequency::{Daily, Monthly};
    use Region::{AsiaPacificExJapan, Developed, DevelopedExUs, Emerging, Europe, Japan, NorthAmerica, Us};

    let name = match (region, dataset, freq) {
        (Us, Ff3, Monthly) => "F-F_Research_Data_Factors_CSV.zip".to_owned(),
        (Us, Ff3, Daily) => "F-F_Research_Data_Factors_daily_CSV.zip".to_owned(),
        (Us, Ff5, Monthly) => "F-F_Research_Data_5_Factors_2x3_CSV.zip".to_owned(),
        (Us, Ff5, Daily) => "F-F_Research_Data_5_Factors_2x3_daily_CSV.zip".to_owned(),
        (Us, Momentum, Monthly) => "F-F_Momentum_Factor_CSV.zip".to_owned(),
        (Us, Momentum, Daily) => "F-F_Momentum_Factor_daily_CSV.zip".to_owned(),
        (Emerging, Ff3, _) => {
            return Err(Error::UnsupportedDataset(
                "Emerging markets: French publishes five factors only".into(),
            ));
        }
        (region, dataset, freq) => {
            let prefix = match region {
                Developed => "Developed",
                DevelopedExUs => "Developed_ex_US",
                Europe => "Europe",
                Japan => "Japan",
                AsiaPacificExJapan => "Asia_Pacific_ex_Japan",
                NorthAmerica => "North_America",
                Emerging => "Emerging",
                Us => unreachable!("US handled above"),
            };
            let body = match dataset {
                Ff3 => "3_Factors",
                Ff5 => "5_Factors",
                Momentum => "Mom_Factor",
            };
            let suffix = match freq {
                Monthly => "",
                Daily => "_Daily",
            };
            format!("{prefix}_{body}{suffix}_CSV.zip")
        }
    };
    Ok(name)
}

/// Client for the library. Cheap to construct; holds no connection.
#[derive(Debug, Clone)]
pub struct FrenchLibrary {
    cache: Cache,
}

impl FrenchLibrary {
    /// A client that reads and writes `cache`.
    pub fn new(cache: Cache) -> Self {
        Self { cache }
    }

    /// Fetch (or load from cache) one dataset as a panel, returns in decimals.
    ///
    /// The `RF` column, where present, is kept: callers need it for excess returns.
    pub fn factors(&self, dataset: Dataset, region: Region, freq: Frequency) -> Result<FactorPanel> {
        let _ = (&self.cache, dataset, region, freq);
        todo!("M0: cache lookup → download zip → extract CSV → parse_monthly_csv")
    }

    /// The six-factor panel: FF5 inner-joined with momentum.
    pub fn ff5_mom(&self, region: Region) -> Result<FactorPanel> {
        let _ = (self, region);
        todo!("M0: factors(Ff5) ⋈ factors(Momentum) on period")
    }
}

/// Parse the monthly section of a French CSV into a panel with returns in decimals.
///
/// Stops at the first blank line after the data begins (the annual section follows),
/// and divides every value by 100.
pub fn parse_monthly_csv(text: &str) -> Result<FactorPanel> {
    let _ = text;
    todo!("M0")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn us_file_names_are_the_known_ones() {
        assert_eq!(
            file_name(Dataset::Ff3, Region::Us, Frequency::Monthly).unwrap(),
            "F-F_Research_Data_Factors_CSV.zip"
        );
        assert_eq!(
            file_name(Dataset::Ff5, Region::Us, Frequency::Monthly).unwrap(),
            "F-F_Research_Data_5_Factors_2x3_CSV.zip"
        );
        assert_eq!(
            file_name(Dataset::Momentum, Region::Us, Frequency::Monthly).unwrap(),
            "F-F_Momentum_Factor_CSV.zip"
        );
    }

    #[test]
    fn international_names_follow_the_pattern() {
        assert_eq!(
            file_name(Dataset::Ff3, Region::Europe, Frequency::Monthly).unwrap(),
            "Europe_3_Factors_CSV.zip"
        );
        assert_eq!(
            file_name(Dataset::Momentum, Region::DevelopedExUs, Frequency::Monthly).unwrap(),
            "Developed_ex_US_Mom_Factor_CSV.zip"
        );
        assert!(matches!(
            file_name(Dataset::Ff3, Region::Emerging, Frequency::Monthly),
            Err(Error::UnsupportedDataset(_))
        ));
    }
}
