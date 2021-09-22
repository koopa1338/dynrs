#[macro_use]
extern crate dotenv_codegen;

use dotenv::dotenv;
use dynrs::{DynamicDns, Provider, PROVIDER_MAP};
use ureq::Agent;

mod provider;

fn main() {
    dotenv().ok();

    let prov = dotenv!("PROVIDER");
    let host = dotenv!("HOST");
    let token = dotenv!("TOKEN");

    let provider: Provider = match PROVIDER_MAP.get(prov) {
        Some(p) => *p,
        None => panic!("unsupported Provider!"),
    };

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
    match provider {
        Provider::Spdns => {
            let username = dotenv!("USERNAME");
            let handler = provider::spdns::Spdns {
                host,
                username,
                token,
            };
            handler.update(&agent).unwrap();
        },
        Provider::Dyndns => {
            let username = dotenv!("USERNAME");
            let handler = provider::dyndns::Dyndns {
                host,
                username,
                token,
            };
            handler.update(&agent).unwrap();
        },
        Provider::Duckdns => {
            let handler = provider::duckdns::DuckDns {
                host,
                token,
            };
            handler.update(&agent).unwrap();
        }
    };
}
