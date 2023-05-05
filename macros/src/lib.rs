extern crate proc_macro;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    time::Instant,
};

use lazy_static::lazy_static;
use proc_macro::TokenStream;
use proc_macro2::{Group, Ident};
use quote::quote;
use regex::Regex;
use syn::{parse2, parse_macro_input, Attribute, DeriveInput, ItemFn, LitStr, Type};

const METHODS: [&str; 5] = ["#[get", "#[post", "#[put", "#[delete", "#[patch"];

lazy_static! {
    static ref FUNCTION_REGEX: Regex = Regex::new(r"fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\(").unwrap();
    static ref METHOD_NAME_REGEX: Regex = Regex::new(r"#\[(.*)\(.*\)\]").unwrap();
    static ref ROUTE_REGEX: Regex = Regex::new(r#"#\[.*\("(.*)"\)\]"#).unwrap();
}

fn find_controller_path(name: String, crate_dir: String) -> String {
    let target_dir = PathBuf::from(crate_dir);
    let target_dir = target_dir.parent().unwrap();
    let routes_file_path = target_dir.join("target/tmp/routes.txt");
    let reader = BufReader::new(File::open(routes_file_path).expect("Failed to open routes file"));
    let path = reader.lines().find(|line| {
        let line = line.as_ref().unwrap();
        if line.starts_with(&format!("{}:", name)) {
            return true;
        }
        false
    });
    let path = path
        .unwrap()
        .unwrap()
        .split(':')
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .to_string();
    path
}

fn destructure_struct_data(struct_data: DeriveInput) -> (Ident, Vec<Ident>, Vec<Type>) {
    let name = struct_data.ident;
    let fields = match struct_data.data {
        syn::Data::Struct(data_struct) => data_struct.fields,
        _ => panic!("Expected struct"),
    };

    let (field_names, field_types): (Vec<_>, Vec<_>) = fields
        .iter()
        .map(move |field| (field.ident.clone().unwrap(), field.ty.clone()))
        .unzip();
    (name, field_names, field_types)
}

#[proc_macro_derive(RouteController, attributes(base_route, file_path))]
pub fn controller_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let attrs = input.attrs.clone();

    let base_route = parse_controller_attributes(attrs.clone(), 0);
    let file_path = parse_controller_attributes(attrs, 1);

    let file = File::open(file_path).expect("Failed to open routes file");
    let start_time = Instant::now();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let routes = find_routes(lines);
    let end = Instant::now();
    println!("Time to read file: {:?}", end - start_time);

    let routes_tokens = convert_routes_to_token_stream(routes);

    let name = input.ident;
    let generated_code = quote! {
        use app_core::traits::Controller;
        impl Controller for #name {
            fn name(&self) -> &'static str {
                stringify!(#name)
            }
            fn base_path(&self) -> &'static str {
                #base_route
            }
            fn register_routes(&self, router: Router) -> Router {
                router
                #( #routes_tokens )*

            }
        }

    };
    TokenStream::from(generated_code)
}

