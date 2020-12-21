use std::{thread, time};
use ureq::Agent;

fn main() {
    let agent = Agent::new();
    let url = "http://checkip.spdns.de/";
    loop {
        let response = agent.get(url).call();
        println!("{}", response.into_string().unwrap());
        thread::sleep(time::Duration::new(5, 0));
    }
}
