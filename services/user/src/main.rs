use actix_cors::Cors;
use actix_web::{
    middleware::Logger,
    web::{scope, ServiceConfig},
    App, HttpServer
};

use utils::*;

mod routes;
use routes::*;

/* #[macro_use]
extern crate dotenv_codegen;

const SESSION_KEY: &str = dotenv!(
    "SESSION_KEY",
    "Error getting the SESSION_KEY environment variable."
); */

fn app_config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/api/v1").service(update));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger_setup(); // Setup logger

    HttpServer::new(move || {
        let cors = Cors::permissive(); // Setup the CORS config
        let logger = Logger::new(highlight_logger().as_str()); // Setup the logger

        App::new()
            .wrap(cors)
            .wrap(logger)
            .configure(app_config)
    })
    .bind(("127.0.0.1", 8082))?
    .run()
    .await
}
