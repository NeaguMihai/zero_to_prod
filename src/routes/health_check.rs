use axum::response::{IntoResponse, Response};

#[utoipa::path(
    get,
    path = "/health-check",
    responses(
        (status = 200, description = "Returns true")
    ),
)]
pub fn health_check() -> Response<Vec<u8>> {
    Response::body(vec!([]))
}
