use std::time;

pub struct DynConfig<'a> {
    url: &'a str,
    protocol: &'a str,
    username: &'a str,
    token: &'a str,
    delay: time::Duration,
}

impl DynConfig<'_> {
    pub fn new<'a>(url: &'a str, protocol: &'a str, username: &'a str, token: &'a str, delay: time::Duration) -> DynConfig<'a> {
        return DynConfig {
            url,
            protocol,
            username,
            token,
            delay,
        }
    }

    pub fn get_url(&self) -> &str {
        self.url
    }

    pub fn get_protocol(&self) -> &str {
        self.protocol
    }

    pub fn get_username(&self) -> &str {
        self.username
    }

    pub fn get_token(&self) -> &str {
        self.token
    }

    pub fn get_delay(&self) -> time::Duration {
        self.delay
    }
}
