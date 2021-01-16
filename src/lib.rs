use ureq::{Agent, Request};
use std::io::Error;

pub struct DynConfig<'a> {
    checkip_url: &'a str,
    service: Service,
    protocol: Protocol,
    username: &'a str,
    token: &'a str,
}


pub enum Protocol {
    Ipv4,
    Ipv6,
}

pub enum Service {
    Spdns,
    Dyndns,
}

impl DynConfig<'_> {
    pub fn new<'a>(
        checkip_url: &'a str,
        service: Service,
        protocol: Protocol,
        username: &'a str,
        token: &'a str,
    ) -> DynConfig<'a> {
        DynConfig {
            checkip_url,
            service,
            protocol,
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
    match config.protocol {
        Protocol::Ipv4 => {
            match config.service {
                Service::Spdns => {
                    update_url = format!("{}:{}@url/nic/update/{}", config.username, config.token, ip);
                    
                }
                Service::Dyndns => {
                    update_url = format!("{}:{}@url/nic/update/{}", config.username, config.token, ip);
                }
            }
        }
        Protocol::Ipv6 => {
            match config.service {
                Service::Spdns => {
                    update_url = format!("{}:{}@url/nic/update/{}", config.username, config.token, ip);
                    
                }
                Service::Dyndns => {
                    update_url = format!("{}:{}@url/nic/update/{}", config.username, config.token, ip);
                }
            }
        }
    }
    return agent.get(&update_url);
}
