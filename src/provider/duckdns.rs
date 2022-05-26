use dynrs::DynamicDns;
use ureq::{Agent, Error as UreqError, Response};

pub struct DuckDns<'d> {
    host: &'d str,
    token: &'d str,
}

impl<'d> DuckDns<'d> {
    pub fn new(host: &'d str, token: &'d str) -> Self {
        Self { host, token }
    }
}

impl DynamicDns for DuckDns<'_> {
    fn update(&self, agent: &Agent) -> Result<Response, UreqError> {
        let host = self.host;
        let token = self.token;
        //NOTE: duckdns will detect our ip address if we do not pass one.
        let update_url = format!("https://www.duckdns.org/update?domains={host}&token={token}");
        agent.get(&update_url).call()
    }
}
