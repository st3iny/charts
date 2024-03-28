use std::env;

use anyhow::{Context, Result};
use chart::Chart;
use clap::Parser;
use github::fetch_latest_tag;
use semver::Version;

mod chart;
mod github;

#[derive(Parser)]
#[command(version)]
struct Args {
    /// Enable verbose logging
    #[arg(long, short)]
    verbose: bool,

    /// Token does not need any scopes
    #[arg(long)]
    github_token: Option<String>,

    /// Path to Chart.yaml
    #[arg()]
    chart: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.verbose {
        env::set_var("RUST_LOG", "debug");
    } else {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    if let Err(error) = main_impl(args).await {
        log::error!("{error:?}");
    }
}

async fn main_impl(args: Args) -> Result<()> {
    let tag = match fetch_latest_tag(args.github_token.as_ref())
        .await
        .context("Failed to fetch latest tag from GitHub")?
    {
        Some(tag) => tag,
        None => {
            log::warn!("No suitable release found");
            return Ok(());
        }
    };
    let tag = Version::parse(tag.trim_start_matches('v'))
        .context("Failed to parse latest trilium tag as semver")?;

    let mut chart = Chart::load(&args.chart).context("Failed to load chart")?;

    if chart.app_version >= tag {
        log::info!("Chart is up to date");
        return Ok(());
    }

    log::info!("Updating trilium from {} to {tag}", chart.app_version);
    chart.app_version = tag;
    chart.version.minor += 1;
    chart.version.patch = 0;

    chart.save(&args.chart).context("Failed to save chart")?;

    Ok(())
}
