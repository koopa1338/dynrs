use phf::{phf_map, Map};
use ureq::{Agent, Response};

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
        host: &'a str,
        username: &'a str,
        token: &'a str,
    ) -> Response {
        let update_url: String;
        let ipv6 = self.ipv6;
        let ip = self.resolv(agent);
        match self.provider {
            Provider::Spdns => {
                update_url = format!("https://update.spdyn.de/nic/update?hostname={}&myip={}&user={}&pass={}", host, ip, username, token);
            }
            Provider::Dyndns => {
                if ipv6 {
                    update_url = format!("{}:{}@ipv6url/nic/update/{}", username, token, ip);
                } else {
                    update_url = format!("{}:{}@url/nic/update/{}", username, token, ip);
                }
            }
        }
        // TODO: check status code and return an error if update was not successful
        agent.get(&update_url).call()

    }

    fn resolv(&self, agent: &Agent) -> String {
        agent.get(&self.server_url).call().into_string().expect("No response from resolving.")
    }
}
