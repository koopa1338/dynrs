mod config;

use std::{thread, time};
use ureq::Agent;

fn main() {
    let agent = Agent::new();
    let url = "http://checkip.spdns.de/";
    let config: config::DynConfig = config::DynConfig::new(
        url,
        "web",
        false,
        "username",
        "token",
        time::Duration::new(5, 0));
    loop {
        let ip = dyndns::get_ip(&agent, &config);
        println!("### IP IS: {}", ip);
        dyndns::update_ip(&agent, &config, &ip);
        thread::sleep(config.get_delay);
    }
}
