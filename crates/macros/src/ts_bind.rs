use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

use crate::config::Config;
use crate::{
  derive_attrs::DeriveAttrs, files::write_to_file, parsers::p_struct::parse_struct,
  ts::gen_struct::gen_struct,
};
use crate::parsers::p_enum::parse_enum;
use crate::ts::gen_enum::gen_enum;

pub fn handle_ts_bind(input: &DeriveInput) -> anyhow::Result<TokenStream> {
    let config = Config::load();
    let attr = DeriveAttrs::from(input.ident.to_string(), &input.attrs);

    let code = match &input.data {
        Data::Struct(data) => gen_struct(
          &attr,
          &config,
          &parse_struct(&data)?,
        )?,
        Data::Enum(data) => gen_enum(&attr, &config, parse_enum(&data)?)?,
        _ => return Ok(quote! {}.into()) //dont gen file
    };

    write_to_file(&config.create_path(&attr.get_export_path()), &code)?;
    Ok(quote! {}.into())
}
