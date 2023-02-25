use actix_session::Session;
use actix_web::{
    get,
    web::{Data, Query},
    HttpResponse, Responder
};

use oauth2::{
    reqwest::async_http_client, AuthorizationCode, CsrfToken,
    PkceCodeChallenge, PkceCodeVerifier, Scope, TokenResponse
};

use redis::Client as RedisClient;

use crate::providers::*;

#[get("/create")]
pub async fn create(
    redis: Data<RedisClient>,
    session: Session
) -> impl Responder {
    let provider = Provider::Discord;
    has_valid_from(session, provider.to_string());

    let client = OauthClient::from(
        provider.to_string(),
        "https://discord.com/api/oauth2/authorize",
        "https://discord.com/api/oauth2/token"
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
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("identify".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    if (redis::Cmd::set_ex::<&str, &str>(
        csrf_token.secret(),
        pkce_verifier.secret(),
        600
    )
    .query_async::<_, String>(&mut conn)
    .await)
        .is_err()
    {
        return HttpResponse::InternalServerError().finish();
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
    let provider = Provider::Discord;
    let client = OauthClient::from(
        provider.to_string(),
        "https://discord.com/api/oauth2/authorize",
        "https://discord.com/api/oauth2/token"
    );

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

    if session
        .insert("provider", provider.to_string())
        .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    }

    // Get the user data
    let user = get_user_from(
        Api::from(provider),
        token_result.access_token().secret()
    )
    .await;

    // Create session
    let uid = match user {
        Err(_) => return HttpResponse::Forbidden().finish(),
        Ok(user) => user["id"].to_string()
    };

    if session.insert("id", uid).is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    // Return a redirect to the frontend w/ the session
    HttpResponse::TemporaryRedirect()
        .append_header(("Location", "http://127.0.0.1:3000"))
        .finish()
}

// fn create_oauth_provider(provider: Provider) -> impl Responder {
// ...
// }
//
