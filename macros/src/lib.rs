extern crate proc_macro;
extern crate regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use proc_macro::TokenStream;
use proc_macro2::{Group, Ident, Literal};
use quote::{quote, ToTokens};
use syn::{parse2, parse_macro_input, Attribute, AttributeArgs, DeriveInput, Item, ItemFn, LitStr};

#[proc_macro_attribute]
pub fn controller(_attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("controller");
    let _base_route = parse2::<LitStr>(_attr.into()).unwrap();
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
            .split(":")
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap()
            .to_string();

        let generated_code = quote! {


            #[derive(RouteController)]
            #[base_route("/test")]
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
    println!("Route Controller");
    let attrs = input.attrs.clone();

    let base_route = parse_controller_attributes(attrs.clone(), 0);
    let file_path = parse_controller_attributes(attrs, 1);

    println!("base_route: {:?}", base_route);
    println!("base_route: {:?}", file_path);

    let name = input.ident;
    let generated_code = quote! {

        impl Controller for #name {
            fn name(&self) -> &'static str {
                stringify!(#name)
            }
            fn base_path(&self) -> &'static str {
                "test"
            }
            fn register_routes(&self) -> () {
                println!("Registering routes for {}", self.name());

            }
        }

        impl #name {
            pub fn smth() -> () {
                CONTROLLER.register_routes();
            }
        }

        const CONTROLLER: #name = #name {
            router: None,
        };

    };
    TokenStream::from(generated_code)
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

// #[proc_macro]
// pub fn register_routes(item: TokenStream) -> TokenStream {
//     // let ident = parse2::<Ident>(item.into()).unwrap();

//     TokenStream::new()
// }asd

//TODO for future me: gather all controllers in the same folter
// After that, generate a file with all the routes. I think I should run a macro that activates at runtime
// This macro should walk all files in the controller directory and generate a file with all the routes
// To know the routes I should have the route macro generate struct const for each route
// The struct must look something like:
// struct Route {
//     path: &'static str,
//     method: &'static str,
//     handler: fn() -> (), //here I can use the ident of the function
// etcasfaasdasdafsasdasdasdasdssa 5
// }

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
