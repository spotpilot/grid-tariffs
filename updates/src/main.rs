mod helpers;
mod locator;
mod pricing_info;
mod registry;

use tracing_subscriber::{
    filter::{LevelFilter, Targets},
    fmt,
    prelude::*,
};

use clap::{Parser, Subcommand};
use tracing::info;

use crate::locator::*;
use crate::registry::*;

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
    CheckAll,
    /// Get all the relevant texts from the HTML of the defined website
    ExtractText {
        grid_operator: String,
        #[arg(long)]
        store_cache: bool,
        #[arg(long)]
        load_cache: bool,
    },
    DownloadHtml {
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
        CliAction::CheckAll => {
            for _pi in PRICING_INFO.iter() {
                todo!()
                // let diff = pi.download_and_compare().await?;
            }
        }
        CliAction::ExtractText {
            grid_operator,
            store_cache,
            load_cache,
        } => {
            let Some(pi) = PRICING_INFO.get(&grid_operator) else {
                anyhow::bail!("Grid operator not found");
            };
            let html = pi.extract_html(store_cache, load_cache).await?;
            let text = pi.locator.locate_content(&html);
            info!("{}", text.unwrap_or_default());
        }
        CliAction::DownloadHtml {
            grid_operator,
            store_cache,
            load_cache,
        } => {
            let Some(pi) = PRICING_INFO.get(&grid_operator) else {
                anyhow::bail!("Grid operator not found");
            };
            let html = pi.extract_raw_html(store_cache, load_cache).await?;
            println!("{}", html);
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
