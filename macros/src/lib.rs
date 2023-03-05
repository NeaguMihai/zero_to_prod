extern crate proc_macro;
use proc_macro::{TokenStream, Span};
use proc_macro2::Group;
use quote::quote;
use syn::{parse2, parse_macro_input, DeriveInput, ItemFn, LitStr};

#[proc_macro_attribute]
pub fn controller(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let _base_route = parse2::<LitStr>(_attr.into()).unwrap();
    let struct_data = parse2::<DeriveInput>(item.into()).unwrap();
    if let syn::Data::Struct(_) = struct_data.data {
        let name = struct_data.ident;
        let fields = match struct_data.data {
            syn::Data::Struct(data_struct) => data_struct.fields,
            _ => panic!("Expected struct"),
        };
        let text = Span::call_site().source();
        println!("text: {:?}", text);
        let field_names = fields.iter().map(|field| field.ident.as_ref().unwrap());
        let field_types = fields.iter().map(|field| &field.ty);

        let generated_code = quote! {


            #[derive(RouteController)]
            #[base_route("/test")]
            struct #name {
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
    attrs.iter().for_each(|field| {
        let tokens = field.tokens.clone();
        let parsed_stream = parse2::<Group>(tokens.clone()).unwrap();
        parsed_stream.stream().into_iter().for_each(|token| {
            println!("token: {:?}", token);
            if let proc_macro2::TokenTree::Ident(ident) = token {
                println!("ident: {:?}", ident);
            }
        });
        // println!("field: {:?}", parsed_stream);
    });

    let name = input.ident;
    let generated_code = quote! {

        impl Controller for #name {
            fn name(&self) -> &'static str {
                stringify!(#name)
            }
            fn base_path(&self) -> &'static str {
                "test"
            }
            fn register_routes(&self, router: Router) -> Router
            {
                println!("register_routes {}", file!());
                register_routes!(file!());
                router
            }
        }
    };
    TokenStream::from(generated_code)
}

#[proc_macro]
pub fn register_routes(item: TokenStream) -> TokenStream {
    let ts2: proc_macro2::TokenStream = item.into();
    let exp = quote! {
    // {

    // let input = parse2::<LitStr>(#item).unwrap();
    // println!("Welll? {}", input.value());
    // let source = std::fs::read_to_string(input.value()).unwrap();
    };
    // println!("input: {}", #input);

    // println!("source: {}", source);
    // let mut routes = vec![];

    // items.iter().for_each(|item| match item {
    //     syn::Item::Fn(item_fn) => {
    //         println!("{}", item_fn.sig.ident);
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

    // }
    // };

    exp.into()
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
