use actix_web::{post, web::Query, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::Value;
use user::Provider;

#[derive(Deserialize)]
pub struct Update {
    provider: String
}

#[post("/user")]
pub async fn update(query: Query<Update>, body: String) -> impl Responder {
    let provider = match Provider::from(&query.provider) {
        Ok(provider) => provider,
        Err(_) => return HttpResponse::BadRequest().finish()
    };

    let user = User::from(provider, body);
    println!("{user:?}");

    HttpResponse::Ok().finish()
}

#[allow(dead_code)]
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
