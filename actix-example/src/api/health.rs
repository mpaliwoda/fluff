use actix_web::{get, HttpResponse, Responder};
use serde_json;

#[get("/health")]
pub async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "success" }))
}
