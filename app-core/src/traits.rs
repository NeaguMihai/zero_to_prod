use axum::Router;

pub trait Module {
    fn name(&self) -> &'static str;
    fn register_controllers(&self, router: Router) -> Router;
}

pub trait Controller {
    fn name(&self) -> &'static str;
    fn base_path(&self) -> &'static str;
    fn register_routes(&self, router: Router) -> ();
}
