use dynrs::DynamicDns;
use ureq::{Agent, Error as UreqError, Response};

pub struct DuckDns<'d> {
    pub host: &'d str,
    pub token: &'d str,
}

impl DynamicDns for DuckDns<'_> {
    fn update(&self, agent: &Agent) -> Result<Response, UreqError> {
        //NOTE: duckdns will detect our ip address if we do not pass one.
        let update_url = format!("https://www.duckdns.org/update?domains={}&token={}", self.host, self.token);
        agent.get(&update_url).call()
    }
}
