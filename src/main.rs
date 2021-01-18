use dynrs::{DynConfig, Service, get_ip, update_ip};
use ureq::Agent;

fn main() {
    let agent = Agent::new();
    // TODO: use config file to set the fields
    let checkip_url = "http://checkip.spdns.de/";
    let config: DynConfig = DynConfig::new(checkip_url, Service::Spdns{ipv6: false}, "username", "token");
    let ip = get_ip(&agent, &config).unwrap();
    println!("### IP IS: {}", ip);
    update_ip(&agent, &config, &ip);
}
