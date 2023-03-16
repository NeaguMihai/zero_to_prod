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
use syn::{parse2, parse_macro_input, Attribute, DeriveInput, ItemFn, LitStr};

const METHODS: [&str; 5] = ["#[get", "#[post", "#[put", "#[delete", "#[patch"];

lazy_static! {
    static ref FUNCTION_REGEX: Regex = Regex::new(r"fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\(").unwrap();
    static ref METHOD_NAME_REGEX: Regex = Regex::new(r"#\[(.*)\(.*\)\]").unwrap();
    static ref ROUTE_REGEX: Regex = Regex::new(r#"#\[.*\("(.*)"\)\]"#).unwrap();
}

#[proc_macro_attribute]
pub fn controller(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("controller");
    let base_route = parse2::<LitStr>(attr.into()).unwrap();
    let struct_data = parse2::<DeriveInput>(item.into()).unwrap();
    if let syn::Data::Struct(_) = struct_data.data {
        let name = struct_data.ident;
        let fields = match struct_data.data {
            syn::Data::Struct(data_struct) => data_struct.fields,
            _ => panic!("Expected struct"),
        };

        let field_names = fields.iter().map(|field| field.ident.as_ref().unwrap());
        let field_types = fields.iter().map(|field| &field.ty);
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR").expect("Failed to get current dir");
        let target_dir = PathBuf::from(crate_dir);
        let target_dir = target_dir.parent().unwrap();
        let routes_file_path = target_dir.join("target/tmp/routes.txt");
        let reader =
            BufReader::new(File::open(routes_file_path).expect("Failed to open routes file"));
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

        let generated_code = quote! {


            #[derive(RouteController)]
            #[base_route(#base_route)]
            #[file_path(#path)]
            struct #name {
                pub router: Option<Router>,
                #( #field_names: #field_types ),*
            }
        };
        return TokenStream::from(generated_code);
    }

    panic!("Expected struct");
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
            let group = match parse2::<Group>(attr.tokens.clone()) {
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
