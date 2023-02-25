pub mod discord;
pub mod github;

use std::env;

use anyhow::{anyhow, Result};
use reqwest::{header::HeaderMap, Client as HTTPClient, Url};

use actix_session::Session;
use serde::Deserialize;

use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl
};
use serde_json::Value;

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
                    .expect(
                        format!(
                            "Missing the {}_CLIENT_ID environment variable.",
                            provider.to_uppercase()
                        )
                        .as_str()
                    )
            ),
            Some(ClientSecret::new(
                env::var(format!("{}_CLIENT_SECRET", provider.to_uppercase()))
                    .expect(
                        format!(
                        "Missing the {}_CLIENT_SECRET environment variable.",
                        provider.to_uppercase()
                    )
                        .as_str()
                    )
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
                "http://127.0.0.1:8081/api/v1/oauth/{}/resolve",
                provider
            ))
            .expect("Invalid redirect URL")
        )
    }
}

pub fn has_valid_from(session: Session, provider: String) -> bool {
    if let Ok(og) = session.get::<String>("provider") {
        if og.is_some_and(|x| x == provider) {
            return true;
        } else {
            return false;
        }
    } else {
        return false;
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
    Github
}

impl ToString for Provider {
    fn to_string(&self) -> String {
        match self {
            Provider::Discord => "discord".to_string(),
            Provider::Github => "github".to_string()
        }
    }
}

#[non_exhaustive]
pub struct API;

impl API {
    pub fn from(provider: Provider) -> Url {
        match provider {
            Provider::Discord => {
                Url::parse("https://discord.com/api/v10/users/@me")
                    .expect("Invalid Discord API URL")
            }
            Provider::Github => Url::parse("https://api.github.com/user")
                .expect("Invalid Github API URL")
        }
    }
}

async fn get_user_from(provider: Url, token: &String) -> Result<Value> {
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
