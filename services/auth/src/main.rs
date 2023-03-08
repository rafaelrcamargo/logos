mod oauth;

use actix_cors::Cors;
use actix_session::{storage::RedisActorSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{Key, SameSite},
    middleware::Logger,
    web::{scope, Data, ServiceConfig},
    App, HttpServer
};

use redis::{Client as RedisClient};
use reqwest::Client as HTTPClient;

use utils::*;

#[macro_use]
extern crate dotenv_codegen;

const SESSION_KEY: &str = dotenv!(
    "SESSION_KEY",
    "Error getting the SESSION_KEY environment variable."
);

const REDIS_URL: &str = dotenv!(
    "REDIS_URL",
    "Error getting the REDIS_URL environment variable."
);

fn app_config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api/v1/oauth")
            .service(oauth::create)
            .service(oauth::resolve)
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger_setup(); // Setup logger

    // HTTP client setup
    let http = HTTPClient::default();

    // Redis setup
    let redis = RedisClient::open(format!("redis://{REDIS_URL}/"))
        .expect("Error connecting to Redis.");

    HttpServer::new(move || {
        let cors = Cors::permissive(); // Setup the CORS config
        let logger = Logger::new(highlight_logger().as_str()); // Setup the logger

        // Setup the session middleware builder
        let session = SessionMiddleware::builder(
            RedisActorSessionStore::new(REDIS_URL),
            Key::from(SESSION_KEY.as_bytes())
        )
        .cookie_http_only(false)
        .cookie_same_site(SameSite::Lax)
        .build();

        App::new()
            .wrap(cors)
            .wrap(logger)
            .wrap(session)
            .configure(app_config)
            .app_data(Data::new(http.clone()))
            .app_data(Data::new(redis.clone()))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
