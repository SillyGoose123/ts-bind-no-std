use crate::parsers::field_attributes::{FieldAttributes, parse_field_attributes};
use syn::{DataEnum, Expr, Ident};

pub type ParsedVariant = (Ident, Option<Expr>, FieldAttributes);

pub fn parse_enum(data: &DataEnum) -> anyhow::Result<Vec<ParsedVariant>> {
    let mut parsed_variants: Vec<ParsedVariant> = Vec::new();

    for variant in &data.variants {
        if !variant.fields.is_empty() {
          continue;
        }
        parsed_variants.push((
            variant.clone().ident,
            variant.clone().discriminant.map(|t| t.1),
            parse_field_attributes(&variant.attrs)?,
        ));
    }

    Ok(parsed_variants)
}
