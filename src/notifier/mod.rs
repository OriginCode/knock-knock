use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;

use self::telegram::Telegram;
use crate::config::Config;

mod telegram;

// State Machine
//
//    Success             Fail
//    ┌──────┐ Fail       ┌───────┐
//   ┌┴──────▼┌─────────►┌┴───────▼┐
//   │ Online │   Success│ Offline │
//   └────────┘◄─────────┴─────────┘
#[derive(PartialEq, Eq, Clone, Copy)]
pub(crate) enum State {
    Online,
    Offline,
}

#[async_trait]
pub(crate) trait NotifierTrait {
    async fn update(&self, listener: &str, state: State) -> Result<()>;
}

pub(crate) struct Notifiers {
    notifiers: Vec<Box<dyn NotifierTrait>>,
    states: HashMap<String, State>,
}

impl Notifiers {
    pub(crate) fn init(config: &Config, init_states: HashMap<String, State>) -> Self {
        Self {
            notifiers: vec![Box::new(Telegram::init(config))],
            states: init_states,
        }
    }

    pub(crate) async fn update(&mut self, states: Vec<State>) -> Result<()> {
        for ((l, o), n) in self.states.iter_mut().zip(states) {
            if *o != n {
                *o = n;
                for i in &self.notifiers {
                    i.update(l, n).await?;
                }
            }
        }

        Ok(())
    }
}
