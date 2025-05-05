extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn, NestedMeta, Meta};

#[proc_macro_attribute]
pub fn my_attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(input as ItemFn);

    let arg_str = match &args[0] {
        NestedMeta::Meta(Meta::NameValue(nv)) => {
            if let Some(ident) = nv.path.get_ident() {
                ident.to_string()
            } else {
                panic!("Expected named argument");
            }
        }
        _ => panic!("Expected named argument"),
    };

    let fn_name = &input.sig.ident;
    let expanded = quote! {
        println!("Attribute argument: {}", #arg_str);
        fn #fn_name() {
            println!("Hello from the decorated function!");
            #input
        }
    };

    TokenStream::from(expanded)
}
