pub mod traits;

pub struct Route {
    pub method: String,
    pub path: String,
    pub handler: String,
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
    }
}
