use app_core::traits::Controller;
use macros::{controller, register_routes, RouteController, route};
// #[derive(RouteController)]
struct A {}

#[derive(RouteController)]
struct B {}

// // #[controller]
// fn oky() -> A {
//     A {}
// }

fn main() {
    let a = A {};
    // let b = B {};
    // b.name();
    // println!("{}", b.base_path());
    // b.register_route();
}
