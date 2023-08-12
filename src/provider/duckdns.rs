use crate::{DnsConfig, DynamicDns};
use ureq::{Agent, Error as UreqError, Response};

pub struct DuckDns {
    host: String,
    token: String,
}

impl DuckDns {
    #[must_use]
    pub fn new(config: DnsConfig) -> Self {
        Self {
            host: config.host,
            token: config.token,
        }
    }
}

impl DynamicDns for DuckDns {
    fn update(&self, agent: &Agent) -> Result<Response, UreqError> {
        //NOTE: duckdns will detect our ip address if we do not pass one.
        let update_url = format!(
            "https://www.duckdns.org/update?domains={}&token={}",
            self.host, self.token
        );
        agent.get(&update_url).call()
    }
}