fn convert_routes_to_token_stream(
    routes: Vec<(String, String, String)>,
) -> Vec<proc_macro2::TokenStream> {
    let axum_ident = Ident::new("axum", proc_macro2::Span::call_site());
    let route_ident = Ident::new("routing", proc_macro2::Span::call_site());
    let routes_tokens = routes
        .iter()
        .map(|(method_name, route, function_name)| {
            let method_name = Ident::new((method_name).as_str(), proc_macro2::Span::call_site());
            let route = LitStr::new(route, proc_macro2::Span::call_site());
            let function_name = Ident::new(function_name, proc_macro2::Span::call_site());
            quote! {
                .route(#route, #axum_ident::#route_ident::#method_name(#function_name))
            }
        })
        .collect::<Vec<_>>();
    routes_tokens
}

fn find_routes(mut lines: std::io::Lines<BufReader<File>>) -> Vec<(String, String, String)> {
    let mut routes: Vec<(String, String, String)> = Vec::new();
    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }
        let line = line.unwrap().unwrap();
        if METHODS.iter().any(|method| line.starts_with(method)) {
            let function_name = lines.next().unwrap().unwrap();

            let function_name = FUNCTION_REGEX
                .captures(&function_name)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();
            let method_name = METHOD_NAME_REGEX
                .captures(&line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();
            let route = ROUTE_REGEX
                .captures(&line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();

            routes.push((
                method_name.to_string(),
                route.to_string(),
                function_name.to_string(),
            ));
        }
    }
    routes
}

fn parse_controller_attributes(attrs: Vec<Attribute>, index: u16) -> String {
    let base_route: String = match attrs.get(index as usize) {
        Some(attr) => {
            let group: Group = match attr.parse_args() {
                Ok(group) => group,
                Err(err) => panic!("Error parsing base route or controller path: {:?}", err),
            };
            match parse2::<LitStr>(group.stream()) {
                Ok(lit_str) => lit_str.value(),
                Err(err) => panic!("Error parsing base route or controller path: {:?}", err),
            }
        }
        None => panic!("No base route or controller path provided"),
    };
    base_route
}

fn route(_attr: TokenStream, item: TokenStream, _method_type: &str) -> TokenStream {
    let stream_copy = item.to_string().parse().unwrap();

    let input = parse2::<ItemFn>(stream_copy).unwrap();

    let expanded = quote! {
        #input
    };
    expanded.into()
}

#[proc_macro_attribute]
pub fn get(_attr: TokenStream, item: TokenStream) -> TokenStream {
    route(_attr, item, "get")
}
#[proc_macro_attribute]
pub fn post(_attr: TokenStream, item: TokenStream) -> TokenStream {
    route(_attr, item, "post")
}
#[proc_macro_attribute]
pub fn put(_attr: TokenStream, item: TokenStream) -> TokenStream {
    route(_attr, item, "put")
}
#[proc_macro_attribute]
pub fn delete(_attr: TokenStream, item: TokenStream) -> TokenStream {
    route(_attr, item, "delete")
}
#[proc_macro_attribute]
pub fn patch(_attr: TokenStream, item: TokenStream) -> TokenStream {
    route(_attr, item, "patch")
}
#[proc_macro_attribute]
pub fn options(_attr: TokenStream, item: TokenStream) -> TokenStream {
    route(_attr, item, "options")
}

#[proc_macro_attribute]
pub fn controller(attr: TokenStream, item: TokenStream) -> TokenStream {
    let base_route = parse2::<LitStr>(attr.into()).unwrap();
    let struct_data = parse2::<DeriveInput>(item.clone().into()).unwrap();
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").expect("Failed to get current dir");
    let (name, field_names, field_types) = destructure_struct_data(struct_data);
    let path = find_controller_path(name.to_string(), crate_dir);

    let generated_code = quote! {

        #[derive(RouteController)]
        #[base_route(#base_route)]
        #[file_path(#path)]
        struct #name {
            pub router: Option<Router>,
            #( #field_names: #field_types ),*
        }
    };
    TokenStream::from(generated_code)
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write, panic};

    use quote::quote;
    use syn::{parse2, DeriveInput};

    use crate::find_controller_path;

    #[test]
    fn test_route_regex_works() {
        let input = r#"
            #[get("/test")]
            async fn test() {}
        "#;

        let binding = input.to_string();
        println!("binding: {}", binding);
        let function_name = super::ROUTE_REGEX
            .captures(&binding)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();

        assert_eq!(function_name, "/test");

        let binding = input.to_string();
        println!("binding: {}", binding);
        let function_name = super::METHOD_NAME_REGEX
            .captures(&binding)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();

        assert_eq!(function_name, "get");

        let binding = input.to_string();
        println!("binding: {}", binding);
        let function_name = super::FUNCTION_REGEX
            .captures(&binding)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();

        assert_eq!(function_name, "test");
    }

    #[test]
    fn test_destructure_struct_data_works_without_attributes() {
        let input = quote! {
            struct TestController {}
        };

        let struct_data = parse2::<DeriveInput>(input).unwrap();

        let (ident, field_names, field_types) = super::destructure_struct_data(struct_data);

        assert_eq!(ident, "TestController");
        assert!(field_names.is_empty());
        assert!(field_types.is_empty());
    }

    #[test]
    fn test_destructure_struct_data_works_witho_attributes() {
        let input = quote! {
            struct TestController {
                attr1: String,
                attr2: i32,
            }
        };

        let struct_data = parse2::<DeriveInput>(input).unwrap();

        let (ident, field_names, field_types) = super::destructure_struct_data(struct_data);

        assert_eq!(ident, "TestController");
        assert!(field_names.get(0).is_some());
        assert_eq!(field_names.get(0).unwrap().to_string(), "attr1");
        assert!(field_names.get(1).is_some());
        assert_eq!(field_names.get(1).unwrap().to_string(), "attr2");
        assert!(field_types.get(0).is_some());
        let field_path = match field_types.get(0).unwrap() {
            syn::Type::Path(path) => path,
            _ => panic!("Expected a path type"),
        };
        assert!(field_path.path.segments.first().is_some());
        assert_eq!(
            field_path.path.segments.first().unwrap().ident.to_string(),
            "String"
        );
        assert!(field_types.get(0).is_some());
        let field_path = match field_types.get(1).unwrap() {
            syn::Type::Path(path) => path,
            _ => panic!("Expected a path type"),
        };
        assert!(field_path.path.segments.first().is_some());
        assert_eq!(
            field_path.path.segments.first().unwrap().ident.to_string(),
            "i32"
        );
    }

    #[test]
    fn test_destructure_struct_should_panic() {
        let input = quote! {
            enum TestController {}
        };

        let struct_data = parse2::<DeriveInput>(input).unwrap();

        let result = panic::catch_unwind(|| super::destructure_struct_data(struct_data));

        assert!(result.is_err());
        match result {
            Ok(_) => {
                panic!("Expected a panic");
            }
            Err(err) => {
                assert_eq!(
                    err.downcast_ref::<&str>().unwrap().to_string(),
                    "Expected struct"
                );
            }
        };
    }

    #[test]
    fn test_controller_input_works() {
        let current_dir = std::env::current_dir().unwrap().join("tests");
        File::create(current_dir.join("target/tmp/routes.txt"))
            .unwrap()
            .write_all(
                ("TestController:".to_string()
                    + current_dir.join("test_controller.rs").to_str().unwrap())
                .as_bytes(),
            )
            .unwrap();
        let path = find_controller_path(
            "TestController".to_string(),
            current_dir.join("target").to_str().unwrap().to_string(),
        );
        assert_eq!(
            path,
            current_dir
                .join("test_controller.rs")
                .to_str()
                .unwrap()
                .to_string()
        );
    }
}
