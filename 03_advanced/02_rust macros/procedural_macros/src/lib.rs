extern crate proc_macro;
/*
This special syntax is needed to import proc_macro tools from the compiler.

There are three types of procedural macros.
1) function like (The simplest of the three)
2) attribute like (Need to be in separate crate to work)
3) custom derive
 */
use proc_macro::{TokenStream};
use chrono::Utc;
use quote::{quote};
use syn;

// *** functional macro ***
#[proc_macro] // for functional procedural macros
pub fn log_info(input: TokenStream) -> TokenStream {
    /*
    The returned TokenStream will replace the input TokenStream.
     */
    let mut output = "[Info] ".to_owned();

    for token in input {
        let token_string = token.to_string();

        match token_string.as_str() {
            "[TIME]" => {
                let time = Utc::now().time().to_string();
                output.push_str(&format!("{} ", time));
            }

            _ => {
                output.push_str(&format!("{} ", token_string));
            }
        }
    }

    TokenStream::from(quote! {
        println!("{}", #output); // # is needed to achieve Variable interpolation
    })
}
// *** end of functional macro ***

// *** derive macro ***
#[proc_macro_derive(Log)] // for derived macros, taking an argument which is the identifier that's gonna to be used in the derived attribute.
pub fn log_derive(input: TokenStream) -> TokenStream {
    /*
    Can only annotate structs and enums.
    The input token stream is going to be the item, the derive attribute is attached to.
    The returned TokenStream will be appended to the containing block or module of the annotated item.
     */
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let trait_impl = quote! {
        impl Log for #name {
            fn info(&self, msg: &str) {
                println!("[Info] {}: {}", stringify!(#name), msg);
            }
            fn warn(&self, msg: &str) {
                println!("[Warn] {}: {}", stringify!(#name), msg);
            }
            fn error(&self, msg: &str) {
                println!("[Error] {}: {}", stringify!(#name), msg);
            }
        }
    };

    trait_impl.into() // converts the token stream from quote! macro to the TokenStream aspect by the returned type
}
// ** end of derive macro ***