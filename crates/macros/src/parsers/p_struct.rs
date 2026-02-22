use crate::parsers::field_attributes::{FieldAttributes, parse_field_attributes};
use syn::{DataStruct, Fields, Ident, Type};

pub type ParsedField = (Ident, Type, FieldAttributes);

pub fn parse_struct(data: &DataStruct) -> anyhow::Result<Vec<ParsedField>> {
    let mut fields_info = Vec::new();

    if let Fields::Named(ref fields_named) = data.fields {
        for field in fields_named.named.iter() {
            if let Some(ident) = &field.ident {
                fields_info.push((
                    ident.clone(),
                    field.ty.clone(),
                    parse_field_attributes(&field.attrs)?,
                ));
            }
        }
    }
    Ok(fields_info)
}