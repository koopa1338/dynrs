use ureq::{Agent, Request};
use std::io::Error;

pub const UPDATE_URL: &str = "http://checkip.spdns.de/";

pub struct DynConfig<'a> {
    checkip_url: &'a str,
    service: Service,
    username: &'a str,
    token: &'a str
}



pub enum Service {
    Spdns { ipv6: bool },
    Dyndns { ipv6: bool },
}

impl DynConfig<'_> {
    pub fn new<'a>(
        checkip_url: &'a str,
        service: Service,
        username: &'a str,
        token: &'a str,
    ) -> DynConfig<'a> {
        DynConfig {
            checkip_url,
            service,
            username,
            token,
        }
    }
}

pub fn get_ip<'a>(agent: &Agent, config: &'a DynConfig) -> Result<String, Error> {
    agent.get(config.checkip_url).call().into_string()
}

pub fn update_ip<'a>(agent: &Agent, config: &'a DynConfig, ip: &str) -> Request {
    let update_url: String;
    match config.service {
        Service::Spdns {ipv6} => {
            if ipv6 {
                update_url = format!("{}:{}@url/nic/update/{}", config.username, config.token, ip);
            } else {
                update_url = format!("{}:{}@url/nic/update/{}", config.username, config.token, ip);
            }
        }
        Service::Dyndns {ipv6} => {
            if ipv6 {
                update_url = format!("{}:{}@url/nic/update/{}", config.username, config.token, ip);
            } else {
                update_url = format!("{}:{}@url/nic/update/{}", config.username, config.token, ip);
            }
        }
    }
    return agent.get(&update_url);
}
