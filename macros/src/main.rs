use axum::Router;
use macros::{controller, register_routes, RouteController};
use app_core::traits::Controller;
// #[derive(RouteController)]
// struct A {}

use syn::LitStr;

#[controller("asd")]
struct B {}

// // #[controller]
// fn oky() -> A {
//     A {}
// }

fn main() {
    // let a = A {};
    // a.;
    let b = B {};
    // let r = Router::new();
    // b.register_routes(r);
    // println!("{}", b.base_path());
    // b.register_route();
    // let a = "asdas";
    // let tli = LitStr::new(&a, proc_macro2::Span::call_site());
    // register_routes!("./src/lib.rs");
}
