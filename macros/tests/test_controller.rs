// use macros::controller;

use std::sync::Arc;

use axum::{Router, handler::Handler};
use lazy_static::lazy_static;

// #[controller("/test")]
// struct TestController {}
static ROUTES: Arc<Router> = Arc::new(Router::new());

