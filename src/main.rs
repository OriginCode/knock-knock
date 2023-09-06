use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use xdg::BaseDirectories;

use cli::Args;
use config::Config;

mod cli;
mod config;
mod notifier;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let xdg_dirs = BaseDirectories::with_prefix("knock-knock")?;
    let config = Config::parse(
        args.config
            .unwrap_or(xdg_dirs.place_config_file("config.toml")?),
    )?;

    Ok(())
}
