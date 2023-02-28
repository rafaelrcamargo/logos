use crate::oauth::{is_valid_for, OAuthClient, Provider};
use actix_session::Session;
use actix_web::{
    get,
    web::{Data, Query},
    HttpResponse, Responder
};
use oauth2::PkceCodeChallenge;
use redis::Client as RedisClient;
use serde::Deserialize;
use utils::{error, info, warn};

#[derive(Deserialize)]
pub struct OAuthCreate {
    provider: String
}

#[get("/create")]
pub async fn create(
    query: Query<OAuthCreate>,
    redis: Data<RedisClient>,
    session: Session
) -> impl Responder {
    let provider = match Provider::from(&query.provider) {
        Ok(provider) => {
            info!("Provider: {}", provider.to_string());
            provider
        }
        Err(e) => {
            warn!("{}", format!("Bad Request: {e}"));
            return HttpResponse::BadRequest().finish();
        }
    };

    if is_valid_for(&session, provider.to_string()) {
        info!("Redirecting to /");
        return HttpResponse::TemporaryRedirect()
            .append_header(("Location", "http://127.0.0.1:3000"))
            .finish();
    }

    if session
        .insert("provider", provider.to_string())
        .is_err()
    {
        error!("Error setting provider in session");
        return HttpResponse::InternalServerError().finish();
    }

    let mut conn = match redis
        .get_tokio_connection_manager()
        .await
    {
        Err(_) => {
            error!("Error getting Redis connection");
            return HttpResponse::ServiceUnavailable().finish();
        }
        Ok(conn) => conn
    };

    let client = OAuthClient::from(&provider);

    let (pkce_challenge, pkce_verifier) =
        PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = OAuthClient::create(&provider, &client)
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
        error!("Error setting Redis key");
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::TemporaryRedirect()
        .append_header(("Location", auth_url.to_string()))
        .finish()
}
