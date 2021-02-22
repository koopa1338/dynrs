use phf::{phf_map, Map};
use std::io::Error;
use ureq::{Agent, Request};

pub const FALLBACK_URL: &str = "http://checkip.spdns.de/";

pub const PROVIDER_MAP: Map<&'static str, Provider> = phf_map! {
    "spdns" => Provider::Spdns,
    "dyndns" => Provider::Dyndns,
};

pub struct Credentials<'a> {
    username: &'a str,
    token: &'a str,
}

impl Credentials<'_> {
    pub fn new<'a>(username: &'a str, token: &'a str) -> Credentials<'a> {
        Credentials { username, token }
    }
}

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

    pub fn update<'a>(self, agent: &Agent, creds: &'a Credentials) -> Result<Request, Error> {
        let update_url: String;
        let ipv6 = self.ipv6;
        let ip = self.resolv(agent)?;
        let user = creds.username;
        let pw = creds.token;
        match self.provider {
            Provider::Spdns => {
                if ipv6 {
                    update_url = format!("{}:{}@ipv6url/nic/update/{}", user, pw, ip);
                } else {
                    update_url = format!("{}:{}@url/nic/update/{}", user, pw, ip);
                }
            }
            Provider::Dyndns => {
                if ipv6 {
                    update_url = format!("{}:{}@ipv6url/nic/update/{}", user, pw, ip);
                } else {
                    update_url = format!("{}:{}@url/nic/update/{}", user, pw, ip);
                }
            }
        }
        Ok(agent.get(&update_url))
    }

    fn resolv(&self, agent: &Agent) -> Result<String, Error> {
        agent.get(&self.server_url).call().into_string()
    }
}
