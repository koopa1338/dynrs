use crate::{DnsConfig, DynamicDns};
use ureq::{Agent, Error as UreqError, Response};

const RESOLVE_URL: &str = "http://checkip.spdns.de/";

pub struct Spdns {
    host: String,
    username: String,
    token: String,
}

impl Spdns {
    #[must_use]
    pub fn new(config: DnsConfig) -> Self {
        Self {
            host: config.host,
            username: config.username.expect("required username missing."),
            token: config.token,
        }
    }
}

impl DynamicDns for Spdns {
    fn update(&self, agent: &Agent) -> Result<Response, UreqError> {
        let ip = &self.resolve(agent);
        let update_url = format!(
            "https://update.spdyn.de/nic/update?hostname={}&myip={}&user={}&pass={}",
            self.host, ip, self.username, self.token
        );
        agent.get(&update_url).call()
    }

    fn get_url(&self) -> Option<&str> {
        Some(RESOLVE_URL)
    }
}
