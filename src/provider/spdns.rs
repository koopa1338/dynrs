use dynrs::{resolve, DynamicDns};
use ureq::{Agent, Error as UreqError, Response};

const RESOLVE_URL: &str = "http://checkip.spdns.de/";

pub struct Spdns<'d> {
    pub host: &'d str,
    pub username: &'d str,
    pub token: &'d str,
}

impl DynamicDns for Spdns<'_> {
    fn update(&self, agent: &Agent) -> Result<Response, UreqError> {
        let ip = resolve(agent, Some(RESOLVE_URL));
        let update_url = format!(
            "https://update.spdyn.de/nic/update?hostname={}&myip={}&user={}&pass={}",
            self.host, ip, self.username, self.token
        );
        Ok(agent.get(&update_url).call()?)
    }
}
