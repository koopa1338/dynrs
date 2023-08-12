use crate::{DynamicDns, DnsConfig};
use ureq::{Agent, Error as UreqError, Response};

pub struct Dyndns {
    host: String,
    username: String,
    token: String,
}

impl Dyndns {
    #[must_use]
    pub fn new(config: DnsConfig) -> Self {
        Self {
            host: config.host,
            username: config.username.expect("required username not found"),
            token: config.token,
        }
    }
}

impl DynamicDns for Dyndns {
    fn update(&self, agent: &Agent) -> Result<Response, UreqError> {
        let ip = &self.resolve(agent);
        let update_url = format!(
            "https://{}:{}@members.dyndns.org/v3/update?hostname={}&myip={}",
            self.username, self.token, self.host, ip
        );
        agent.get(&update_url).call()
    }
}
