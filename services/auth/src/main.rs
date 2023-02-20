use actix_cors::Cors;
use actix_web::{
    middleware::Logger,
    web::{scope, Data, Query},
    App, HttpServer,
};

use redis::{Client, Commands, Connection};

use auth::*;

use actix_web::get;
use actix_web::{HttpResponse, Responder};

use dotenv::dotenv;
use serde::Deserialize;
use std::{
    env,
    sync::{Arc, Mutex},
};

use oauth2::{basic::BasicClient, reqwest::async_http_client, AuthorizationCode, PkceCodeVerifier};
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope, TokenUrl,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger_setup();

    HttpServer::new(move || {
        // Redis
        let client = Client::open("redis://localhost:6380/").expect("Failed to connect to Redis");

        let connection = client
            .get_connection()
            .expect("Failed to get Redis connection");

        let redis = Arc::new(Mutex::new(connection));

        // Middlewares
        let cors = Cors::permissive();
        let logger = Logger::new(highlight_logger().as_str());

        // Scopes and services
        let v1 = scope("v1").service(start).service(end);
        let api = scope("api").service(v1);

        App::new()
            .app_data(Data::new(redis))
            .wrap(cors)
            .wrap(logger)
            .service(api)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}

#[get("/")]
async fn start(redis: Data<Arc<Mutex<Connection>>>) -> impl Responder {
    dotenv().ok();

    let github_client_id = ClientId::new(
        env::var("GITHUB_CLIENT_ID").expect("Missing the GITHUB_CLIENT_ID environment variable."),
    );
    let github_client_secret = ClientSecret::new(
        env::var("GITHUB_CLIENT_SECRET")
            .expect("Missing the GITHUB_CLIENT_SECRET environment variable."),
    );
    let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
        .expect("Invalid token endpoint URL");

    // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and token URL.
    let client = BasicClient::new(
        github_client_id,
        Some(github_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(
        // Set the URL the user will be redirected to after the authorization process.
        RedirectUrl::new("http://127.0.0.1:8081/api/v1/return".to_string())
            .expect("Invalid redirect URL"),
    );

    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("read:user".to_string()))
        .add_scope(Scope::new("user:email".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    // This is the URL you should redirect the user to, in order to trigger the authorization process.
    println!("Browse to: {auth_url}");

    println!("{}: {}", csrf_token.secret(), pkce_verifier.secret());

    let _ = redis
        .lock()
        .expect("Failed to lock Redis connection")
        .set_ex::<&str, &str, String>(csrf_token.secret(), pkce_verifier.secret(), 600)
        .expect("Failed to set Redis key");

    HttpResponse::Ok().body(auth_url.to_string())
}

#[derive(Deserialize)]
struct OAuth {
    code: String,
    state: String,
}

#[get("/return")]
async fn end(redis: Data<Arc<Mutex<Connection>>>, oauth: Query<OAuth>) -> impl Responder {
    dotenv().ok();

    let github_client_id = ClientId::new(
        env::var("GITHUB_CLIENT_ID").expect("Missing the GITHUB_CLIENT_ID environment variable."),
    );
    let github_client_secret = ClientSecret::new(
        env::var("GITHUB_CLIENT_SECRET")
            .expect("Missing the GITHUB_CLIENT_SECRET environment variable."),
    );
    let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
        .expect("Invalid token endpoint URL");

    // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and token URL.
    let client = BasicClient::new(
        github_client_id,
        Some(github_client_secret),
        auth_url,
        Some(token_url),
    );

    let state = oauth.state.clone();
    let code = oauth.code.clone();

    let pkce_verifier = redis
        .lock()
        .expect("Failed to lock Redis connection")
        .get::<&str, String>(state.as_str())
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
        .del::<&str, usize>(state.as_str())
        .expect("Failed to set Redis key");

    println!("Token: {token_result:#?}");
    HttpResponse::Ok()
}
