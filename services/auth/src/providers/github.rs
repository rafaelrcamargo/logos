use std::sync::{Arc, Mutex};

use actix_session::Session;
use actix_web::{
    get,
    web::{Data, Query},
    HttpResponse, Responder
};

use oauth2::{
    reqwest::async_http_client, AuthorizationCode, CsrfToken,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenResponse
};

use auth::utils::oauth::{github_client, BasicResponse};
use redis::{Commands, Connection};
use reqwest::{header::HeaderMap, Client};

#[get("/create")]
pub async fn create(redis: Data<Arc<Mutex<Connection>>>) -> impl Responder {
    let client = github_client().set_redirect_uri(
        RedirectUrl::new(
            "http://127.0.0.1:8081/api/v1/oauth/github/resolve".to_string()
        )
        .expect("Invalid redirect URL")
    );

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

    let _ = redis
        .lock()
        .expect("Failed to lock Redis connection")
        .set_ex::<&str, &str, String>(
            csrf_token.secret(),
            pkce_verifier.secret(),
            600
        )
        .expect("Failed to set Redis key");

    // Return a redirect to the frontend w/ the session
    HttpResponse::PermanentRedirect()
        .append_header(("Location", auth_url.to_string()))
        .finish()
}

#[get("/resolve")]
pub async fn resolve(
    redis: Data<Arc<Mutex<Connection>>>,
    session: Session,
    query: Query<BasicResponse>
) -> impl Responder {
    let client = github_client();

    let state = query.state.as_str();
    let code = query.code.to_string();

    let pkce_verifier = redis
        .lock()
        .expect("Failed to lock Redis connection")
        .get::<&str, String>(state)
        .expect("Failed to set Redis key");

    // Generate a PKCE.
    let pkce = PkceCodeVerifier::new(pkce_verifier);

    // Now you can trade it for an access token.
    let token_result = client
        .exchange_code(AuthorizationCode::new(code))
        // Set the PKCE code verifier.
        .set_pkce_verifier(pkce)
        .request_async(async_http_client)
        .await
        .expect("Failed to get token");

    // Delete the PKCE verifier from Redis
    let _ = redis
        .lock()
        .expect("Failed to lock Redis connection")
        .del::<&str, usize>(state)
        .expect("Failed to delete Redis key");

    // Get the user data from GitHub
    let user = get_user(token_result.access_token().secret()).await;

    // Create session
    session
        .insert(
            "id",
            user.get("id")
                .unwrap()
                .as_u64()
                .unwrap()
        )
        .unwrap();

    // Return a redirect to the frontend w/ the session
    HttpResponse::PermanentRedirect()
        .append_header(("Location", "http://127.0.0.1:3000"))
        .finish()
}

fn http(headers: HeaderMap) -> Client {
    Client::builder()
        .user_agent("logos-auth")
        .default_headers(headers)
        .build()
        .unwrap()
}

async fn get_user(token: &String) -> serde_json::Value {
    // Get the user data from GitHub
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Bearer {token}")
            .parse()
            .unwrap()
    );

    http(headers)
        .get("https://api.github.com/user")
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap()
}
