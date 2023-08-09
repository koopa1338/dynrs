use crate::{DynamicDns, DnsConfig};
use ureq::{Agent, Error as UreqError, Response};

pub struct Dyndns<'d> {
    host: &'d str,
    username: &'d str,
    token: &'d str,
}

impl<'d> Dyndns<'d> {
    pub fn new(config: &'d DnsConfig) -> Self {
        Self {
            host: config.host,
            username: config.username.expect("required username not found"),
            token: config.token,
        }
    }
}

impl DynamicDns for Dyndns<'_> {
    fn update(&self, agent: &Agent) -> Result<Response, UreqError> {
        let ip = &self.resolve(agent);
        let update_url = format!(
            "https://{}:{}@members.dyndns.org/v3/update?hostname={}&myip={}",
            self.username, self.token, self.host, ip
        );
        agent.get(&update_url).call()
    }
}
