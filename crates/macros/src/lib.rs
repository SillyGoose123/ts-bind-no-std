use crate::ts_bind_const::handle_ts_bind_const;
use error::ToCompileError;
use proc_macro::TokenStream;
use syn::{DeriveInput, ItemConst, Meta, parse_macro_input};
use ts_bind::handle_ts_bind;

mod config;
mod error;
mod files;
mod parsers;
mod rename_all;
mod derive_attrs;
mod ts;
mod ts_bind;
mod ts_bind_const;

#[proc_macro_derive(TsBind, attributes(ts_bind))]
pub fn ts_bind_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    handle_ts_bind(&input).unwrap_or_else(|e| e.to_compile_error())
}

#[proc_macro_attribute]
pub fn ts_bind_const(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_clone = item.clone();
    let const_item = parse_macro_input!(item_clone as ItemConst);
    let args: Result<Meta, syn::Error> = syn::parse(attr);

    handle_ts_bind_const(const_item, args)
        .map_err(|e| e.to_compile_error())
        .unwrap();
    item
}
