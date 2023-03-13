use ureq::{Agent, Error as UreqError, Response};

pub const FALLBACK_URL: &str = "http://checkip.spdns.de/";

#[derive(Debug, Clone)]
pub enum ProviderType {
    Spdns,
    Dyndns,
    Duckdns,
    Noipdns,
}

impl From<&str> for ProviderType {
    fn from(value: &str) -> Self {
        match value {
            "spdns" => Self::Spdns,
            "dyndns" => Self::Dyndns,
            "duckdns" => Self::Duckdns,
            "noip" => Self::Noipdns,
            _ => unimplemented!("not supported provider."),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Provider<T: ProviderTrait> {
    host_data: T,
}

impl<T: ProviderTrait> Provider<T> {
    pub fn new(host_data: T) -> Self {
        Self { host_data }
    }
}

pub trait ProviderTrait {
    fn update_url(&self) -> &str;
    fn update_ip(&self) {}
    fn fetch_ip(&self, agent: &Agent) -> Option<String> {
        //TODO: handle ipv4 and ipv6
        Some(
            agent
                .get("http://checkip.spdns.de/")
                .call()
                .unwrap()
                .into_string()
                .expect("No response from resolving."),
        )
    }
}

pub trait DynamicDns {
    fn update(&self, agent: &Agent) -> Result<Response, UreqError>;
    fn resolve(&self, agent: &Agent, url: Option<&str>) -> String;
}

impl<T> DynamicDns for Provider<T>
where
    T: ProviderTrait,
{
    fn update(&self, agent: &Agent) -> Result<Response, UreqError> {
        self.host_data.update_ip(agent);
        agent.get(self.host_data.update_url()).call()
    }

    fn resolve(&self, agent: &Agent, url: Option<&str>) -> String {
        agent
            .get(url.unwrap_or(FALLBACK_URL))
            .call()
            .unwrap()
            .into_string()
            .expect("No response from resolving.")
    }
}
