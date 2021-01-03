use std::time;
use ureq::{Agent, Request};

pub struct DynConfig<'a> {
    url: &'a str,
    protocol: &'a str,
    username: &'a str,
    token: &'a str,
    delay: time::Duration,
}

impl DynConfig<'_> {
    pub fn new<'a>(
        url: &'a str,
        protocol: &'a str,
        username: &'a str,
        token: &'a str,
        delay: time::Duration,
    ) -> DynConfig<'a> {
        return DynConfig {
            url,
            protocol,
            username,
            token,
            delay,
        };
    }

    pub fn get_url(&self) -> &str {
        self.url
    }

    pub fn get_protocol(&self) -> &str {
        self.protocol
    }

    pub fn get_username(&self) -> &str {
        self.username
    }

    pub fn get_token(&self) -> &str {
        self.token
    }

    pub fn get_delay(&self) -> time::Duration {
        self.delay
    }
}

pub fn get_ip<'a>(agent: &Agent, config: &'a DynConfig) -> String {
    match config.get_protocol() {
        "web" => {
            let response = agent.get(config.get_url()).call();
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
        config.get_username(),
        config.get_token(),
        ip
    );
    return agent.post(&update_url);
}
