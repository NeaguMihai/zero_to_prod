use macros::{ route, Controller };
use macros::traits::APIController;
// #[derive(Controller)]
struct A {}

#[route]
fn oky() -> A {
    A {}
}

fn main() {
    
    let a = A {};
    
}
