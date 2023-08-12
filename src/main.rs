use dynrs::{DnsClient, DynamicDns};
use ureq::Agent;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: Vec<_> = std::env::args_os().collect();
    args.remove(0);
    let config_path = args
        .get(0)
        .and_then(|s| s.to_str())
        .expect("No config path passed.");
    let client = DnsClient::new(config_path);
    let agent = Agent::new();
    let response = client.update(&agent)?;
    println!("Update response: {response:?}");
    Ok(())
}
