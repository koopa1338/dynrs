use dynrs::{Handler, Provider, FALLBACK_URL, PROVIDER_MAP};
use ureq::Agent;

fn main() {
    let mut settings = config::Config::default();
    // TODO: specify config file as parameter otherwise fallback to default path.
    settings.merge(config::File::with_name("config")).unwrap();

    let server_url = match settings.get_str("server_url") {
        Ok(url) => url,
        Err(_) => FALLBACK_URL.to_string(),
    };

    let provider: Provider = match settings.get_str("provider") {
        Ok(provider) => match PROVIDER_MAP.get(provider.as_str()) {
            Some(p) => *p,
            None => Provider::Spdns,
        },
        Err(_) => panic!("No provider specified"),
    };

    let host = match settings.get_str("host") {
        Ok(host) => host,
        Err(_) => panic!("No host found in config file"),
    };

    let username = match settings.get_str("username") {
        Ok(username) => username,
        Err(_) => panic!("No username found in config file"),
    };

    let token = match settings.get_str("token") {
        Ok(token) => token,
        Err(_) => panic!("No token found in config file"),
    };

    let ipv6 = settings.get_bool("ipv6").unwrap_or(false);

    let handler: Handler = Handler::new(provider, ipv6, server_url);

    let agent = Agent::new();
    handler.update(&agent, &host, &username, &token);
}
