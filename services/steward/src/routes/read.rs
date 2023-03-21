use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder
};
use neo4rs::{query as graph_query, Graph, Node};
use serde_json::json;
use std::sync::Arc;
use utils::error;

#[get("/{id}")]
pub async fn read(id: Path<String>, graph: Data<Arc<Graph>>) -> impl Responder {
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
            let mut user = json!({});
            if let Ok(Some(row)) = result.next().await {
                let node: Node = row.get("user").unwrap();
                user = json!({
                    "email": node.get::<String>("email").unwrap(),
                    "username": node.get::<String>("username").unwrap(),
                    "image": node.get::<String>("image").unwrap(),
                    "verified": &node.get::<String>("verified").unwrap() == "true",
                });
            }
            HttpResponse::Ok().json(user)
        }
    }
}
