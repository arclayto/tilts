//! Error type shared by every fallible operation in `tilts-core`.

/// Everything that can go wrong in the math layer.
///
/// These are *input* errors. Numerical failures that indicate a bug (a non-finite
/// coefficient from clean data, say) are panics, not variants.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Two series that must be aligned have different lengths.
    #[error("series lengths differ: {left} vs {right}")]
    LengthMismatch {
        /// Length of the first series.
        left: usize,
        /// Length of the second series.
        right: usize,
    },

    /// Not enough observations for the requested regression or window.
    #[error("need at least {needed} observations, got {got}")]
    TooFewObservations {
        /// Minimum required.
        needed: usize,
        /// What was supplied.
        got: usize,
    },

    /// The design matrix has (numerically) collinear columns.
    #[error("design matrix is rank-deficient (column {column} is collinear)")]
    RankDeficient {
        /// Zero-based column index of the offending regressor.
        column: usize,
    },

    /// A model spec asked for a factor the panel doesn't contain.
    #[error("factor {0:?} is not in the panel")]
    MissingFactor(String),

    /// Periods are not strictly increasing, or two series cover different periods.
    #[error("periods are misaligned: {0}")]
    Misaligned(String),

    /// A `YearMonth` string could not be parsed.
    #[error("invalid year-month {0:?}; expected YYYY-MM")]
    InvalidYearMonth(String),
}

/// Result alias used throughout `tilts-core`.
pub type Result<T> = std::result::Result<T, Error>;
