use std::path::Path;

use config::{Config, File};
use serde::Deserialize;
use ureq::{Agent, Error as UreqError, Response};

pub const FALLBACK_URL: &str = "http://checkip.spdns.de/";

#[derive(Debug, Deserialize)]
pub struct DnsClient {
    pub host: String,
    pub secret: String,
    pub update_url: String,
    pub username: Option<String>,
    pub resolve_url: Option<String>,
}

impl DnsClient {
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
}

pub trait DynamicDns {
    //TODO: handle ipv4 and ipv6
    fn get_url(&self) -> Option<&str> {
        None
    }

    fn resolve(&self, agent: &Agent) -> String {
        agent
            .get(&self.get_url().unwrap_or(FALLBACK_URL))
            .call()
            .unwrap()
            .into_string()
            .expect("No response from resolving.")
    }

    fn update(&self, agent: &Agent) -> Result<Response, UreqError>;
}

impl DynamicDns for DnsClient {
    fn update(&self, agent: &Agent) -> Result<Response, UreqError> {
        let ip = &self
            .resolve(agent)
            .split_whitespace()
            .last()
            .expect("Resolved IP was empty")
            .to_string();

        // TODO: format with url crate and add the params
        let update_url = &self.update_url;

        agent.get(&self.update_url).call()
    }

    fn get_url(&self) -> Option<&str> {
        self.resolve_url.as_deref()
    }
}
