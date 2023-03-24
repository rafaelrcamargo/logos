use actix_session::Session;
use actix_web::{get, HttpRequest, HttpResponse, Responder};

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
pub async fn intercept(session: Session, req: HttpRequest) -> impl Responder {
    let uri = match req.headers().get("X-Original-URI") {
        Some(uri) => uri.to_str().unwrap(),
        None => return HttpResponse::Forbidden().finish()
    };

    let user = User::new(session);
    if uri == "/api/v1/user" && user.role == "user" {
        return HttpResponse::Ok()
            .append_header(("X-User-Id", user.id))
            .finish();
    } else if user.id.is_empty() {
        return HttpResponse::Ok().finish();
    }

    HttpResponse::Forbidden().finish()
}
