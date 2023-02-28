use crate::routes::health_check::__path_health_check;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        health_check
    ),
    tags(
        (name = "Zero2Prod", description = "Zero to Prod Book.")
    ),
    components(),
)]
struct ApiDoc;

pub fn initialize_openapi() -> utoipa::openapi::OpenApi {
    ApiDoc::openapi()
}
