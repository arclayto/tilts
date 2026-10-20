//! `tilts-data`: everything that touches the network or the disk.
//!
//! * [`french`] — factor series from Ken French's data library (free, no key).
//! * [`source`] — the [`PriceSource`] trait, with vendor
//!   implementations in [`tiingo`] and [`yahoo`].
//! * [`cache`] — on-disk Parquet cache so nothing is downloaded twice.
//! * [`holdings`] — the holdings CSV a user hands to the CLI.
//!
//! Design rule: every fetch goes through the cache, and every source is a trait
//! object, because vendor APIs break and we want swapping them to be a one-line
//! change for the user (`--source yahoo`).
//!
//! **Status: scaffold.**

pub mod cache;
pub mod error;
pub mod french;
pub mod holdings;
pub mod source;
pub mod tiingo;
pub mod yahoo;

pub use error::{Error, Result};
pub use source::PriceSource;
