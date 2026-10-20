//! Error type for data fetching, parsing, and caching.

/// Everything that can go wrong between the network and a clean panel.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// HTTP failure talking to a vendor or French's site.
    #[error("request to {url} failed: {source}")]
    Http {
        /// The URL that failed.
        url: String,
        /// Underlying client error.
        #[source]
        source: Box<ureq::Error>,
    },

    /// The remote returned something we couldn't parse.
    #[error("could not parse {what} from {origin}: {detail}")]
    Parse {
        /// What we were trying to read (e.g. "FF3 monthly factors").
        what: String,
        /// Where it came from.
        origin: String,
        /// What went wrong.
        detail: String,
    },

    /// A ticker the vendor doesn't know.
    #[error("{source_name} has no data for ticker {ticker:?}")]
    UnknownTicker {
        /// Which source said so.
        source_name: &'static str,
        /// The offending ticker.
        ticker: String,
    },

    /// A source needs credentials it couldn't find.
    #[error("{source_name} needs {env_var} to be set")]
    MissingCredentials {
        /// Which source.
        source_name: &'static str,
        /// The environment variable it looked for.
        env_var: &'static str,
    },

    /// A dataset/region/frequency combination French doesn't publish (or we haven't mapped).
    #[error("no French dataset for {0}")]
    UnsupportedDataset(String),

    /// Holdings file problems.
    #[error("holdings file: {0}")]
    Holdings(String),

    /// Cache directory or file I/O.
    #[error("cache I/O: {0}")]
    Io(#[from] std::io::Error),

    /// Frame construction or Parquet errors.
    #[error(transparent)]
    Polars(#[from] polars::prelude::PolarsError),

    /// Alignment or validation errors from the core layer.
    #[error(transparent)]
    Core(#[from] tilts_core::Error),
}

/// Result alias used throughout `tilts-data`.
pub type Result<T> = std::result::Result<T, Error>;
