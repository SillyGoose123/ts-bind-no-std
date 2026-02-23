use crate::config::Config;
use crate::derive_attrs::DeriveAttrs;
use crate::parsers::p_enum::ParsedVariant;
use crate::ts::FILE_DISCLAIMER;
use crate::ts::ts_map::ts_rs_map;
use crate::ts::utils::{apply_name_attr, gen_imports};
use quote::ToTokens;
use syn::Expr;

pub fn gen_enum(
    derive_attrs: &DeriveAttrs,
    config: &Config,
    data: Vec<ParsedVariant>,
) -> anyhow::Result<String> {
    let mut output = String::from(format!("\nexport const {} = {{\n", derive_attrs.get_name()));

    for (index, (ident, discriminant, attrs, _)) in data.iter().enumerate() {
        if attrs.skip {
            continue;
        }
        output.push_str(&format!(
            "  {}: {},\n",
            apply_name_attr(ident.to_string(), derive_attrs, config, attrs),
            map_discriminant(discriminant, index),
        ));
    }

    output.push_str("};\n");

    if derive_attrs
        .get_enum_type_export()
        .unwrap_or(&config.enum_type_export.unwrap_or(false))
        .clone()
    {
        let (imports, type_export) = generate_type_export(derive_attrs, data);
        if !imports.is_empty() {
            output = format!("{}{}", imports, output)
        }
        output.push_str(&type_export);
    }

    Ok(format!("{}\n{}", FILE_DISCLAIMER, output))
}

fn generate_type_export(derive_attrs: &DeriveAttrs, data: Vec<ParsedVariant>) -> (String, String) {
    if !data.iter().any(|(_, _, _, field)| field.is_some()) {
        return (
            String::new(),
            format!(
                "\nexport type {0} = typeof {0}[keyof typeof {0}];",
                derive_attrs.get_name()
            ),
        );
    }

    let mut imports = Vec::new();
    let mut output = format!("\nexport type {} = ", derive_attrs.get_name());

    for (index, (_, discriminant, _, field)) in data.iter().enumerate() {
        output.push_str(&format!(
            "\n| {{ type: {}, data: {} }}",
            map_discriminant(discriminant, index),
            field
                .clone()
                .map(|f| ts_rs_map(&f.ty, &mut imports))
                .unwrap_or(String::from("any")),
        ))
    }

    (gen_imports(imports, derive_attrs.get_name()), output)
}

fn map_discriminant(discriminant: &Option<Expr>, index: usize) -> String {
    discriminant
        .clone()
        .map(|f| f.to_token_stream().to_string())
        .unwrap_or(index.to_string())
}
