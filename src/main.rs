#[macro_use]
extern crate dotenv_codegen;

mod provider;
use provider::{duckdns::DuckDns, dyndns::Dyndns, noip::Noip, spdns::Spdns};

use dotenv::dotenv;
use dynrs::{DynamicDns, Provider, ProviderType};
use ureq::Agent;

fn main() {
    dotenv().ok();

    let host = dotenv!("HOST");
    let token = dotenv!("TOKEN");

    let provider_type: ProviderType = dotenv!("PROVIDER").into();

    /* REVIEW: config crate not compiling, switched to dotenv for now

    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("config")).unwrap();

    let server_url = match settings.get_str("server_url") {
        Ok(url) => url,
        Err(_) => FALLBACK_URL.to_string(),
    };

    let provider: Provider = match settings.get_str("provider") {
        Ok(provider) => match PROVIDER_MAP.get(provider.as_str()) {
            Some(p) => *p, None => Provider::Spdns,
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
    */

    let agent = Agent::new();
    let provider: Provider = match provider_type {
        ProviderType::Spdns => {
            let username = dotenv!("USERNAME");
            let host_data = Spdns::new(host, username, token);
            Provider::new(host_data)
        }
        ProviderType::Dyndns => {
            let username = dotenv!("USERNAME");
            let host_data = Dyndns::new(host, username, token);
            Provider::new(host_data)
        }
        ProviderType::Duckdns => {
            let host_data = DuckDns::new(host, token);
            Provider::new(host_data)
        }
        ProviderType::Noipdns => {
            let username = dotenv!("USERNAME");
            let host_data = Noip::new(host, username, token);
            Provider::new(host_data)
        }
    };
}
