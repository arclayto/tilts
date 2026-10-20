//! Calendar months. Monthly is the native resolution of `tilts` v1, so this is the
//! only time type the core needs; no `chrono`/`time` dependency.

use std::fmt;
use std::str::FromStr;

use crate::error::Error;

/// A calendar month, e.g. `2024-03`.
///
/// Ordered chronologically. Parses from and prints as `YYYY-MM`, which is also the
/// format Ken French's data library uses (as `YYYYMM`; see [`YearMonth::from_yyyymm`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct YearMonth {
    year: i32,
    month: u8,
}

impl YearMonth {
    /// Build from a year and a 1-based month. Returns `None` if `month` is not in `1..=12`.
    pub fn new(year: i32, month: u8) -> Option<Self> {
        (1..=12).contains(&month).then_some(Self { year, month })
    }

    /// Parse the `YYYYMM` integer form used by French's CSV files (e.g. `202403`).
    pub fn from_yyyymm(v: u32) -> Option<Self> {
        let year = i32::try_from(v / 100).ok()?;
        let month = u8::try_from(v % 100).ok()?;
        Self::new(year, month)
    }

    /// Calendar year.
    pub fn year(self) -> i32 {
        self.year
    }

    /// 1-based month.
    pub fn month(self) -> u8 {
        self.month
    }

    /// The following month.
    #[must_use]
    pub fn next(self) -> Self {
        if self.month == 12 {
            Self {
                year: self.year + 1,
                month: 1,
            }
        } else {
            Self {
                year: self.year,
                month: self.month + 1,
            }
        }
    }

    /// The preceding month.
    #[must_use]
    pub fn prev(self) -> Self {
        if self.month == 1 {
            Self {
                year: self.year - 1,
                month: 12,
            }
        } else {
            Self {
                year: self.year,
                month: self.month - 1,
            }
        }
    }

    /// Signed number of months from `self` to `other` (`other - self`).
    pub fn months_until(self, other: Self) -> i64 {
        (i64::from(other.year) - i64::from(self.year)) * 12 + (i64::from(other.month) - i64::from(self.month))
    }

    /// Every month from `self` through `end`, inclusive. Empty if `end < self`.
    pub fn range_inclusive(self, end: Self) -> impl Iterator<Item = Self> {
        let n = self.months_until(end).max(-1) + 1;
        std::iter::successors(Some(self), |m| Some(m.next())).take(usize::try_from(n).unwrap_or(0))
    }
}

impl fmt::Display for YearMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}", self.year, self.month)
    }
}

impl FromStr for YearMonth {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || Error::InvalidYearMonth(s.to_owned());
        let (y, m) = s.split_once('-').ok_or_else(err)?;
        let year: i32 = y.parse().map_err(|_| err())?;
        let month: u8 = m.parse().map_err(|_| err())?;
        Self::new(year, month).ok_or_else(err)
    }
}

impl TryFrom<String> for YearMonth {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl From<YearMonth> for String {
    fn from(ym: YearMonth) -> Self {
        ym.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_and_display_round_trip() {
        let ym: YearMonth = "2024-03".parse().unwrap();
        assert_eq!(ym, YearMonth::new(2024, 3).unwrap());
        assert_eq!(ym.to_string(), "2024-03");
    }

    #[test]
    fn rejects_bad_months() {
        assert!(YearMonth::new(2024, 0).is_none());
        assert!(YearMonth::new(2024, 13).is_none());
        assert!("2024-13".parse::<YearMonth>().is_err());
        assert!("202403".parse::<YearMonth>().is_err());
    }

    #[test]
    fn yyyymm_matches_french_format() {
        assert_eq!(YearMonth::from_yyyymm(192_607), YearMonth::new(1926, 7));
        assert_eq!(YearMonth::from_yyyymm(192_613), None);
    }

    #[test]
    fn arithmetic_crosses_year_boundaries() {
        let dec = YearMonth::new(2023, 12).unwrap();
        assert_eq!(dec.next(), YearMonth::new(2024, 1).unwrap());
        assert_eq!(dec.next().prev(), dec);
        assert_eq!(dec.months_until(YearMonth::new(2024, 3).unwrap()), 3);
        assert_eq!(YearMonth::new(2024, 3).unwrap().months_until(dec), -3);
    }

    #[test]
    fn ranges_are_inclusive_and_ordered() {
        let from = YearMonth::new(2023, 11).unwrap();
        let to = YearMonth::new(2024, 2).unwrap();
        let months: Vec<String> = from.range_inclusive(to).map(|m| m.to_string()).collect();
        assert_eq!(months, ["2023-11", "2023-12", "2024-01", "2024-02"]);
        assert_eq!(to.range_inclusive(from).count(), 0);
        assert!(from < to);
    }
}
