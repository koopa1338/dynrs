use dynrs::DnsConfig;
use ureq::Agent;

fn main() {
    let mut args: Vec<_> = std::env::args_os().collect();
    args.remove(0);
    let config_path = args
        .get(0)
        .and_then(|s| s.to_str())
        .expect("No config path passed.");
    let config = DnsConfig::new(config_path);
    let agent = Agent::new();
    config.run(&agent);
}
