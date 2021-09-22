use phf::{phf_map, Map};
use ureq::{Agent, Error as UreqError, Response};

pub static PROVIDER_MAP: Map<&'static str, Provider> = phf_map! {
    "spdns" => Provider::Spdns,
    "dyndns" => Provider::Dyndns,
};

#[derive(Clone, Copy)]
pub enum Provider {
    Spdns,
    Dyndns,
}

pub trait DynamicDns {
    fn update(&self, agent: &Agent) -> Result<Response, UreqError>;
}

//TODO: handle ipv4 and ipv6
pub fn resolve(agent: &Agent, url: &str) -> String {
    agent
        .get(url)
        .call()
        .unwrap()
        .into_string()
        .expect("No response from resolving.")
}
