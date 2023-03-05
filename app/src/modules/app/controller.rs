use std::convert::Infallible;

use app_core::traits::Controller;
use axum::{
    response::{IntoResponse, Response},
    routing::MethodRouter,
};

pub struct HealthCheckController {}

impl Controller for HealthCheckController {
    fn name(&self) -> &'static str {
        "HealthCheckController"
    }

    fn base_path(&self) -> &'static str {
        "/app"
    }

    fn register_routes<S, B>(&self) -> Vec<(String, MethodRouter<S, B, Infallible>)>
    where
        B: axum::body::HttpBody + Send + Sync + 'static,
        S: Clone + Send + Sync + 'static,
    {
        vec![("health-check".to_string(), axum::routing::get(health_check))]
    }
}

#[utoipa::path(
    get,
    path = "/health-check",
    responses(
        (status = 200, description = "Returns true")
    ),
)]
pub async fn health_check() -> Response {
    ("true").into_response()
}
