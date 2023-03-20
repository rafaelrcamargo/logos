// ? OAuth utils
pub enum Provider {
    Discord,
    Github,
    Spotify
}
impl Provider {
    pub fn from(provider: &str) -> Result<Provider, &str> {
        match provider {
            "discord" => Ok(Provider::Discord),
            "github" => Ok(Provider::Github),
            "spotify" => Ok(Provider::Spotify),
            _ => Err("Invalid provider")
        }
    }
}
impl ToString for Provider {
    fn to_string(&self) -> String {
        match self {
            Provider::Github => "github".to_string(),
            Provider::Discord => "discord".to_string(),
            Provider::Spotify => "spotify".to_string()
        }
    }
}

pub fn sanitize(s: String) -> String { s.replace('\"', "") }
