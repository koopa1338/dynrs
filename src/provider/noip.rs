use dynrs::{resolve, DynamicDns};
use ureq::{Agent, Error as UreqError, Response};

const RESOLVE_URL: &str = "https://dynupdate.no-ip.com/nic/update";

pub struct Noip<'d> {
    host: &'d str,
    username: &'d str,
    token: &'d str,
}

impl<'d> Noip<'d> {
    pub fn new(host: &'d str, username: &'d str, token: &'d str) -> Self {
        Self {
            host,
            username,
            token,
        }
    }
}

impl DynamicDns for Noip<'_> {
    fn update(&self, agent: &Agent) -> Result<Response, UreqError> {
        // NOTE: the second part of the string is the ip address
        let ip = resolve(agent, Some(RESOLVE_URL))
            .split_whitespace()
            .last()
            .unwrap()
            .to_string();
        let update_url = format!(
            "https://{}:{}@dynupdate.no-ip.com/nic/update?hostname={}&myip={}",
            self.username, self.token, self.host, ip
        );
        // TODO: set the user agent as the api docs say to prevent blocking
        agent.get(&update_url).call()
    }
}
