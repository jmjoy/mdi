#![warn(rust_2018_idioms)]
#![warn(clippy::dbg_macro, clippy::print_stdout)]

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, FnArg, Ident, ItemFn, Pat};

#[proc_macro]
pub fn call(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as Ident);
    let ident = Ident::new(
        &format!("mdi_inject__{}", ident.to_string()),
        Span::call_site(),
    );
    let result = quote! { #ident() };
    result.into()
}

#[proc_macro_attribute]
pub fn inject(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input0 = input.clone();
    let mut item_fn = parse_macro_input!(input0 as ItemFn);

    let sig = &mut item_fn.sig;
    let attrs = &item_fn.attrs;
    let vis = item_fn.vis;

    let ident = sig.ident.clone();
    sig.ident = Ident::new(
        &format!("mdi_inject__{}", sig.ident.to_string()),
        Span::call_site(),
    );

    let mut params = Vec::new();
    for input in &sig.inputs {
        match input {
            FnArg::Typed(ty) => match ty.pat.as_ref() {
                Pat::Ident(ident) => {
                    let ident = Ident::new(
                        &format!("mdi_resolve__{}", ident.ident.to_string()),
                        Span::call_site(),
                    );
                    params.push(quote! { crate::di::#ident() });
                }
                _ => {}
            },
            _ => {}
        }
    }

    sig.inputs = Punctuated::new();

    let result = quote! {
        #(#attrs)*
        #vis #sig {
            #ident(
                #(#params),*
            )
        }
    };

    let mut code = TokenStream::from(quote! { #[allow(dead_code)] });
    code.extend(input);
    code.extend::<TokenStream>(result.into());
    code
}

#[proc_macro_attribute]
pub fn resolve(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input0 = input.clone();
    let mut item_fn = parse_macro_input!(input0 as ItemFn);

    let sig = &mut item_fn.sig;
    let body = &item_fn.block;
    let attrs = &item_fn.attrs;
    let vis = item_fn.vis;

    sig.ident = Ident::new(
        &format!("mdi_resolve__{}", sig.ident.to_string()),
        Span::call_site(),
    );

    let result = quote! {
        #(#attrs)*
        #vis #sig {
            #body
        }
    };

    let mut code = TokenStream::from(quote! { #[allow(dead_code)] });
    code.extend(input);
    code.extend::<TokenStream>(result.into());
    code
}

#[proc_macro]
pub fn extern_di(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as Ident);
    let result = quote! { pub use #ident::di::*; };
    result.into()
}
