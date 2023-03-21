use actix_web::{
    patch,
    web::{Data, Query},
    HttpResponse, Responder
};
use neo4rs::{query as graph_query, Graph};
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;
use steward::{sanitize, Provider};
use utils::error;

#[derive(Deserialize)]
pub struct Update {
    provider: String
}

#[patch("")]
pub async fn update(
    body: String,
    query: Query<Update>,
    graph: Data<Arc<Graph>>
) -> impl Responder {
    let provider = match Provider::from(&query.provider) {
        Ok(provider) => provider,
        Err(_) => return HttpResponse::BadRequest().finish()
    };

    let user = User::from(provider, body);

    match graph
        .run(
            graph_query("MERGE (u: User { email: $email }) SET u.id = $id, u.email = $email, u.locale = $locale, u.mfa_enabled = $mfa_enabled, u.username = $username, u.name = $name, u.verified = $verified, u.image = $image, u.misc = $misc")
                .param("id", user.id)
                .param("email", user.email)
                .param("locale", user.locale)
                .param("mfa_enabled", user.mfa_enabled)
                .param("username", user.username)
                .param("name", user.name)
                .param("verified", user.verified)
                .param("image", user.image)
                .param("misc", user.misc)
        )
        .await
    {
        Err(e) => {
            error!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        }
        Ok(_) => HttpResponse::Ok().finish(),
    }
}

#[derive(Debug)]
struct User {
    id: String,
    email: String,
    locale: String,
    mfa_enabled: String,
    username: String,
    name: String,
    verified: String,
    image: String,
    misc: String
}

impl User {
    pub fn from(provider: Provider, body: String) -> User {
        let json = serde_json::from_str(&body).unwrap();

        match provider {
            Provider::Discord => User::from_discord(&json),
            Provider::Github => User::from_github(&json),
            Provider::Spotify => User::from_spotify(&json)
        }
    }
    fn from_discord(json: &Value) -> User {
        User {
            id: sanitize(json["id"].to_string()),
            email: sanitize(json["email"].to_string()),
            locale: sanitize(json["locale"].to_string()),
            mfa_enabled: sanitize(json["mfa_enabled"].to_string()),
            username: sanitize(json["username"].to_string()),
            name: sanitize(json["display_name"].to_string()),
            verified: sanitize(json["verified"].to_string()),
            image: format!(
                "https://cdn.discordapp.com/avatars/{}/{}.png",
                sanitize(json["id"].to_string()),
                sanitize(json["avatar"].to_string()),
            ),
            misc: json.to_string()
        }
    }
    fn from_github(json: &Value) -> User {
        User {
            id: sanitize(json["id"].to_string()),
            email: sanitize(json["email"].to_string()),
            locale: String::from(""),
            mfa_enabled: sanitize(
                json["two_factor_authentication"].to_string()
            ),
            username: sanitize(json["login"].to_string()),
            name: sanitize(json["name"].to_string()),
            verified: String::from(""),
            image: sanitize(json["avatar_url"].to_string()),
            misc: json.to_string()
        }
    }
    fn from_spotify(json: &Value) -> User {
        User {
            id: sanitize(json["id"].to_string()),
            email: sanitize(json["email"].to_string()),
            locale: String::from(""),
            mfa_enabled: String::from(""),
            username: sanitize(json["display_name"].to_string()),
            name: sanitize(json["display_name"].to_string()),
            verified: String::from(""),
            image: sanitize(json["images"][0]["url"].to_string()),
            misc: json.to_string()
        }
    }
}
