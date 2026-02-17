use super::ts_map::ts_rs_map;
use crate::config::Config;
use crate::ts::FILE_DISCLAIMER;
use crate::ts::utils::{apply_name_attr, gen_imports};
use crate::{derive_attrs::DeriveAttrs, parsers::p_struct::ParsedField};

pub fn gen_struct(
    struct_attrs: &DeriveAttrs,
    config: &Config,
    fields: &Vec<ParsedField>,
) -> anyhow::Result<String> {
    let mut ts_bind = String::from(format!(
        "\nexport interface {} {{\n",
        struct_attrs.get_name()
    ));
    let mut imports = Vec::new();
    for (ident, ty, attrs) in fields.iter() {
        if attrs.skip {
            continue;
        }
        let map_result = ts_rs_map(ty, &mut imports);
        ts_bind.push_str(&format!(
            "   {}: {};\n",
            apply_name_attr(ident.to_string(), struct_attrs, config, attrs),
            map_result
        ));
    }

    ts_bind.push_str("}");

    ts_bind = format!(
        "{}\n{}",
        gen_imports(imports, struct_attrs.get_name()),
        ts_bind
    );

    if ts_bind.contains("import") {
        ts_bind = format!("\n{}", ts_bind)
    }

    ts_bind = format!("{}\n{}", FILE_DISCLAIMER, ts_bind);

    Ok(ts_bind)
}
