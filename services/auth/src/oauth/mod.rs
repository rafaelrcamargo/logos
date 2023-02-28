mod create;
pub use create::*;

mod resolve;
pub use resolve::*;

use actix_session::Session;
use anyhow::{anyhow, Result};
use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationRequest, ClientId, ClientSecret,
    CsrfToken, RedirectUrl, Scope, TokenUrl
};
use reqwest::{header::HeaderMap, Client as HTTPClient, Url};
use serde_json::{Map, Value};
use std::env;

pub enum Provider {
    Discord,
    Github,
    Spotify
}
impl Provider {
    pub fn from(provider: &str) -> Result<Provider> {
        match provider {
            "discord" => Ok(Provider::Discord),
            "github" => Ok(Provider::Github),
            "spotify" => Ok(Provider::Spotify),
            _ => Err(anyhow!("Invalid provider"))
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

#[non_exhaustive]
pub struct Api;
impl Api {
    pub fn oauth(provider: &Provider) -> (String, String) {
        match provider {
            Provider::Github => (
                String::from("https://github.com/login/oauth/authorize"),
                String::from("https://github.com/login/oauth/access_token")
            ),
            Provider::Discord => (
                String::from("https://discord.com/api/oauth2/authorize"),
                String::from("https://discord.com/api/oauth2/token")
            ),
            Provider::Spotify => (
                String::from("https://accounts.spotify.com/authorize"),
                String::from("https://accounts.spotify.com/api/token")
            )
        }
    }
    pub fn user(provider: &Provider) -> Url {
        match provider {
            Provider::Github => {
                Url::parse("https://api.github.com/user").unwrap()
            }
            Provider::Discord => {
                Url::parse("https://discord.com/api/v10/users/@me").unwrap()
            }
            Provider::Spotify => {
                Url::parse("https://api.spotify.com/v1/me").unwrap()
            }
        }
    }
}

pub struct OAuthClient {}
impl OAuthClient {
    pub fn from(provider: &Provider) -> BasicClient {
        let (auth_url, token_url) = Api::oauth(provider);
        let provider = provider.to_string();

        BasicClient::new(
            ClientId::new(
                env::var(format!("{}_CLIENT_ID", provider.to_uppercase()))
                    .unwrap()
            ),
            Some(ClientSecret::new(
                env::var(format!("{}_CLIENT_SECRET", provider.to_uppercase()))
                    .unwrap()
            )),
            AuthUrl::new(auth_url).unwrap(),
            Some(TokenUrl::new(token_url).unwrap())
        )
        .set_redirect_uri(
            RedirectUrl::new(
                "http://127.0.0.1:8081/api/v1/oauth/resolve".to_string()
            )
            .unwrap()
        )
    }

    pub fn create<'a>(
        provider: &'a Provider,
        client: &'a BasicClient
    ) -> AuthorizationRequest<'a> {
        match provider.to_string().as_str() {
            "github" => client
                .authorize_url(CsrfToken::new_random)
                .add_scope(Scope::new("read:user".to_string()))
                .add_scope(Scope::new("user:email".to_string())),
            "discord" => client
                .authorize_url(CsrfToken::new_random)
                .add_scope(Scope::new("email".to_string()))
                .add_scope(Scope::new("identify".to_string())),
            "spotify" => client
                .authorize_url(CsrfToken::new_random)
                .add_scope(Scope::new("user-read-email".to_string()))
                .add_scope(Scope::new("user-read-private".to_string())),
            _ => panic!("How???")
        }
    }
}

// HTTP
fn http(headers: HeaderMap) -> HTTPClient {
    HTTPClient::builder()
        .user_agent("logos-auth")
        .default_headers(headers)
        .build()
        .unwrap()
}

// User
async fn get_user(provider: Url, token: &String) -> Result<Value> {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Bearer {token}")
            .parse()
            .unwrap()
    );

    match http(headers).get(provider).send().await {
        Err(_) => Err(anyhow!("Failed to get user data")),
        Ok(response) => match response
            .json::<serde_json::Value>()
            .await
        {
            Err(_) => Err(anyhow!("Failed to parse user data")),
            Ok(user) => {
                println!("{user:?}");
                Ok(user)
            }
        }
    }
}
async fn save_user(provider: String, user: &Map<String, Value>) -> Result<()> {
    match http(HeaderMap::new())
        .post(&format!(
            "https://4006c06b-d8de-4361-8e53-6f7f2b431d32.mock.pstmn.io/api/v1/user?provider={provider}"
        ))
        .json::<Map<String, Value>>(user)
        .send()
        .await
    {
        Err(_) => Err(anyhow!("Failed to get user data")),
        Ok(_) => Ok(())
    }
}

// Session
pub fn is_valid_for(session: Session, provider: String) -> bool {
    if let Ok(og) = session.get::<String>("provider") {
        og.is_some_and(|x| x == provider)
    } else {
        false
    }
}
