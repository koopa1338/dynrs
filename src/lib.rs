use phf::{phf_map, Map};
use ureq::{Agent, Error as UreqError, Response};

pub const FALLBACK_URL: &str = "http://checkip.spdns.de/";

pub static PROVIDER_MAP: Map<&'static str, Provider> = phf_map! {
    "spdns" => Provider::Spdns,
    "dyndns" => Provider::Dyndns,
    "duckdns" => Provider::Duckdns,
    "noip" => Provider::Noipdns,
};

#[derive(Clone, Copy)]
pub enum Provider {
    Spdns,
    Dyndns,
    Duckdns,
    Noipdns,
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
