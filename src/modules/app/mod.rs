use axum::Router;

use crate::common::core::traits::Module;
use crate::common::utils::register_routes;

pub mod controller;

pub struct AppModule;

impl Module for AppModule {
    fn name(&self) -> &'static str {
        "AppModule"
    }

    fn register_controllers(&self, router: Router) -> Router {
        let controllers = vec![controller::HealthCheckController {}];

        register_routes(controllers, router)
        
    }
}
