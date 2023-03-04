use std::convert::Infallible;

use axum::{routing::MethodRouter, Router};

pub trait Module {
    fn name(&self) -> &'static str;
    fn register_controllers(&self, router: Router) -> Router;
}

pub trait Controller {
    fn name(&self) -> &'static str;
    fn base_path(&self) -> &'static str;
    fn register_routes<S, B>(&self) -> Vec<(String, MethodRouter<S, B, Infallible>)>
    where
        B: axum::body::HttpBody + Send + Sync + 'static,
        S: Clone + Send + Sync + 'static;
}
