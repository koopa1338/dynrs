use dynrs::{resolve, DynamicDns};
use ureq::{Agent, Error as UreqError, Response};

pub struct Dyndns<'d> {
    pub username: &'d str,
    pub token: &'d str,
}

impl DynamicDns for Dyndns<'_> {
    fn update(&self, agent: &Agent) -> Result<Response, UreqError> {
        let ip = resolve(agent, None);
        let update_url = format!("{}:{}@url/nic/update/{}", self.username, self.token, ip);
        Ok(agent.get(&update_url).call()?)
    }
}
