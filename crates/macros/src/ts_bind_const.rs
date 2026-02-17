use crate::config::Config;
use crate::files::write_const_file;
use crate::rename_all::RenameAll;
use crate::ts::ts_map::ts_rs_map;
use quote::ToTokens;
use std::path::PathBuf;
use syn::{ItemConst, Meta};


pub fn handle_ts_bind_const(item: ItemConst, meta: Result<Meta, syn::Error>) -> anyhow::Result<()> {
    let config = Config::load();
    let mut imports = Vec::new();

    let const_name = item.ident.to_token_stream().to_string();

    let name = if meta.is_ok()
        && let Some(case) = read_case(meta?)
    {
        case.to_case(&const_name)
    } else if let Some(case) = &config.default_case {
        case.to_case(&const_name)
    } else {
        const_name
    };
    let content = format!(
        "export const {}: {} = {};",
        name,
        ts_rs_map(item.ty.as_ref(), &mut imports),
        item.expr.as_ref().to_token_stream().to_string()
    );
  
    write_const_file(
        &config.create_path(&PathBuf::new().join("bindings").join("const.ts")),
        imports,
        &content,
    )?;
    Ok(())
}

fn read_case(meta: Meta) -> Option<RenameAll> {
    if let Meta::NameValue(nv) = meta {
        if nv.path.is_ident("rename") {
            return Some(
                RenameAll::from_string(
                    nv.value
                        .to_token_stream()
                        .to_string()
                        .replace("\"", "")
                        .as_str(),
                )
                .unwrap(),
            );
        }
    }

    None
}
