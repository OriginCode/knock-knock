use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Config {
    pub(crate) listening: HashMap<String, String>,
    pub(crate) notifier: NotifierConfig,
    pub(crate) sched_secs: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct NotifierConfig {
    pub(crate) telegram: TelegramConfig,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct TelegramConfig {
    pub(crate) token: String,
    pub(crate) chat_id: String,
}

impl Config {
    pub(crate) fn parse<P: AsRef<Path>>(path: P) -> Result<Config> {
        Ok(toml::from_str(&fs::read_to_string(path.as_ref())?)?)
    }
}
