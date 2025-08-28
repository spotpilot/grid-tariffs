mod helpers;
mod locator;
mod pricing_info;
mod registry;

use std::path::PathBuf;

use tracing_subscriber::{
    filter::{LevelFilter, Targets},
    fmt,
    prelude::*,
};

use clap::{Parser, Subcommand};

use crate::registry::*;
use crate::{locator::*, pricing_info::ResultStore};

#[derive(Parser)]
struct Cli {
    #[arg(short = 'l', long, env, default_value = "INFO")]
    log_level: LevelFilter,
    #[command(subcommand)]
    action: CliAction,
}

#[derive(Subcommand)]
enum CliAction {
    /// Check for updates on each operator's website
    CheckAll {
        #[arg(long, env, default_value = "./results")]
        results_dir: PathBuf,
    },
    /// Get all the relevant texts from the HTML of the defined website
    ExtractText {
        #[arg(long, env, default_value = "./results")]
        results_dir: PathBuf,
        grid_operator: String,
        #[arg(long)]
        store_cache: bool,
        #[arg(long)]
        load_cache: bool,
    },
    DownloadHtml {
        #[arg(long, env, default_value = "./results")]
        results_dir: PathBuf,
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
        CliAction::CheckAll { results_dir } => {
            let store = ResultStore::new(results_dir).await;
            for pi in PRICING_INFO.iter() {
                let comparison = store.fetch_and_compare(pi).await?;
                println!("{:?}", comparison);
                println!("Diff: {}", comparison.diff());
            }
        }
        CliAction::ExtractText {
            results_dir,
            grid_operator,
            store_cache,
            load_cache,
        } => {
            let Some(pi) = PRICING_INFO.get(&grid_operator) else {
                anyhow::bail!("Grid operator not found");
            };
            let store = ResultStore::new(results_dir).await;
            let result = if load_cache {
                store.load_or_remote_fetch(pi).await?
            } else {
                store.remote_fetch(pi).await?
            };
            if store_cache {
                store.store(pi, &result).await?;
            }
            let text = result.extracted_text().unwrap_or_default();
            println!("{text}");
        }
        CliAction::DownloadHtml {
            results_dir,
            grid_operator,
            store_cache,
            load_cache,
        } => {
            let Some(pi) = PRICING_INFO.get(&grid_operator) else {
                anyhow::bail!("Grid operator not found");
            };
            let store = ResultStore::new(results_dir).await;
            let result = if load_cache {
                store.load_or_remote_fetch(pi).await?
            } else {
                store.remote_fetch(pi).await?
            };
            if store_cache {
                store.store(pi, &result).await?;
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
