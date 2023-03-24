use actix_cors::Cors;
use actix_session::{storage::RedisActorSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{Key, SameSite},
    middleware::{Logger, NormalizePath, TrailingSlash},
    web::{scope, ServiceConfig},
    App, HttpServer
};
use utils::*;

mod routes;

#[macro_use]
extern crate dotenv_codegen;

const SESSION_KEY: &str = dotenv!("SESSION_KEY");
const REDIS_URL: &str = dotenv!("REDIS_URL");

fn app_config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/api/v1").service(routes::intercept));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger_setup(); // Setup logger

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
            .configure(app_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
