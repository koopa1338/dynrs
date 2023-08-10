use crate::{DnsConfig, DynamicDns};
use ureq::{Agent, Error as UreqError, Response};

const RESOLVE_URL: &str = "https://dynupdate.no-ip.com/nic/update";

pub struct Noip<'d> {
    host: &'d str,
    username: &'d str,
    token: &'d str,
}

impl<'d> Noip<'d> {
    #[must_use]
    pub fn new(config: &'d DnsConfig) -> Self {
        Self {
            host: config.host,
            username: config.username.expect("required username missing."),
            token: config.token,
        }
    }
}

impl DynamicDns for Noip<'_> {
    fn update(&self, agent: &Agent) -> Result<Response, UreqError> {
        // NOTE: the second part of the string is the ip address
        let ip = &self
            .resolve(agent)
            .split_whitespace()
            .last()
            .expect("Resolved IP was empty")
            .to_string();
        let update_url = format!(
            "https://{}:{}@dynupdate.no-ip.com/nic/update?hostname={}&myip={}",
            self.username, self.token, self.host, ip
        );
        // TODO: set the user agent as the api docs say to prevent blocking
        agent.get(&update_url).call()
    }

    fn get_url(&self) -> Option<&str> {
        Some(RESOLVE_URL)
    }
}
