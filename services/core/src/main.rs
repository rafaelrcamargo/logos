use actix_cors::Cors;
use actix_web::{middleware::Logger, web::scope, App, HttpServer};

use utils::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger_setup();

    HttpServer::new(|| {
        // Middlewares
        let cors = Cors::permissive();
        let logger = Logger::new(highlight_logger().as_str());

        // Scopes and services
        let v1 = scope("v1").service(ping);
        let api = scope("api").service(v1);

        App::new()
            .wrap(cors)
            .wrap(logger)
            .service(api)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// Routes
use actix_web::{get, HttpResponse, Responder};

#[get("/ping")]
async fn ping() -> impl Responder { HttpResponse::Ok().body("Pong!") }
