use crate::oauth::{get_user, save_user, OAuthClient, Provider};
use actix_session::Session;
use actix_web::{
    get,
    web::{Data, Query},
    HttpResponse, Responder
};
use oauth2::{
    reqwest::async_http_client, AuthorizationCode, PkceCodeVerifier,
    TokenResponse
};
use serde::Deserialize;
use utils::{error, info, warn};
use uuid::Uuid;

use redis::Client as RedisClient;
use reqwest::Client as HTTPClient;

#[derive(Deserialize)]
pub struct OAuthResolve {
    code: String,
    state: String
}

#[get("/resolve")]
pub async fn resolve(
    query: Query<OAuthResolve>,
    redis: Data<RedisClient>,
    http: Data<HTTPClient>,
    session: Session
) -> impl Responder {
    let provider = match session.get::<String>("provider") {
        Ok(Some(provider)) => {
            info!("Provider: {}", provider.to_string());
            match Provider::from(&provider) {
                Ok(provider) => provider,
                Err(e) => {
                    warn!("{}", format!("Bad Request: {e}"));
                    return HttpResponse::BadRequest().finish();
                }
            }
        }
        Ok(None) => {
            warn!("Bad Request: No provider found in session");
            return HttpResponse::BadRequest().finish();
        }
        Err(_) => {
            error!("Error getting _ from session");
            return HttpResponse::InternalServerError().finish();
        }
    };

    let client = OAuthClient::from(&provider);

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

    let pkce_verifier = match redis::Cmd::get_del::<&str>(query.state.as_str())
        .query_async::<_, String>(&mut conn)
        .await
    {
        Err(_) => {
            error!("Error getting Redis key");
            return HttpResponse::InternalServerError().finish();
        }
        Ok(pkce) => PkceCodeVerifier::new(pkce)
    };

    let token_result = match client
        .exchange_code(AuthorizationCode::new(query.code.to_string()))
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await
    {
        Err(_) => {
            error!("Error retrieving USER token from OAuth provider");
            return HttpResponse::Forbidden().finish();
        }
        Ok(token_result) => token_result
    };

    // Create a new UUID
    let id = Uuid::new_v4().to_string();

    // Get the user data
    let user = match get_user(
        http.clone(),
        &provider,
        token_result.access_token().secret()
    )
    .await
    {
        Err(_) => {
            error!("Error getting USER data from OAuth provider");
            return HttpResponse::InternalServerError().finish();
        }
        Ok(user) => match user.as_object() {
            Some(user) => {
                let mut user = user.clone();
                user.insert("id".to_string(), id.clone().into());
                user
            }
            None => {
                error!("Error parsing USER data");
                return HttpResponse::InternalServerError().finish();
            }
        }
    };

    if session.insert("id", id).is_err() {
        error!("Error creating USER session");
        return HttpResponse::InternalServerError().finish();
    }

    match save_user(http, &provider, &user).await {
        Err(_) => {
            error!("Error saving USER data");
            HttpResponse::InternalServerError().finish()
        }
        Ok(_) => HttpResponse::TemporaryRedirect()
            .append_header(("Location", "http://127.0.0.1:3000"))
            .finish()
    }
}
