use axum::{
    response::{IntoResponse, Response},
};
use macros::get;

pub struct HealthCheckController {}


#[utoipa::path(
    get,
    path = "/health-check",
    responses(
        (status = 200, description = "Returns true")
    ),
)]
// #[get("health-check")]
pub async fn health_check() -> Response {
    ("true").into_response()
}
