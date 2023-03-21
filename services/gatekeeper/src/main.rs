use actix_cors::Cors;
use actix_session::{
    storage::RedisActorSessionStore, Session, SessionMiddleware
};
use actix_web::{
    cookie::{Key, SameSite},
    get,
    middleware::Logger,
    web::{self, scope, ServiceConfig},
    App, HttpResponse, HttpServer, Responder
};
use utils::*;

#[macro_use]
extern crate dotenv_codegen;

const SESSION_KEY: &str = dotenv!("SESSION_KEY");
const REDIS_URL: &str = dotenv!("REDIS_URL");

fn app_config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/api/v1").service(intercept));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger_setup(); // Setup logger

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
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

struct User {
    id: String,
    role: String
}

impl User {
    fn new() -> User {
        User {
            id: String::new(),
            role: String::new()
        }
    }
}

#[get("/{tail:.*}")]
async fn intercept(
    path: web::Path<String>,
    session: Session
) -> impl Responder {
    println!("Path: {}", path);

    let mut user = User::new();

    println!("Provider: {:?}", session.get::<String>("provider"));
    println!("Session: {:?}", session.get::<String>("id"));

    if let Ok(Some(id)) = session.get::<String>("id") {
        println!("ID: {}", id);
        user.id = id;
    }

    if let Ok(Some(role)) = session.get::<String>("role") {
        println!("Role: {}", role);
        user.role = role;
    }

    if path.contains("me") && user.role == "user" {
        return HttpResponse::TemporaryRedirect()
            .append_header(("Location", "/api/v1/users/me"))
            .append_header(("X-User-Id", user.id))
            .finish();
    }

    HttpResponse::Forbidden().finish()
}
