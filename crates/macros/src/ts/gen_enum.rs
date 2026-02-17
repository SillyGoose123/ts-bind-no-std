use crate::config::Config;
use crate::derive_attrs::DeriveAttrs;
use crate::parsers::p_enum::ParsedVariant;
use crate::ts::utils::apply_name_attr;
use quote::ToTokens;

pub fn gen_enum(
    derive_attrs: &DeriveAttrs,
    config: &Config,
    data: Vec<ParsedVariant>,
) -> anyhow::Result<String> {
    let mut output = String::from(format!("\nexport enum {} {{\n", derive_attrs.get_name()));

    for (ident, discriminant, attrs) in &data {
        if attrs.skip {
            continue;
        }
        output.push_str(&format!(
            "  {}{},\n",
            apply_name_attr(ident.to_string(), derive_attrs, config, attrs),
            discriminant.clone()
                .map(|f| format!(" = {}", f.to_token_stream().to_string()))
                .unwrap_or(String::new()),
        ));
    }

    output.push_str("}\n");

    Ok(output)
}
