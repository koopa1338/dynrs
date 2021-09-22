use dynrs::{resolve, DynamicDns};
use ureq::{Agent, Error as UreqError, Response};

pub struct Dyndns<'d> {
    host: &'d str,
    username: &'d str,
    token: &'d str,
}

impl<'d> Dyndns<'d> {
    pub fn new(host: &'d str, username: &'d str, token: &'d str) -> Self {
        Self {
            host,
            username,
            token,
        }
    }
}

impl DynamicDns for Dyndns<'_> {
    fn update(&self, agent: &Agent) -> Result<Response, UreqError> {
        let ip = resolve(agent, None);
        let update_url = format!(
            "https://{}:{}@members.dyndns.org/v3/update?hostname={}&myip={}",
            self.username, self.token, self.host, ip
        );
        agent.get(&update_url).call()
    }
}
