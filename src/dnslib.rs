use super::config;

use ureq::{Agent, Request};

pub fn get_ip<'a>(agent: &Agent, config: &'a config::DynConfig) -> String {
    match config.get_protocol() {
        "web" => {
            let response = agent.get(config.get_url()).call();
            return response.into_string().unwrap();
        },
        "if" => {
            todo!();
        }
        _ => {
            todo!()
        }
    }
}

pub fn update_ip<'a>(agent: &Agent, config: &'a config::DynConfig, ip: &str) -> Request {
    // TODO: get the right url maybe an enum?
    let update_url = format!("{}:{}@url/nic/update/{}", config.get_username(), config.get_token(), ip);
    return agent.post(&update_url);
}
