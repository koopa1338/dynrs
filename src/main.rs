use dynrs::DnsConfig;
use ureq::Agent;

fn main() {
    let config = DnsConfig::new();
    let agent = Agent::new();
    config.run(&agent);
}
