use crate::parsers::field_attributes::{FieldAttributes, parse_field_attributes};
use syn::{DataEnum, Expr, Field, Fields, Ident};

pub type ParsedVariant = (Ident, Option<Expr>, FieldAttributes, Option<Field>);

pub fn parse_enum(data: &DataEnum) -> anyhow::Result<Vec<ParsedVariant>> {
    let mut parsed_variants: Vec<ParsedVariant> = Vec::new();

    for variant in &data.variants {
        let opt_type = if let Fields::Unnamed(fields_unnamed) = &variant.fields
            && let Some(field) = fields_unnamed.unnamed.first()
        {
            Some(field.clone())
        } else {
            None
        };

        parsed_variants.push((
            variant.clone().ident,
            variant.clone().discriminant.map(|t| t.1),
            parse_field_attributes(&variant.attrs)?,
            opt_type,
        ));
    }

    Ok(parsed_variants)
}
