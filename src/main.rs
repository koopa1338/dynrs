use dynrs;
use std::{thread, time};
use ureq::Agent;

fn main() {
    let agent = Agent::new();
    let url = "http://checkip.spdns.de/";
    // TODO: use config file to set the fields
    let config: dynrs::DynConfig =
        dynrs::DynConfig::new(url, "web", "username", "token", time::Duration::new(5, 0));
    loop {
        let ip = dynrs::get_ip(&agent, &config);
        println!("### IP IS: {}", ip);
        dynrs::update_ip(&agent, &config, &ip);
        thread::sleep(config.delay);
    }
}
