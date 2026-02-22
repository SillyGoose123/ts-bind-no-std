use syn::{Attribute, LitStr};
use syn::meta::ParseNestedMeta;

#[derive(Default)]
pub(crate) struct FieldAttributes {
  pub rename: Option<String>,
  pub skip: bool,
}

impl FieldAttributes {
  pub fn new() -> Self {
    Self::default()
  }
}

pub(crate) fn parse_field_attributes(attrs: &[Attribute]) -> anyhow::Result<FieldAttributes> {
  let mut field_attrs = FieldAttributes::new();

  for attr in attrs.iter() {
    if attr.path().is_ident("ts_bind") {
      attr.parse_nested_meta(|meta| {
        let path = &meta.path;

        let ident = path.get_ident();
        if let Some(ident) = ident {
          let ident_str = ident.to_string();
          match ident_str.as_str() {
            "rename" => {
              field_attrs.rename = Some(
                get_nested_value(&meta).expect("Failed to parse rename attribute"),
              );
            }
            "skip" => {
              field_attrs.skip = true;
            },
            _ => {
              panic!("Invalid attribute name: {}", ident_str);
            }
          }
        }

        Ok(())
      })?;
    }
  }

  Ok(field_attrs)
}

pub fn get_nested_value(meta: &ParseNestedMeta) -> anyhow::Result<String> {
  let value = meta.value()?;
  let s: LitStr = value.parse()?;

  let value = s.value();

  Ok(value)
}