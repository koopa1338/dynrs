use dynrs::{resolve, DynamicDns};
use ureq::{Agent, Error as UreqError, Response};

const RESOLVE_URL: &str = "http://checkip.spdns.de/";

pub struct Spdns<'d> {
    host: &'d str,
    username: &'d str,
    token: &'d str,
}

impl<'d> Spdns<'d> {
    pub fn new(host: &'d str, username: &'d str, token: &'d str) -> Self {
        Self {
            host,
            username,
            token,
        }
    }
}

impl DynamicDns for Spdns<'_> {
    fn update(&self, agent: &Agent) -> Result<Response, UreqError> {
        let ip = resolve(agent, Some(RESOLVE_URL));
        let host = self.host;
        let username = self.username;
        let token = self.token;
        let update_url = format!("https://update.spdyn.de/nic/update?hostname={host}&myip={ip}&user={username}&pass={token}");
        agent.get(&update_url).call()
    }
}
