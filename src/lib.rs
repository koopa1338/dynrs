use std::time;
use ureq::{Agent, Request};

pub struct DynConfig<'a> {
    url: &'a str,
    protocol: &'a str,
    username: &'a str,
    token: &'a str,
    pub delay: time::Duration,
}

impl DynConfig<'_> {
    pub fn new<'a>(
        url: &'a str,
        protocol: &'a str,
        username: &'a str,
        token: &'a str,
        delay: time::Duration,
    ) -> DynConfig<'a> {
        DynConfig {
            url,
            protocol,
            username,
            token,
            delay,
        }
    }
}

pub fn get_ip<'a>(agent: &Agent, config: &'a DynConfig) -> String {
    match config.protocol {
        "web" => {
            let response = agent.get(config.url).call();
            return response.into_string().unwrap();
        }
        "if" => {
            todo!();
        }
        _ => todo!(),
    }
}

pub fn update_ip<'a>(agent: &Agent, config: &'a DynConfig, ip: &str) -> Request {
    // TODO: get the right url maybe an enum?
    let update_url = format!(
        "{}:{}@url/nic/update/{}",
        config.username,
        config.token,
        ip
    );
    return agent.post(&update_url);
}
