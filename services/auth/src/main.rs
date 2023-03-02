mod oauth;

use actix_cors::Cors;
use actix_session::{storage::RedisActorSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{Key, SameSite},
    middleware::Logger,
    web::{scope, Data, ServiceConfig},
    App, HttpServer
};
use dotenv::dotenv;
use redis::Client as RedisClient;
use reqwest::Client as HTTPClient;
use std::env;
use utils::*;

fn app_config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api/v1/oauth")
            .service(oauth::create)
            .service(oauth::resolve)
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load .env file
    logger_setup(); // Setup logger

    // Check if all the required environment variables are set
    check_env(vec![
        ("REDIS_URL".to_string(), false),
        ("SESSION_KEY".to_string(), false),
        ("GITHUB_CLIENT_ID".to_string(), false),
        ("GITHUB_CLIENT_SECRET".to_string(), false),
        ("DISCORD_CLIENT_ID".to_string(), false),
        ("DISCORD_CLIENT_SECRET".to_string(), false),
        ("SPOTIFY_CLIENT_ID".to_string(), false),
        ("SPOTIFY_CLIENT_SECRET".to_string(), false),
    ]);

    // Session setup
    let session_key = env::var("SESSION_KEY")
        .expect("Missing the SESSION_KEY environment variable.");

    // Redis setup
    let (redis_url, redis_connection) = redis_setup();

    HttpServer::new(move || {
        let cors = Cors::permissive(); // Setup the CORS config
        let logger = Logger::new(highlight_logger().as_str()); // Setup the logger

        // Setup the session middleware builder
        let session = SessionMiddleware::builder(
            RedisActorSessionStore::new(redis_url.clone()),
            Key::from(session_key.as_bytes())
        )
        .cookie_http_only(false)
        .cookie_same_site(SameSite::Lax)
        .build();

        let http = HTTPClient::builder()
            .user_agent("logos-auth")
            .build()
            .unwrap();

        App::new()
            .wrap(cors)
            .wrap(logger)
            .wrap(session)
            .configure(app_config)
            .app_data(Data::new(http))
            .app_data(Data::new(redis_connection.clone()))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}

fn redis_setup() -> (String, redis::Client) {
    let redis_url = env::var("REDIS_URL")
        .expect("Missing the REDIS_URL environment variable.");

    let redis_pool = RedisClient::open(format!("redis://{redis_url}/"))
        .expect("Failed to connect to Redis");

    (redis_url, redis_pool)
}
