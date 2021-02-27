use phf::{phf_map, Map};
use std::io::Error;
use ureq::{Agent, Request};

pub const FALLBACK_URL: &str = "http://checkip.spdns.de/";

pub const PROVIDER_MAP: Map<&'static str, Provider> = phf_map! {
    "spdns" => Provider::Spdns,
    "dyndns" => Provider::Dyndns,
};

#[derive(Clone, Copy)]
pub enum Provider {
    Spdns,
    Dyndns,
}

pub struct Handler {
    provider: Provider,
    ipv6: bool,
    server_url: String,
}

impl Handler {
    pub fn new(provider: Provider, ipv6: bool, server_url: String) -> Self {
        Handler {
            provider,
            ipv6,
            server_url,
        }
    }

    pub fn update<'a>(
        self,
        agent: &Agent,
        username: &'a str,
        token: &'a str,
    ) -> Result<Request, Error> {
        let update_url: String;
        let ipv6 = self.ipv6;
        let ip = self.resolv(agent)?;
        match self.provider {
            Provider::Spdns => {
                if ipv6 {
                    update_url = format!("{}:{}@ipv6url/nic/update/{}", username, token, ip);
                } else {
                    update_url = format!("{}:{}@url/nic/update/{}", username, token, ip);
                }
            }
            Provider::Dyndns => {
                if ipv6 {
                    update_url = format!("{}:{}@ipv6url/nic/update/{}", username, token, ip);
                } else {
                    update_url = format!("{}:{}@url/nic/update/{}", username, token, ip);
                }
            }
        }
        Ok(agent.get(&update_url))
    }

    fn resolv(&self, agent: &Agent) -> Result<String, Error> {
        agent.get(&self.server_url).call().into_string()
    }
}
