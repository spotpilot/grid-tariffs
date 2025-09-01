mod codegen;
mod completion;
mod helpers;
mod locator;
mod pricing_info;
mod store;

use std::{path::PathBuf, str::FromStr};

use clap::{Parser, Subcommand};
use grid_tariffs::{Country, GridOperator};
use serde::{Deserialize, Deserializer};
use tokio::task::JoinSet;
use tracing::{debug, error, info};
use tracing_subscriber::{
    filter::{LevelFilter, Targets},
    fmt,
    prelude::*,
};

use crate::{completion::OutputFormat, store::ResultStore};

#[derive(Parser)]
struct Cli {
    #[arg(short = 'l', long, env, default_value = "INFO")]
    log_level: LevelFilter,
    #[command(subcommand)]
    action: CliAction,
}

#[derive(Subcommand)]
enum CliAction {
    /// Check completion rate for each grid operator module
    CompletionReport {
        #[arg(short, long, default_value = "xlsx")]
        format: OutputFormat,
        output_dir: PathBuf,
    },
    /// Check for updates on each operator's website
    CheckAll {
        #[arg(long, env, default_value = "./results")]
        results_dir: PathBuf,
    },
    /// Check for updates for the given grid operator
    Check {
        #[arg(long, env, default_value = "./results")]
        results_dir: PathBuf,
        country: Country,
        grid_operator: String,
    },
    /// Create a new grid operator
    New {
        country: Country,
        name: String,
        vat_number: String,
        fee_link: String,
    },
    /// Create new grid operators from CSV file
    Import { csv_path: PathBuf },
    /// Get all the relevant texts from the HTML of the defined website
    ExtractText {
        #[arg(long, env, default_value = "./results")]
        results_dir: PathBuf,
        country: Country,
        grid_operator: String,
        #[arg(long)]
        store_cache: bool,
        #[arg(long)]
        load_cache: bool,
    },
    DownloadHtml {
        #[arg(long, env, default_value = "./results")]
        results_dir: PathBuf,
        country: Country,
        grid_operator: String,
        #[arg(long)]
        store_cache: bool,
        #[arg(long)]
        load_cache: bool,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    setup_tracing(cli.log_level);

    match cli.action {
        CliAction::CompletionReport { format, output_dir } => {
            completion::report(format, &output_dir)?;
        }
        CliAction::CheckAll { results_dir } => {
            let mut joinset = JoinSet::new();
            let store = ResultStore::new(results_dir).await;
            for op in GridOperator::all() {
                let store = store.clone();
                joinset.spawn(async move { (op, store.fetch_and_compare(op).await) });
            }
            while let Some(res) = joinset.join_next().await {
                let (op, res) = res?;
                match res {
                    Ok(comparison) => {
                        info!(
                            %comparison,
                            operator = op.name(),
                            diff = comparison.diff()
                        );
                    }
                    Err(err) => {
                        error!(%err, ?op, "failed");
                    }
                }
            }
        }
        CliAction::Check {
            results_dir,
            country,
            grid_operator,
        } => {
            let store = ResultStore::new(results_dir).await;
            for op in GridOperator::where_name_starts_with(country, &grid_operator) {
                let comparison = store.fetch_and_compare(op).await?;
                println!("{comparison} diff: {}", comparison.diff());
            }
        }
        CliAction::New {
            country,
            name,
            vat_number,
            fee_link,
        } => {
            codegen::generate_grid_operator(country, &name, &vat_number, &fee_link)?;
        }
        CliAction::Import { csv_path } => {
            #[derive(Debug, serde::Deserialize)]
            struct CsvRecord {
                #[serde(deserialize_with = "from_str")]
                country: Country,
                name: String,
                vat_number: String,
                fee_info_url: String,
            }
            let mut rdr = csv::Reader::from_path(csv_path)?;
            for result in rdr.deserialize() {
                let record: CsvRecord = result?;
                debug!(?record, "generating code");
                codegen::generate_grid_operator(
                    record.country,
                    &record.name,
                    &record.vat_number,
                    &record.fee_info_url,
                )?;
            }
        }
        CliAction::ExtractText {
            results_dir,
            country,
            grid_operator,
            store_cache,
            load_cache,
        } => {
            let Some(op) = GridOperator::get(country, &grid_operator) else {
                anyhow::bail!("Grid operator not found");
            };
            let store = ResultStore::new(results_dir).await;
            let result = if load_cache {
                store.load_or_remote_fetch(op).await?
            } else {
                store.remote_fetch(op).await?
            };
            if store_cache {
                store.store(op, &result).await?;
            }
            let text = result.extracted_content();
            println!("{text}");
        }
        CliAction::DownloadHtml {
            results_dir,
            country,
            grid_operator,
            store_cache,
            load_cache,
        } => {
            let Some(op) = GridOperator::get(country, &grid_operator) else {
                anyhow::bail!("Grid operator not found");
            };
            let store = ResultStore::new(results_dir).await;
            let result = if load_cache {
                store.load_or_remote_fetch(op).await?
            } else {
                store.remote_fetch(op).await?
            };
            if store_cache {
                store.store(op, &result).await?;
            }
            println!("{}", result.html());
        }
    }
    Ok(())
}

fn setup_tracing(default_level: LevelFilter) {
    let targets = Targets::new()
        .with_default(default_level)
        .with_target("html5ever", LevelFilter::INFO)
        .with_target("h2", LevelFilter::INFO)
        .with_target("hyper_util", LevelFilter::INFO)
        .with_target("selectors", LevelFilter::INFO);

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(targets)
        .init();
}

pub fn from_str<'de, T, D>(de: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: std::fmt::Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(de)?;
    T::from_str(&s).map_err(serde::de::Error::custom)
}
