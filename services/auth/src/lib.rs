#![feature(is_some_and)]

use actix_session::Session;
use actix_web::web::Data;
use dotenv_codegen::dotenv;
use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationRequest, ClientId, ClientSecret,
    CsrfToken, RedirectUrl, Scope, TokenUrl
};
use reqwest::{Client as HTTPClient, Url};
use serde_json::{Map, Value};

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
    pub fn get_env(&self) -> (&str, &str) {
        match self {
            Provider::Github => (
                dotenv!("GITHUB_CLIENT_ID", "Error getting the GITHUB_CLIENT_ID environment variable."),
                dotenv!("GITHUB_CLIENT_SECRET", "Error getting the GITHUB_CLIENT_SECRET environment variable."),
            ),
            Provider::Discord => (
                dotenv!("DISCORD_CLIENT_ID", "Error getting the DISCORD_CLIENT_ID environment variable."),
                dotenv!("DISCORD_CLIENT_SECRET", "Error getting the DISCORD_CLIENT_SECRET environment variable."),
            ),
            Provider::Spotify => (
                dotenv!("SPOTIFY_CLIENT_ID", "Error getting the SPOTIFY_CLIENT_ID environment variable."),
                dotenv!("SPOTIFY_CLIENT_SECRET", "Error getting the SPOTIFY_CLIENT_SECRET environment variable."),
            )
        }
    }
    pub fn get_oauth_url(&self) -> (&str, &str) {
        match self {
            Provider::Github => (
                "https://github.com/login/oauth/authorize",
                "https://github.com/login/oauth/access_token"
            ),
            Provider::Discord => (
                "https://discord.com/api/oauth2/authorize",
                "https://discord.com/api/oauth2/token"
            ),
            Provider::Spotify => (
                "https://accounts.spotify.com/authorize",
                "https://accounts.spotify.com/api/token"
            )
        }
    }
    pub fn get_user_url(&self) -> Url {
        match self {
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
impl ToString for Provider {
    fn to_string(&self) -> String {
        match self {
            Provider::Github => "github".to_string(),
            Provider::Discord => "discord".to_string(),
            Provider::Spotify => "spotify".to_string()
        }
    }
}

pub struct OAuthClient {}
impl OAuthClient {
    pub fn from(provider: &Provider) -> BasicClient {
        let (auth_url, token_url) = provider.get_oauth_url();
        let (client_id, client_secret) = provider.get_env();

        BasicClient::new(
            ClientId::new(client_id.to_owned()),
            Some(ClientSecret::new(client_secret.to_owned())),
            AuthUrl::new(auth_url.to_owned()).unwrap(),
            Some(TokenUrl::new(token_url.to_owned()).unwrap())
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

// User
pub async fn get_user(
    http: Data<HTTPClient>,
    provider: &Provider,
    token: &String
) -> Result<Value, ()> {
    match http
        .get(provider.get_user_url())
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await
    {
        Err(_) => Err(()),
        Ok(response) => match response
            .json::<serde_json::Value>()
            .await
        {
            Err(_) => Err(()),
            Ok(user) => {
                println!("{user:?}");
                Ok(user)
            }
        }
    }
}
pub async fn save_user(
    http: Data<HTTPClient>,
    provider: &Provider,
    user: &Map<String, Value>
) -> Result<(), ()> {
    match http
        .post(&format!(
            "https://4006c06b-d8de-4361-8e53-6f7f2b431d32.mock.pstmn.io/api/v1/user?provider={}",
            provider.to_string()
        ))
        .json::<Map<String, Value>>(user)
        .send()
        .await
    {
        Err(_) => Err(()),
        Ok(_) => Ok(())
    }
}

// Session
pub fn is_valid_for(session: &Session, provider: String) -> bool {
    if let Ok(og) = session.get::<String>("provider") {
        og.is_some_and(|x| x == provider)
    } else {
        false
    }
}
