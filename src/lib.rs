use std::path::Path;

use config::{Config, File};
use serde::Deserialize;
use ureq::{Agent, Error as UreqError, Response};

pub mod provider;
use provider::{duckdns::DuckDns, dyndns::Dyndns, noip::Noip, spdns::Spdns};

pub const FALLBACK_URL: &str = "http://checkip.spdns.de/";

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum Provider {
    Spdns,
    Dyndns,
    Duckdns,
    Noipdns,
}

#[derive(Debug, Deserialize)]
pub struct DnsConfig {
    pub provider: Provider,
    pub host: String,
    pub token: String,
    pub username: Option<String>,
}

impl DnsConfig {
    #[must_use]
    pub fn new(config_path: impl AsRef<Path>) -> Self {
        let settings = Config::builder()
            .add_source(
                File::with_name(
                    config_path
                        .as_ref()
                        .to_str()
                        .expect("no config file provided"),
                )
                .required(true),
            )
            .build()
            .expect("config not found.");

        settings
            .try_deserialize::<Self>()
            .expect("could not read config file")
    }

    #[inline(always)]
    pub fn run(self, agent: &Agent) {
        let handler: Box<dyn DynamicDns> = match self.provider {
            Provider::Spdns => Box::new(Spdns::new(self)),
            Provider::Dyndns => Box::new(Dyndns::new(self)),
            Provider::Duckdns => Box::new(DuckDns::new(self)),
            Provider::Noipdns => Box::new(Noip::new(self)),
        };

        handler.update(agent).unwrap();
    }
}

pub trait DynamicDns {
    //TODO: handle ipv4 and ipv6
    fn get_url(&self) -> Option<&str> {
        None
    }

    fn resolve(&self, agent: &Agent) -> String {
        agent
            .get(self.get_url().unwrap_or(FALLBACK_URL))
            .call()
            .unwrap()
            .into_string()
            .expect("No response from resolving.")
    }

    fn update(&self, agent: &Agent) -> Result<Response, UreqError>;
}
