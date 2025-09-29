extern crate proc_macro;
/*
This special syntax is needed to import proc_macro tools from the compiler.

There are three types of procedural macros.
1) function like (The simplest of the three)
2) attribute like (Need to be in separate crate to work)
3) custom derive
 */

use proc_macro::{TokenStream};
use quote::{ToTokens};
use darling::{FromMeta, ast::NestedMeta};
use syn::{parse_macro_input, ItemFn, parse_quote, Ident, FnArg, Pat, Stmt, Meta};

// *** attribute like macro ***
#[derive(FromMeta)]
struct MacroArgs {
    #[darling(default)]
    verbose: bool,
}
#[proc_macro_attribute] // for attribute like macros
pub fn log_call(args: TokenStream, input: TokenStream) -> TokenStream {
    /*
    Attribute macros take two TokenStreams--input and args--and return a TokenStream. They can annotate functions,
    structs, and enums. Also, the returned TokenStream will replace the annotated item.
    args are the arguments passed into the attribute.
    input is the item we are annotating.
     */
    let mut input = parse_macro_input!(input as ItemFn);
    let attr_args = if args.is_empty() {
        MacroArgs { verbose: false }
    } else {

        let meta = parse_macro_input!(args as Meta);
        match MacroArgs::from_meta(&meta) {
            Ok(a) => a,
            Err(e) => return e.write_errors().into(),
        }
    };

    impl_log_call(&attr_args, &mut input)
}

fn impl_log_call(attr_args: &MacroArgs, input: &mut ItemFn) -> TokenStream {
    let fn_name = &input.sig.ident;

    if attr_args.verbose {
        let fn_args = extract_args_names(input);
        let statements = generate_verbose_log(fn_name, fn_args);
        input.block.stmts.splice(0..0, statements);
    } else {
        input.block.stmts.insert(0, parse_quote! {
            println!("[Info] calling {}", stringify!(#fn_name));
        });
    }

    input.to_token_stream().into()
}

fn extract_args_names(func: &ItemFn) -> Vec<&Ident> {
    func.sig.inputs.iter().filter_map(|arg| {
        if let FnArg::Typed(pat_type) = arg {
            if let Pat::Ident(pat) = &(*pat_type.pat) {
                return Some(&pat.ident);
            }
        }
        None
    }).collect()
}

fn generate_verbose_log(fn_name: &Ident, fn_args: Vec<&Ident>) -> Vec<Stmt> {
    let mut statements = vec![parse_quote! {
       print!("[Info] calling {} | ", stringify!(#fn_name));
    }];

    for arg in fn_args {
        statements.push(parse_quote! {
            print!("{} = {:?} ", stringify!(#arg), arg);
        })
    }

    statements.push(parse_quote! {println!(); });

    statements
}

// *** end of attribute like macro ***