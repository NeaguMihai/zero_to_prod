extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse2, DeriveInput, ItemFn, LitStr, parse_macro_input, Data};

#[proc_macro_attribute]
pub fn controller(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let base_route = parse2::<LitStr>(_attr.into()).unwrap();
    let struct_data = parse2::<DeriveInput>(item.into()).unwrap();

    if let syn::Data::Struct(_) = struct_data.data {
        let name = struct_data.ident;
        let fields = match struct_data.data {
            syn::Data::Struct(data_struct) => data_struct.fields,
            _ => panic!("Expected struct"),
        };
        let field_names = fields.iter().map(|field| field.ident.as_ref().unwrap());
        let field_types = fields.iter().map(|field| &field.ty);

        let generated_code = quote! {

            #[derive(RouteController)]
            struct #name {
                #( #field_names: #field_types ),*
            }
        };
        return TokenStream::from(generated_code);
    }

    panic!("Expected struct");
}

#[proc_macro_derive(RouteController)]
pub fn controller_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let fields = match input.data {
        Data::Struct(ref data_struct) => &data_struct.fields,
        _ => panic!("This derive macro only works with structs."),
    };
    
    let name = input.ident;
    let generated_code = quote! {
        use axum::routing::MethodRouter;
        use std::convert::Infallible;

        impl Controller for #name {
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


#[proc_macro_attribute]
pub fn register_routes(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // let exp = quote! {
    // {
    let input = parse2::<LitStr>(item.into()).unwrap();
    let source = std::fs::read_to_string(input.value()).unwrap();
    let items = syn::parse_file(&source).unwrap().items;
    // println!("input: {}", #input);

    // println!("source: {}", source);
    // let mut routes = vec![];

    // items.iter().for_each(|item| match item {
    //     syn::Item::Fn(item_fn) => {
    //         routes.push(item_fn);
    //     }
    //     _ => {}
    // });
    // let mut tokens = proc_macro2::TokenStream::new();
    // routes.iter().for_each(|route| {
    //     let name = route.sig.ident.clone();
    //     let expanded = quote! {#name};
    //     tokens.extend(expanded);
    // });
    // println!("tokens: {}", tokens);
    quote! {

        // impl Controller for #name {
        //     fn name(&self) -> &'static str {
        //         stringify!(#name)
        //     }
        //     fn base_path(&self) -> &'static str {
        //         #base_route
        //     }
        //     fn register_routes<S, B>(&self) -> Vec<(String, MethodRouter<S, B, Infallible>)>
        //     where
        //         B: axum::body::HttpBody + Send + Sync + 'static,
        //         S: Clone + Send + Sync + 'static,
        //     {
        //         vec![]
        //     }
        // }
    }
    .into()
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