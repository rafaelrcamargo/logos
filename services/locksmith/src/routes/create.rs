use actix_session::Session;
use actix_web::{
    get,
    web::{Data, Query},
    HttpResponse, Responder
};
use locksmith::{OAuthClient, Provider};
use oauth2::PkceCodeChallenge;
use redis::Client as RedisClient;
use serde::Deserialize;
use utils::{error, warn};

#[derive(Deserialize)]
pub struct Create {
    provider: String
}

#[get("/create")]
pub async fn create(
    query: Query<Create>,
    redis: Data<RedisClient>,
    session: Session
) -> impl Responder {
    if is_valid_for(&session) {
        println!("ID: {:?}", session.get::<String>("id"));

        return HttpResponse::TemporaryRedirect()
            .append_header(("Location", "http://127.0.0.1:3000"))
            .finish();
    }

    let provider = match Provider::from(&query.provider) {
        Err(e) => {
            warn!("Bad Request: {e:?}");
            return HttpResponse::BadRequest().finish();
        }
        Ok(provider) => provider
    };

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
        Err(e) => {
            error!("Error getting Redis connection: {e:?}");
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

// Session skipping
pub fn is_valid_for(session: &Session) -> bool {
    if let Ok(id) = session.get::<String>("id") {
        id.is_some()
    } else {
        false
    }
}
