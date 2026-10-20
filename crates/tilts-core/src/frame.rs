//! `polars::DataFrame` adapters. Enabled with the `polars` feature.
//!
//! The math layer works on slices; this module converts to and from the frame shape
//! that `tilts-data` produces and that Python users (via future bindings) expect:
//! a `period` column of `YYYY-MM` strings plus one `f64` column per series.

use polars::prelude::DataFrame;

use crate::error::Result;
use crate::panel::{FactorPanel, MonthlySeries};

/// Column name holding the month in every frame this crate reads or writes.
pub const PERIOD_COLUMN: &str = "period";

/// Read a panel from a frame with a `period` column and one column per factor.
pub fn panel_from_frame(df: &DataFrame) -> Result<FactorPanel> {
    let _ = df;
    todo!("M0: parse period column, collect f64 columns in frame order")
}

/// Read one named column (plus `period`) as a series.
pub fn series_from_frame(df: &DataFrame, column: &str) -> Result<MonthlySeries> {
    let _ = (df, column);
    todo!("M0")
}

/// Write a panel back to a frame.
pub fn panel_to_frame(panel: &FactorPanel) -> DataFrame {
    let _ = panel;
    todo!("M0")
}
