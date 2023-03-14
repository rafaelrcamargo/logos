use std::sync::Arc;

use actix_web::{
    post,
    web::{Data, Query},
    HttpResponse, Responder
};
use neo4rs::{query as graph_query, Graph};
use serde::Deserialize;
use serde_json::Value;
use user::Provider;
use utils::error;

#[derive(Deserialize)]
pub struct Update {
    provider: String
}

#[post("/user")]
pub async fn update(
    query: Query<Update>,
    body: String,
    graph: Data<Arc<Graph>>
) -> impl Responder {
    let provider = match Provider::from(&query.provider) {
        Ok(provider) => provider,
        Err(_) => return HttpResponse::BadRequest().finish()
    };

    let user = User::from(provider, body);
    println!("{user:?}");

    match graph
        .run(
            graph_query("CREATE (u:User {id: $id, email: $email, locale: $locale, mfa_enabled: $mfa_enabled, username: $username, name: $name, verified: $verified, image: $image, misc: $misc}) RETURN u")
                .param("id", user.id.unwrap_or_default())
                .param("email", user.email.unwrap_or_default())
                .param("locale", user.locale.unwrap_or_default())
                .param("mfa_enabled", user.mfa_enabled.unwrap_or_default().to_string())
                .param("username", user.username.unwrap_or_default())
                .param("name", user.name.unwrap_or_default())
                .param("verified", user.verified.unwrap_or_default().to_string())
                .param("image", user.image.unwrap_or_default())
                .param("misc", user.misc.unwrap_or_default())
        )
        .await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(e) => {
                error!("{:?}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
}

#[derive(Debug)]
struct User {
    id: Option<String>,
    email: Option<String>,
    locale: Option<String>,
    mfa_enabled: Option<bool>,
    username: Option<String>,
    name: Option<String>,
    verified: Option<bool>,
    image: Option<String>,
    misc: Option<String>
}

impl User {
    pub fn from(provider: Provider, body: String) -> User {
        let json: Value = serde_json::from_str(&body).unwrap();

        match provider {
            Provider::Discord => User::from_discord(&json),
            Provider::Github => User::from_github(&json),
            Provider::Spotify => User::from_spotify(&json)
        }
    }
    fn from_discord(json: &Value) -> User {
        User {
            id: Some(json["id"].to_string()),
            email: to_string_or_none(&json["email"]),
            locale: to_string_or_none(&json["locale"]),
            mfa_enabled: json["mfa_enabled"].as_bool(),
            username: to_string_or_none(&json["username"]),
            name: to_string_or_none(&json["display_name"]),
            verified: json["verified"].as_bool(),
            image: Some(format!(
                "https://cdn.discordapp.com/avatars/{}/{}.png",
                json["id"], json["avatar"]
            )),
            misc: Some(json.to_string())
        }
    }
    fn from_github(json: &Value) -> User {
        User {
            id: Some(json["id"].to_string()),
            email: to_string_or_none(&json["email"]),
            locale: None,
            mfa_enabled: json["two_factor_authentication"].as_bool(),
            username: to_string_or_none(&json["login"]),
            name: to_string_or_none(&json["name"]),
            verified: None,
            image: to_string_or_none(&json["avatar_url"]),
            misc: Some(json.to_string())
        }
    }
    fn from_spotify(json: &Value) -> User {
        User {
            id: Some(json["id"].to_string()),
            email: to_string_or_none(&json["email"]),
            locale: None,
            mfa_enabled: None,
            username: to_string_or_none(&json["id"]),
            name: to_string_or_none(&json["display_name"]),
            verified: None,
            image: to_string_or_none(&json["images"][0]["url"]),
            misc: Some(json.to_string())
        }
    }
}

fn to_string_or_none(value: &Value) -> Option<String> {
    if value.is_null() {
        None
    } else {
        Some(value.to_string())
    }
}
