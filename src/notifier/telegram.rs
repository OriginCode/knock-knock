use anyhow::Result;
use async_trait::async_trait;
use teloxide::prelude::*;

use crate::config::Config;

use super::NotifierTrait;
use super::State;

pub(super) struct Telegram {
    bot: Bot,
    chat_id: String,
}

impl Telegram {
    pub(super) fn init(config: &Config) -> Self {
        Self {
            bot: Bot::new(&config.notifier.telegram.token),
            chat_id: config.notifier.telegram.chat_id.to_owned(),
        }
    }
}

#[async_trait]
impl NotifierTrait for Telegram {
    async fn update(&self, listener: &str, state: State) -> Result<()> {
        self.bot
            .send_message(
                self.chat_id.clone(),
                match state {
                    State::Online => format!("{}: Revived", &listener),
                    State::Offline => format!("{}: Died", &listener),
                },
            )
            .await?;

        Ok(())
    }
}
