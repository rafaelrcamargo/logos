#![feature(is_some_and)]

use std::env;

use actix_cors::Cors;
use actix_session::{storage::RedisActorSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{Key, SameSite},
    middleware::Logger,
    web::{scope, Data, ServiceConfig},
    App, HttpServer
};

use dotenv::{dotenv, vars};
use redis::Client;

use auth::*;
use utils::*;

mod providers;
use providers::{discord, github};

fn app_config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api/v1/oauth")
            .service(
                scope("discord")
                    .service(discord::create)
                    .service(discord::resolve)
            )
            .service(
                scope("github")
                    .service(github::create)
                    .service(github::resolve)
            )
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load .env file
    logger_setup(); // Setup logger

    // Check if all the required environment variables are set
    check_env(vars().collect::<Vec<(String, String)>>());

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
        .cookie_same_site(SameSite::Strict)
        .build();

        App::new()
            .wrap(cors)
            .wrap(logger)
            .wrap(session)
            .configure(app_config)
            .app_data(Data::new(redis_connection.clone()))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}

fn redis_setup() -> (String, redis::Client) {
    let redis_url = env::var("REDIS_URL")
        .expect("Missing the REDIS_URL environment variable.");

    let redis_pool = Client::open(format!("redis://{redis_url}/"))
        .expect("Failed to connect to Redis");

    (redis_url, redis_pool)
}
