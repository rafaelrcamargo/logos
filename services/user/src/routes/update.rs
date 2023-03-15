use std::sync::Arc;

use actix_web::{
    post,
    web::{Data, Query},
    HttpResponse, Responder
};
use neo4rs::{query as graph_query, Graph};
use serde::Deserialize;
use serde_json::{json, Value};
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
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            error!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        }
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
        let json = json!(body);

        match provider {
            Provider::Discord => User::from_discord(&json),
            Provider::Github => User::from_github(&json),
            Provider::Spotify => User::from_spotify(&json)
        }
    }
    fn from_discord(json: &Value) -> User {
        User {
            id: json["id"].to_string(),
            email: json["email"].to_string(),
            locale: json["locale"].to_string(),
            mfa_enabled: json["mfa_enabled"].to_string(),
            username: json["username"].to_string(),
            name: json["display_name"].to_string(),
            verified: json["verified"].to_string(),
            image: format!(
                "https://cdn.discordapp.com/avatars/{}/{}.png",
                json["id"], json["avatar"],
            ),
            misc: json.to_string()
        }
    }
    fn from_github(json: &Value) -> User {
        User {
            id: json["id"].to_string(),
            email: json["email"].to_string(),
            locale: String::from(""),
            mfa_enabled: json["two_factor_authentication"].to_string(),
            username: json["login"].to_string(),
            name: json["name"].to_string(),
            verified: String::from(""),
            image: json["avatar_url"].to_string(),
            misc: json.to_string()
        }
    }
    fn from_spotify(json: &Value) -> User {
        User {
            id: json["id"].to_string(),
            email: json["email"].to_string(),
            locale: String::from(""),
            mfa_enabled: String::from(""),
            username: json["display_name"].to_string(),
            name: json["display_name"].to_string(),
            verified: String::from(""),
            image: json["images"][0]["url"].to_string(),
            misc: json.to_string()
        }
    }
}
