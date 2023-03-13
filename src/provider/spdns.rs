use dynrs::DynamicDns};
use ureq::{Agent, Error as UreqError, Response};

const RESOLVE_URL: &str = "http://checkip.spdns.de/";

pub struct Spdns<'d> {
    host: &'d str,
    username: &'d str,
    token: &'d str,
    ip: Option<String>,
}

impl<'d> Spdns<'d> {
    pub fn new(host: &'d str, username: &'d str, token: &'d str) -> Self {
        Self {
            host,
            username,
            token,
            ip: None,
        }
    }
}

impl<'d> ProviderTrait for Spdns<'d> {
    fn update_url(&self) -> &str {
        format!("https://update.spdyn.de/nic/update?hostname={host}&myip={ip}&user={username}&pass={token}", self.host, self.ip.unwrap(), self.username, self.token).sa_str()
    }
    fn update_ip(&mut self) {
        self.ip = self.fetch_ip()
    }
}

impl DynamicDns for Spdns<'_> {
    fn update(&self, agent: &Agent) -> Result<Response, UreqError> {
        let ip = resolve(agent, Some(RESOLVE_URL));
        agent.get(&update_url).call()
    }
}
