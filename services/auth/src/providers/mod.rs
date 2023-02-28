pub mod discord;
pub mod github;
pub mod spotify;

use std::env;

use anyhow::{anyhow, Result};
use reqwest::{header::HeaderMap, Client as HTTPClient, Url};

use actix_session::Session;
use serde::Deserialize;

use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl
};
use serde_json::{Map, Value};

#[derive(Deserialize)]
pub struct BasicResponse {
    pub code: String,
    pub state: String
}

pub struct OauthClient {}
impl OauthClient {
    pub fn from(
        provider: String,
        auth_url: &str,
        token_url: &str
    ) -> BasicClient {
        BasicClient::new(
            ClientId::new(
                env::var(format!("{}_CLIENT_ID", provider.to_uppercase()))
                    .unwrap_or_else(|_| {
                        panic!(
                            "Missing the {}_CLIENT_ID environment variable.",
                            provider.to_uppercase()
                        )
                    })
            ),
            Some(ClientSecret::new(
                env::var(format!("{}_CLIENT_SECRET", provider.to_uppercase()))
                    .unwrap_or_else(|_| {
                        panic!(
                            "Missing the {}_CLIENT_ID environment variable.",
                            provider.to_uppercase()
                        )
                    })
            )),
            AuthUrl::new(auth_url.to_owned())
                .expect("Invalid authorization endpoint URL"),
            Some(
                TokenUrl::new(token_url.to_owned())
                    .expect("Invalid token endpoint URL")
            )
        )
        .set_redirect_uri(
            RedirectUrl::new(format!(
                "http://127.0.0.1:8081/api/v1/oauth/{provider}/resolve"
            ))
            .expect("Invalid redirect URL")
        )
    }
}

pub fn has_valid_from(session: Session, provider: String) -> bool {
    if let Ok(og) = session.get::<String>("provider") {
        og.is_some_and(|x| x == provider)
    } else {
        false
    }
}

fn http(headers: HeaderMap) -> HTTPClient {
    HTTPClient::builder()
        .user_agent("logos-auth")
        .default_headers(headers)
        .build()
        .unwrap()
}

pub enum Provider {
    Discord,
    Github,
    Spotify
}

impl ToString for Provider {
    fn to_string(&self) -> String {
        match self {
            Provider::Discord => "discord".to_string(),
            Provider::Github => "github".to_string(),
            Provider::Spotify => "spotify".to_string()
        }
    }
}

#[non_exhaustive]
pub struct Api;

impl Api {
    pub fn from(provider: &Provider) -> Url {
        match provider {
            Provider::Discord => {
                Url::parse("https://discord.com/api/v10/users/@me").unwrap()
            }
            Provider::Github => {
                Url::parse("https://api.github.com/user").unwrap()
            }
            Provider::Spotify => {
                Url::parse("https://api.spotify.com/v1/me").unwrap()
            }
        }
    }
}

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
