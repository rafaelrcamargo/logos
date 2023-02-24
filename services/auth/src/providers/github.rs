use std::env;

use actix_session::Session;
use actix_web::{
    get,
    web::{Data, Query},
    HttpResponse, Responder
};

use anyhow::{anyhow, Result};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode,
    ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier,
    RedirectUrl, Scope, TokenResponse, TokenUrl
};

use crate::providers::BasicResponse;

use redis::Client as RedisClient;
use reqwest::{header::HeaderMap, Client as HTTPClient};

#[get("/create")]
pub async fn create(
    redis: Data<RedisClient>,
    session: Session
) -> impl Responder {
    match session.get::<u32>("id") {
        Err(_) => {
            return HttpResponse::TemporaryRedirect()
                .append_header(("Location", "http://127.0.0.1:3000"))
                .finish()
        }
        _ => ()
    }

    let client = github_client().set_redirect_uri(
        RedirectUrl::new(
            "http://127.0.0.1:8081/api/v1/oauth/github/resolve".to_string()
        )
        .unwrap()
    );

    let mut conn = match redis
        .get_tokio_connection_manager()
        .await
    {
        Err(_) => return HttpResponse::ServiceUnavailable().finish(),
        Ok(conn) => conn
    };

    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) =
        PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("read:user".to_string()))
        .add_scope(Scope::new("user:email".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    match redis::Cmd::set_ex::<&str, &str>(
        csrf_token.secret(),
        pkce_verifier.secret(),
        600
    )
    .query_async::<_, String>(&mut conn)
    .await
    {
        Err(_) => return HttpResponse::InternalServerError().finish(),
        _ => ()
    }

    // Return a redirect to the frontend w/ the session
    HttpResponse::TemporaryRedirect()
        .append_header(("Location", auth_url.to_string()))
        .finish()
}

#[get("/resolve")]
pub async fn resolve(
    redis: Data<RedisClient>,
    query: Query<BasicResponse>,
    session: Session
) -> impl Responder {
    let client = github_client();

    let mut conn = match redis
        .get_tokio_connection_manager()
        .await
    {
        Err(_) => return HttpResponse::ServiceUnavailable().finish(),
        Ok(conn) => conn
    };

    let pkce_verifier = match redis::Cmd::get_del::<&str>(query.state.as_str())
        .query_async::<_, String>(&mut conn)
        .await
    {
        Err(_) => return HttpResponse::InternalServerError().finish(),
        Ok(pkce_verifier) => pkce_verifier
    };

    // Generate a PKCE.
    let pkce = PkceCodeVerifier::new(pkce_verifier);

    // Now you can trade it for an access token.
    let token_result = match client
        .exchange_code(AuthorizationCode::new(query.code.to_string()))
        // Set the PKCE code verifier.
        .set_pkce_verifier(pkce)
        .request_async(async_http_client)
        .await
    {
        Err(_) => return HttpResponse::Forbidden().finish(),
        Ok(token_result) => token_result
    };

    // Get the user data from GitHub
    let user = get_user(token_result.access_token().secret()).await;

    // Create session
    let uid = match user {
        Err(_) => return HttpResponse::Forbidden().finish(),
        Ok(user) => user["id"].as_u64().unwrap()
    };

    session.insert("id", uid).unwrap();

    // Return a redirect to the frontend w/ the session
    HttpResponse::TemporaryRedirect()
        .append_header(("Location", "http://127.0.0.1:3000"))
        .finish()
}

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

fn http(headers: HeaderMap) -> HTTPClient {
    HTTPClient::builder()
        .user_agent("logos-auth")
        .default_headers(headers)
        .build()
        .unwrap()
}

async fn get_user(token: &String) -> Result<serde_json::Value> {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Bearer {token}")
            .parse()
            .unwrap()
    );

    match http(headers)
        .get("https://api.github.com/user")
        .send()
        .await
    {
        Err(_) => Err(anyhow!("Failed to get user data from GitHub")),
        Ok(response) => match response
            .json::<serde_json::Value>()
            .await
        {
            Err(_) => Err(anyhow!("Failed to parse user data from GitHub")),
            Ok(user) => Ok(user)
        }
    }
}
