use app_core::traits::Controller;
use axum::Router;


pub fn register_routes(controller: Vec<impl Controller>, router: Router) -> Router {
  controller.into_iter().fold(router, |router, controller| {
      let base_path = controller.base_path();
      let routes = controller.register_routes();

      routes.into_iter().fold(router, |router, (path, route)| {
          let base_path = fold_route_paths(base_path.to_string());
          let route_path = fold_route_paths(path);
          router.route(format!("{}{}", base_path, route_path).as_str(), route)
      })
  })
}

fn fold_route_paths(path: String) -> String {
  path.to_string()
      .split("/")
      .fold(String::from("/"), |acc, token| {
          if token.is_empty() {
              acc
          } else {
              format!("{}/{}", acc, token)
          }
      })
}