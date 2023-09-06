use anyhow::Result;
use std::{collections::HashMap, net::IpAddr, time::Duration};
use tokio_icmp_echo::Pinger;

use crate::{config::Config, notifier::State};

pub(crate) struct Listener {
    ips: Vec<IpAddr>,
}

impl Listener {
    pub(crate) async fn init(config: &Config) -> Result<(Self, HashMap<String, State>)> {
        let mut initial_states = HashMap::new();
        let mut ips = Vec::new();

        for (name, ip) in &config.listening {
            let pinger = Pinger::new().await?;
            let ip = ip.parse::<IpAddr>()?;

            ips.push(ip);

            match pinger.ping(ip, 1024, 0, Duration::from_secs(1)).await? {
                Some(_) => initial_states.insert(name.to_owned(), State::Online),
                None => initial_states.insert(name.to_owned(), State::Offline),
            };
        }

        Ok((Self { ips }, initial_states))
    }

    pub(crate) async fn update(&self) -> Result<Vec<State>> {
        let mut states = Vec::new();
        for ip in &self.ips {
            let pinger = Pinger::new().await?;
            match pinger.ping(*ip, 1024, 0, Duration::from_secs(1)).await? {
                Some(_) => states.push(State::Online),
                None => states.push(State::Offline),
            };
        }

        Ok(states)
    }
}
