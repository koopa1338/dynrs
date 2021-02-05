use dynrs::{DynConfig, Service, get_ip, update_ip, UPDATE_URL};
use ureq::Agent;

fn main() {
    let mut settings = config::Config::default();
    // TODO: specify config file as parameter otherwise fallback to default path.
    settings.merge(config::File::with_name("config")).unwrap();

    let checkip_url = match settings.get_str("check_url") {
        Ok(url) => url,
        Err(_) => UPDATE_URL.to_string(),
    };
    let username = match settings.get_str("username") {
        Ok(username) => username,
        Err(_) => panic!("No username found in config file"),
    };

    let token = match settings.get_str("token") {
        Ok(token) => token,
        Err(_) => panic!("No token found in config file"),
    };

    let ipv6 = match settings.get_bool("ipv6") {
        Ok(ipv6) => ipv6,
        Err(_) => false,
    };

    let config: DynConfig = DynConfig::new(&checkip_url, Service::Spdns{ipv6}, &username, &token);

    let agent = Agent::new();
    let ip = get_ip(&agent, &config).unwrap();
    println!("### IP IS: {}", ip);
    update_ip(&agent, &config, &ip);
}
