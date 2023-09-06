use anyhow::Result;
use clap::Parser;
use tokio::time::{sleep, Duration};
use xdg::BaseDirectories;

use cli::Args;
use config::Config;
use listener::Listener;
use notifier::Notifiers;

mod cli;
mod config;
mod listener;
mod notifier;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let xdg_dirs = BaseDirectories::with_prefix("knock-knock")?;
    let config = Config::parse(
        args.config
            .unwrap_or(xdg_dirs.place_config_file("config.toml")?),
    )?;

    let (listener, init_states) = Listener::init(&config).await?;
    let mut notifiers = Notifiers::init(&config, init_states);

    loop {
        notifiers.update(listener.update().await?).await?;

        sleep(Duration::from_secs(60)).await;
    }
}
