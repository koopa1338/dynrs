use dynrs::ProviderTrait;

pub struct Dyndns<'d> {
    host: &'d str,
    username: &'d str,
    token: &'d str,
    ip: Option<String>,
}

impl<'d> Dyndns<'d> {
    pub fn new(host: &'d str, username: &'d str, token: &'d str) -> Self {
        Self {
            host,
            username,
            token,
            ip: None,
        }
    }
}

impl<'d> ProviderTrait for Dyndns<'d> {
    fn update_url(&self) -> &str {
        format!(
            "https://{}:{}@members.dyndns.org/v3/update?hostname={}&myip={}",
            self.username,
            self.token,
            self.host,
            self.ip.unwrap(),
        )
        .as_str()
    }
}
