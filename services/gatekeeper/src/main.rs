use actix_cors::Cors;
use actix_session::{storage::RedisActorSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{Key, SameSite},
    middleware::{Logger, NormalizePath, TrailingSlash},
    web::{scope, Data, ServiceConfig},
    App, HttpServer
};
use utils::*;

mod config;
mod routes;

#[macro_use]
extern crate dotenv_codegen;

const SESSION_KEY: &str = dotenv!("SESSION_KEY");
const REDIS_URL: &str = dotenv!("REDIS_URL");

use crate::config::Config;

fn app_config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/api/v1").service(routes::intercept));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger_setup(); // Setup logger

    let config: Config = confy::load_path(config::CONFIG_FILE_NAME).unwrap(); // Load the config

    HttpServer::new(move || {
        let cors = Cors::permissive(); // Setup the CORS config
        let logger = Logger::new(highlight_logger().as_str()); // Setup the logger
        let normalize = NormalizePath::new(TrailingSlash::Always); // Setup the path normalizer

        // Setup the session middleware builder
        let session = SessionMiddleware::builder(
            RedisActorSessionStore::new(REDIS_URL),
            Key::from(SESSION_KEY.as_bytes())
        )
        .cookie_secure(false)
        .cookie_http_only(false)
        .cookie_same_site(SameSite::Lax)
        .build();

        App::new()
            .wrap(cors)
            .wrap(logger)
            .wrap(session)
            .wrap(normalize)
            .app_data(Data::new(config.clone()))
            .configure(app_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
