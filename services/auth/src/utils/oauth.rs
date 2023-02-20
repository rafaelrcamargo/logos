use std::env;

use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, TokenUrl};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct BasicResponse {
    pub code: String,
    pub state: String
}

// Create an OAuth2 client by specifying the client ID, client secret, authorization URL and token URL.
pub fn github_client() -> BasicClient {
    BasicClient::new(
        ClientId::new(
            env::var("GITHUB_CLIENT_ID")
                .expect("Missing the GITHUB_CLIENT_ID environment variable.")
        ),
        Some(ClientSecret::new(env::var("GITHUB_CLIENT_SECRET").expect(
            "Missing the GITHUB_CLIENT_SECRET environment variable."
        ))),
        AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
            .expect("Invalid authorization endpoint URL"),
        Some(
            TokenUrl::new(
                "https://github.com/login/oauth/access_token".to_string()
            )
            .expect("Invalid token endpoint URL")
        )
    )
}
