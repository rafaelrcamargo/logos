use std::{
    env,
    sync::{Arc, Mutex}
};

use actix_cors::Cors;
use actix_session::{storage::RedisActorSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{Key, SameSite},
    middleware::Logger,
    web::{scope, Data},
    App, HttpServer
};

use dotenv::dotenv;
use redis::Client;

mod providers;
use auth::utils::logger::*;
use providers::github::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load .env file
    logger_setup(); // Setup logger

    HttpServer::new(move || {
        // Session setup
        let session_key = env::var("SESSION_KEY")
            .expect("Missing the SESSION_KEY environment variable.");

        // Redis setup
        let redis_url = env::var("REDIS_URL")
            .expect("Missing the REDIS_URL environment variable.");
        let redis_client = Client::open(format!("redis://{redis_url}/"))
            .expect("Failed to connect to Redis");
        let redis_connection = redis_client
            .get_connection()
            .expect("Failed to get Redis connection");

        // Middlewares
        let cors = Cors::permissive();
        let logger = Logger::new(highlight_logger().as_str());

        // Scopes and services
        let oauth = scope("oauth").service(
            scope("github")
                .service(create)
                .service(resolve)
        );

        let v1 = scope("v1").service(oauth);
        let api = scope("api").service(v1);

        App::new()
            .app_data(Data::new(Arc::new(Mutex::new(redis_connection))))
            .wrap(
                SessionMiddleware::builder(
                    RedisActorSessionStore::new(redis_url),
                    Key::from(session_key.as_bytes())
                )
                .cookie_http_only(false)
                .cookie_same_site(SameSite::Strict)
                .build()
            )
            .wrap(cors)
            .wrap(logger)
            .service(api)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
