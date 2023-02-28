
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

use crate::routes::health_check::__path_health_check;

#[derive(OpenApi)]
#[openapi(
    // paths(
    //     health_check
    // ),
    tags(
        (name = "Zero2Prod", description = "Zero to Prod Book.")
    ),
    // components(), 
    // // modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

// struct SecurityAddon;

// impl Modify for SecurityAddon {
//     fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
//         let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
//         components.add_security_scheme(
//             "api_key",
//             SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
//         )
//     }
// }


// pub fn initialize_openapi() -> utoipa::openapi::OpenApi  {
//     ApiDoc::openapi()
// }