//! `tilts-core`: factor regressions with honest standard errors.
//!
//! This is the math layer of [tilts](https://github.com/arclayto/tilts). It knows nothing
//! about tickers, data vendors, caches, or terminals. It takes *aligned* monthly return
//! series and factor series and produces regressions with standard errors that account
//! for the autocorrelation and heteroskedasticity that real return data has.
//!
//! The intended pipeline:
//!
//! 1. Build a [`FactorPanel`] (periods × named factor columns) and an excess-return
//!    series of the same length ([`returns::excess`]).
//! 2. Describe the regression with a [`ModelSpec`] ([`FactorSet`] + [`SeMethod`]).
//! 3. Call [`fit`] for a single full-sample regression, or [`rolling::rolling_fit`] for
//!    a moving window.
//!
//! Everything here is plain `f64` slices and `nalgebra` matrices; the optional `polars`
//! feature adds `DataFrame` adapters in [`frame`].
//!
//! **Status: scaffold.** Type shapes are what we intend to keep; most function bodies
//! are `todo!()` until milestone M0 (see the repository README).

pub mod error;
pub mod hac;
pub mod model;
pub mod ols;
pub mod panel;
pub mod regression;
pub mod returns;
pub mod rolling;
pub mod time;

#[cfg(feature = "polars")]
pub mod frame;

pub mod prelude {
    //! Glob-import the types most callers need.
    pub use crate::error::{Error, Result};
    pub use crate::model::{FactorSet, ModelSpec, SeMethod};
    pub use crate::panel::{FactorPanel, MonthlySeries};
    pub use crate::regression::{Coefficient, FactorFit, fit};
    pub use crate::time::YearMonth;
}

pub use error::{Error, Result};
pub use model::{FactorSet, ModelSpec, SeMethod};
pub use panel::{FactorPanel, MonthlySeries};
pub use regression::{Coefficient, FactorFit, fit};
pub use time::YearMonth;
