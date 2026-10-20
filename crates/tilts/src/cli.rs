//! Command-line surface. Keep every flag documented here; the README's usage
//! section is generated from `tilts --help` output.

use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use clap::{Args, Parser, Subcommand, ValueEnum};
use tilts_core::{FactorSet, ModelSpec, SeMethod, YearMonth};
use tilts_data::cache::Cache;
use tilts_data::french::Region;

/// Factor-exposure analysis for real portfolios.
#[derive(Debug, Parser)]
#[command(name = "tilts", version, about, long_about = None)]
pub struct Cli {
    /// Override the cache directory (default: the platform data dir for tilts).
    #[arg(long, global = true, env = "TILTS_CACHE_DIR")]
    pub cache_dir: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Command,
}

/// Top-level subcommands.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Regress a portfolio on a factor model and report exposures.
    Analyze(AnalyzeArgs),
    /// Fetch or inspect factor data.
    Factors {
        #[command(subcommand)]
        command: FactorsCommand,
    },
    /// Inspect or clear the local data cache.
    Cache {
        #[command(subcommand)]
        command: CacheCommand,
    },
}

/// Arguments for `tilts analyze`.
#[derive(Debug, Args)]
pub struct AnalyzeArgs {
    /// Holdings CSV with `ticker,weight` columns (weights or dollar amounts).
    pub holdings: PathBuf,

    /// Factor model.
    #[arg(long, value_enum, default_value_t = Model::Ff3)]
    pub model: Model,

    /// Which market's factors to use.
    #[arg(long, value_enum, default_value_t = RegionArg::Us)]
    pub region: RegionArg,

    /// First month to include (YYYY-MM). Defaults to the earliest month every holding has data.
    #[arg(long)]
    pub since: Option<YearMonth>,

    /// Last month to include (YYYY-MM). Defaults to the latest complete month.
    #[arg(long)]
    pub until: Option<YearMonth>,

    /// Also report rolling exposures over windows of this many months (e.g. 36).
    #[arg(long)]
    pub rolling: Option<usize>,

    /// Standard-error method.
    #[arg(long, value_enum, default_value_t = Se::NeweyWest)]
    pub se: Se,

    /// Newey-West lag truncation. Default is the data-driven rule in tilts-core. Ignored with --se ols.
    #[arg(long)]
    pub lags: Option<usize>,

    /// Price data vendor.
    #[arg(long, value_enum, default_value_t = Source::Tiingo)]
    pub source: Source,
}

/// Factor models exposed on the command line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Model {
    /// Market only.
    Capm,
    /// Fama-French three-factor.
    Ff3,
    /// Fama-French five-factor.
    Ff5,
    /// Five-factor plus momentum.
    Ff5mom,
}

impl From<Model> for FactorSet {
    fn from(m: Model) -> Self {
        match m {
            Model::Capm => Self::Capm,
            Model::Ff3 => Self::Ff3,
            Model::Ff5 => Self::Ff5,
            Model::Ff5mom => Self::Ff5Mom,
        }
    }
}

/// Factor regions exposed on the command line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum RegionArg {
    /// United States.
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
    /// North America.
    NorthAmerica,
    /// Emerging markets (five-factor only).
    Emerging,
}

impl From<RegionArg> for Region {
    fn from(r: RegionArg) -> Self {
        match r {
            RegionArg::Us => Self::Us,
            RegionArg::Developed => Self::Developed,
            RegionArg::DevelopedExUs => Self::DevelopedExUs,
            RegionArg::Europe => Self::Europe,
            RegionArg::Japan => Self::Japan,
            RegionArg::AsiaPacificExJapan => Self::AsiaPacificExJapan,
            RegionArg::NorthAmerica => Self::NorthAmerica,
            RegionArg::Emerging => Self::Emerging,
        }
    }
}

/// Standard-error methods exposed on the command line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Se {
    /// Newey-West HAC (default).
    NeweyWest,
    /// Classical OLS, for comparison.
    Ols,
}

/// Price vendors exposed on the command line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Source {
    /// Tiingo (needs `TIINGO_API_KEY`).
    Tiingo,
    /// Yahoo Finance (unofficial, no key).
    Yahoo,
}

/// `tilts factors …`
#[derive(Debug, Subcommand)]
pub enum FactorsCommand {
    /// Download a factor set into the cache.
    Fetch {
        /// Factor model whose series to fetch.
        #[arg(long, value_enum, default_value_t = Model::Ff3)]
        model: Model,
        /// Region.
        #[arg(long, value_enum, default_value_t = RegionArg::Us)]
        region: RegionArg,
    },
}

/// `tilts cache …`
#[derive(Debug, Subcommand)]
pub enum CacheCommand {
    /// Print the cache directory.
    Path,
    /// Delete everything in the cache.
    Clear,
}

/// Build the core `ModelSpec` from CLI flags.
pub fn model_spec(args: &AnalyzeArgs) -> ModelSpec {
    let se = match args.se {
        Se::NeweyWest => SeMethod::NeweyWest { lags: args.lags },
        Se::Ols => SeMethod::Ols,
    };
    ModelSpec::new(args.model.into()).with_se(se)
}

/// Dispatch a parsed command line.
pub fn run(cli: Cli) -> Result<()> {
    let cache = match &cli.cache_dir {
        Some(dir) => Cache::at(dir),
        None => Cache::default_location().context("locating the cache directory")?,
    };

    match cli.command {
        Command::Cache {
            command: CacheCommand::Path,
        } => {
            println!("{}", cache.root().display());
            Ok(())
        }
        Command::Cache {
            command: CacheCommand::Clear,
        } => {
            cache.clear().context("clearing the cache")?;
            println!("cleared {}", cache.root().display());
            Ok(())
        }
        Command::Factors {
            command: FactorsCommand::Fetch { model, region },
        } => {
            bail!(
                "not implemented yet (M0): would fetch {model:?} factors for {region:?} into {}",
                cache.root().display()
            )
        }
        Command::Analyze(args) => {
            let spec = model_spec(&args);
            bail!(
                "not implemented yet (M0): would regress {} on {:?} with {:?} standard errors",
                args.holdings.display(),
                spec.factors,
                spec.se
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_an_analyze_invocation() {
        let cli = Cli::try_parse_from([
            "tilts",
            "analyze",
            "h.csv",
            "--model",
            "ff5mom",
            "--since",
            "2015-01",
            "--rolling",
            "36",
        ])
        .unwrap();
        let Command::Analyze(args) = cli.command else {
            panic!("expected analyze")
        };
        assert_eq!(args.model, Model::Ff5mom);
        assert_eq!(args.since, YearMonth::new(2015, 1));
        assert_eq!(args.rolling, Some(36));
        let spec = model_spec(&args);
        assert_eq!(spec.factors, FactorSet::Ff5Mom);
        assert_eq!(spec.se, SeMethod::NeweyWest { lags: None });
    }

    #[test]
    fn rejects_bad_month() {
        assert!(Cli::try_parse_from(["tilts", "analyze", "h.csv", "--since", "2015-13"]).is_err());
    }

    #[test]
    fn help_is_well_formed() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
