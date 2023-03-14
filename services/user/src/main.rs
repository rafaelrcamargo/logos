use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{
    middleware::Logger,
    web::{scope, Data, ServiceConfig},
    App, HttpServer
};
use neo4rs::Graph;
use utils::*;
mod routes;
use routes::*;

#[macro_use]
extern crate dotenv_codegen;

const NEO4J_URL: &str = dotenv!(
    "NEO4J_URL",
    "Error getting the NEO4J_URL environment variable."
);

const NEO4J_PASSWORD: &str = dotenv!(
    "NEO4J_PASSWORD",
    "Error getting the NEO4J_PASSWORD environment variable."
);

fn app_config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/api/v1").service(update));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger_setup(); // Setup logger

    let graph = Arc::new(
        Graph::new(NEO4J_URL, "neo4j", NEO4J_PASSWORD)
            .await
            .unwrap()
    );

    HttpServer::new(move || {
        let cors = Cors::permissive(); // Setup the CORS config
        let logger = Logger::new(highlight_logger().as_str()); // Setup the logger

        App::new()
            .wrap(cors)
            .wrap(logger)
            .configure(app_config)
            .app_data(Data::new(graph.clone()))
    })
    .bind(("127.0.0.1", 8082))?
    .run()
    .await
}
