//! Terminal rendering of fits. The terminal is the primary face of the tool in v1;
//! an HTML report is planned for M2 and should reuse these shapes, not this code.

// Nothing calls this until `analyze` can produce a fit (M0). Remove with that change.
#![allow(dead_code)]

use comfy_table::{ContentArrangement, Table, presets::UTF8_FULL_CONDENSED};
use tilts_core::FactorFit;

/// Critical value for the 95% intervals we print.
const Z_95: f64 = 1.96;

/// A coefficient table: name, estimate, standard error, t-stat, 95% interval.
///
/// Alpha is shown annualised (as a percentage) because that's the number people
/// care about; factor loadings are unitless and shown raw.
pub fn coefficient_table(fit: &FactorFit) -> Table {
    let mut table = Table::new();
    table
        .load_style(UTF8_FULL_CONDENSED)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(["", "estimate", "s.e.", "t", "95% interval"]);

    for (name, c) in fit.names.iter().zip(&fit.coefficients) {
        let (lo, hi) = c.interval(Z_95);
        let (est, lo, hi) = if name == "alpha" {
            (
                format!("{:.2}%/yr", annualise_pct(c.estimate)),
                format!("{:.2}%", annualise_pct(lo)),
                format!("{:.2}%", annualise_pct(hi)),
            )
        } else {
            (format!("{:.3}", c.estimate), format!("{lo:.3}"), format!("{hi:.3}"))
        };
        table.add_row([
            name.clone(),
            est,
            format!("{:.3}", c.se),
            format!("{:.2}", c.t_stat),
            format!("[{lo}, {hi}]"),
        ]);
    }
    table
}

/// One-line fit summary printed above the table.
pub fn summary_line(fit: &FactorFit) -> String {
    format!(
        "{} obs, {} to {}, R² {:.3} (adj {:.3}), {:?}",
        fit.n_obs, fit.span.0, fit.span.1, fit.r2, fit.adj_r2, fit.se_method
    )
}

fn annualise_pct(monthly: f64) -> f64 {
    tilts_core::returns::annualise_monthly(monthly) * 100.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use tilts_core::regression::Coefficient;
    use tilts_core::{SeMethod, YearMonth};

    #[test]
    fn renders_a_fit_without_panicking() {
        let fit = FactorFit {
            names: vec!["alpha".into(), "Mkt-RF".into()],
            coefficients: vec![
                Coefficient {
                    estimate: 0.001,
                    se: 0.002,
                    t_stat: 0.5,
                },
                Coefficient {
                    estimate: 1.02,
                    se: 0.03,
                    t_stat: 34.0,
                },
            ],
            r2: 0.98,
            adj_r2: 0.979,
            n_obs: 120,
            span: (YearMonth::new(2015, 1).unwrap(), YearMonth::new(2024, 12).unwrap()),
            se_method: SeMethod::NeweyWest { lags: Some(4) },
            residuals: vec![],
        };
        let rendered = coefficient_table(&fit).to_string();
        assert!(rendered.contains("Mkt-RF"));
        assert!(rendered.contains("%/yr"));
        assert!(summary_line(&fit).starts_with("120 obs, 2015-01 to 2024-12"));
    }
}
