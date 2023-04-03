use actix_session::Session;
use actix_web::{get, web::Data, HttpRequest, HttpResponse, Responder};
use utils::debug;

use crate::config::Config;

struct User {
    id: String,
    role: String
}

impl User {
    fn new(session: Session) -> Self {
        let id = match session.get::<String>("id") {
            Ok(id) => id.unwrap_or(String::new()),
            Err(_) => String::new()
        };
        let role = match session.get::<String>("role") {
            Ok(role) => role.unwrap_or(String::new()),
            Err(_) => String::new()
        };

        Self { id, role }
    }
}

#[get("/{tail:.*}")]
pub async fn intercept(
    session: Session,
    config: Data<Config>,
    req: HttpRequest
) -> impl Responder {
    let uri = match req.headers().get("X-Original-URI") {
        Some(uri) => uri.to_str().unwrap(),
        None => return HttpResponse::BadRequest().finish()
    };

    let uri = uri.split('?').next().unwrap();
    let location = match config.locations.get(uri) {
        Some(location) => location,
        None => return HttpResponse::Forbidden().finish()
    };

    let user = User::new(session);
    debug!("UID: {}", user.id);

    if user.id.is_empty() {
        // TODO: Handle the case where the service is updating the user's
        return HttpResponse::Ok()
            .append_header(("X-User-Id", user.id))
            .finish();
    }

    if user.role == location.role {
        return HttpResponse::Ok()
            .append_header(("X-User-Id", user.id))
            .finish();
    }

    HttpResponse::Forbidden().finish()
}
