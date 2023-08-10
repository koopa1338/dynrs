use std::path::PathBuf;

use config::{Config, File};
use phf::{phf_map, Map};
use serde::Deserialize;
use ureq::{Agent, Error as UreqError, Response};
pub mod provider;
use provider::{duckdns::DuckDns, dyndns::Dyndns, noip::Noip, spdns::Spdns};

pub const FALLBACK_URL: &str = "http://checkip.spdns.de/";

pub static PROVIDER_MAP: Map<&'static str, Provider> = phf_map! {
    "spdns" => Provider::Spdns,
    "dyndns" => Provider::Dyndns,
    "duckdns" => Provider::Duckdns,
    "noip" => Provider::Noipdns,
};

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum Provider {
    Spdns,
    Dyndns,
    Duckdns,
    Noipdns,
}

#[derive(Debug, Deserialize)]
pub struct DnsConfig<'dns> {
    pub provider: Provider,
    pub host: &'dns str,
    pub token: &'dns str,
    pub username: Option<&'dns str>,
}

impl<'dns> Default for DnsConfig<'dns> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'dns> DnsConfig<'dns> {
    #[must_use]
    pub fn new() -> Self {
        let mut settings_path = if let Ok(xdg_env) = std::env::var("XDG_CONFIG_HOME") {
            PathBuf::from(xdg_env)
        } else {
            PathBuf::from(std::env!("HOME")).join(".config")
        };
        settings_path.push("dynrs/config");
        let settings = Config::builder()
            .add_source(File::with_name(settings_path.to_string_lossy().as_ref()).required(true))
            .build()
            .expect("config not found.");

        settings
            .try_deserialize::<Self>()
            .expect("could not read config file")
    }

    #[inline(always)]
    pub fn run(&self, agent: &Agent) {
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
