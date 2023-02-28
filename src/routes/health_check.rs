use std::collections::HashMap;

use actix_web::{get, HttpResponse, Responder};

#[utoipa::path(
    responses(
        (status = 200, description = "Returns true")
    ),
)]
#[get("/health-check")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HashMap::new().insert("status", "true"))
}
