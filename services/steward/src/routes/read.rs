use actix_web::{get, web::Data, HttpRequest, HttpResponse, Responder};
use neo4rs::{query as graph_query, Graph, Node};
use serde_json::json;
use std::sync::Arc;
use utils::error;

#[get("")]
pub async fn read(req: HttpRequest, graph: Data<Arc<Graph>>) -> impl Responder {
    let id = match req.headers().get("X-User-Id") {
        Some(id) => id.to_str().unwrap(),
        None => return HttpResponse::Forbidden().finish()
    };

    match graph
        .execute(
            graph_query("MATCH (user: User { id: $id }) RETURN user")
                .param("id", id.to_string())
        )
        .await
    {
        Err(e) => {
            error!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        }
        Ok(mut result) => {
            if let Ok(Some(row)) = result.next().await {
                let node: Node = row.get("user").unwrap();
                HttpResponse::Ok().
                    content_type("application/json").
                    body(json!({
                            "email": node.get::<String>("email").unwrap(),
                            "username": node.get::<String>("username").unwrap(),
                            "image": node.get::<String>("image").unwrap(),
                            "verified": &node.get::<String>("verified").unwrap() == "true"
                        }).to_string()
                    )
            } else {
                HttpResponse::NotFound().finish()
            }
        }
    }
}
