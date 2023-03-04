extern crate proc_macro;
use std::convert::Infallible;

use axum::routing::MethodRouter;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse2, parse_macro_input, DeriveInput, Expr, Ident, ItemFn, LitStr};

#[proc_macro_derive(Controller)]
pub fn controller_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generated_code = quote! {
        use axum::routing::MethodRouter;
        use std::convert::Infallible;
        use crate::traits::APIController;
        
        impl APIController for #name {
            fn name(&self) -> &'static str {
                stringify!(#name)
            }
            fn base_path(&self) -> &'static str {
                "test"
            }
            fn register_routes<S, B>(&self) -> Vec<(String, MethodRouter<S, B, Infallible>)>
            where
                B: axum::body::HttpBody + Send + Sync + 'static,
                S: Clone + Send + Sync + 'static,
            {
                vec![]
            }
        }
    };
    TokenStream::from(generated_code)
}

#[proc_macro]
pub fn find_routes(input: TokenStream) -> TokenStream {
    // let exp = quote! {
    // {
    let input = parse2::<LitStr>(input.into()).unwrap();
    let source = std::fs::read_to_string(input.value()).unwrap();
    let items = syn::parse_file(&source).unwrap().items;
    // println!("input: {}", #input);

    // println!("source: {}", source);
    let mut classes = vec![];

    items.iter().for_each(|item| match item {
        syn::Item::Struct(item_struct) => {
            classes.push(item_struct);
        }
        _ => {}
    });
    if classes.len() == 0 {
        // panic!("No classes found");
    }
    TokenStream::new()
    // }
    // };

    // exp.into()
}

#[proc_macro_attribute]
pub fn route(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let stream_copy = item.to_string().parse().unwrap();
    let input = parse2::<ItemFn>(stream_copy).unwrap();
    // Get the name and inputs of the function being wrapped
    let name = input.sig.ident;
    let inputs = input.sig.inputs;
    let return_type = input.sig.output;
    // Generate the code for the new function that wraps the original function
    let expanded = quote! {

            fn #name(#inputs) #return_type {
                #name(#inputs)
            }



    };

    TokenStream::from(expanded)
    // TokenStream::new()
}
