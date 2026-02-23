use syn::{Attribute, LitBool};

use crate::parsers::field_attributes::get_nested_value;
use crate::rename_all::RenameAll;
use std::path::PathBuf;

#[derive(Debug)]
pub struct DeriveAttrs {
    name: String,
    rename_all: Option<RenameAll>,
    export: Option<PathBuf>,
    enum_type_export: Option<bool>,
}

impl DeriveAttrs {
    pub fn from(struct_name: String, attrs: &Vec<Attribute>) -> Self {
        let mut struct_attrs = Self {
            name: struct_name,
            rename_all: None,
            export: None,
            enum_type_export: None,
        };

        Self::parse_attrs(&mut struct_attrs, attrs);

        struct_attrs
    }

    fn parse_attrs(struct_attrs: &mut Self, attrs: &Vec<Attribute>) {
        attrs.iter().for_each(|attr| {
            if attr.path().is_ident("ts_bind") {
                attr.parse_nested_meta(|meta| {
                    let path = &meta.path;

                    let ident = path.get_ident();

                    if let Some(ident) = ident {
                        let ident_str = ident.to_string();

                        match ident_str.as_str() {
                            "rename" => {
                                let value = get_nested_value(&meta)
                                    .expect("Failed to parse rename attribute");

                                struct_attrs.name = value;
                            }
                            "rename_all" => {
                                let value = get_nested_value(&meta)
                                    .expect("Failed to parse rename attribute");

                                struct_attrs.rename_all = RenameAll::from_string(&value)
                            }
                            "export" => {
                                let value = get_nested_value(&meta)
                                    .expect("Failed to parse export attribute");

                                struct_attrs.export = Some(PathBuf::from(value));
                            }
                            "enum_type_export" => {
                              let value = meta.value()?;
                              let s: LitBool = value.parse()?;
                              struct_attrs.enum_type_export = Some(s.value);
                            }
                            _ => {
                                panic!("Invalid attribute name: {}", ident_str);
                            }
                        }
                    }

                    Ok(())
                })
                .expect("Failed to parse nested meta");
            }
        });
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_export_path(&self, is_enum: bool) -> PathBuf {
        self.export
            .clone()
            .unwrap_or_else(|| PathBuf::new().join("bindings"))
            .join(format!(
                "{}{}.ts",
                self.get_name(),
                if is_enum { "" } else { ".d" }
            ))
    }

    pub fn get_rename_all(&self) -> Option<&RenameAll> {
        self.rename_all.as_ref()
    }

    pub fn get_enum_type_export(&self) -> Option<&bool> {
        self.enum_type_export.as_ref()
    }
}
