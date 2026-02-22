use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};
use crate::config::Config;
use crate::{
  derive_attrs::DeriveAttrs, files::write_to_file,
  ts::gen_struct::gen_struct,
};
use crate::parsers::p_enum::parse_enum;
use crate::parsers::p_struct::parse_struct;
use crate::ts::gen_enum::gen_enum;

pub fn handle_ts_bind(input: &DeriveInput) -> anyhow::Result<TokenStream> {
    let config = Config::load();
    let attr = DeriveAttrs::from(input.ident.to_string(), &input.attrs);
    let mut is_enum = false;
    
    let code = match &input.data {
        Data::Struct(data) => gen_struct(&attr, config, &parse_struct(data)?)?,
        Data::Enum(data) => {
          is_enum = true;
          gen_enum(&attr, &config, parse_enum(&data)?)?
        },
        _ => return Ok(quote! {}.into())
    };

    write_to_file(&config.create_path(&attr.get_export_path(is_enum)), &code)?;
    Ok(quote! {}.into())
}
