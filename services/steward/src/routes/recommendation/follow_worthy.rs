use actix_web::{get, web::Data, HttpRequest, HttpResponse, Responder};
use neo4rs::{query as graph_query, Graph, Node};
use serde_json::json;
use std::sync::Arc;
use utils::error;

#[get("/follow-worthy")]
pub async fn follow_worthy(
    req: HttpRequest,
    graph: Data<Arc<Graph>>
) -> impl Responder {
    let id = match req.headers().get("X-User-Id") {
        Some(id) => id.to_str().unwrap(),
        None => return HttpResponse::Forbidden().finish()
    };

    let mut candidates = Vec::new();

    match graph
        .execute(
            graph_query("MATCH (user: User { id: $id })-[:Follows]->(followeed: User) MATCH (followeed)-[:Follows]->(recommended: User) WHERE NOT (user)-[:Follows]->(recommended) RETURN recommended")
                .param("id", id.to_string())
        )
        .await
    {
        Err(e) => {
            error!("{:?}", e);
        }
        Ok(mut result) => {
            while let Ok(Some(row)) = result.next().await {
                let node: Node = row.get("recommended").unwrap();
                candidates.push(json!({
                    "id": node.get::<String>("id").unwrap(),
                    "name": node.get::<String>("name").unwrap(),
                    "image": node.get::<String>("image").unwrap()
                }))
            }
        }
    }

    HttpResponse::Ok().json(candidates)
}
