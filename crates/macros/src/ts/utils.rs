use crate::config::Config;
use crate::derive_attrs::DeriveAttrs;
use crate::parsers::field_attributes::FieldAttributes;

fn sorter(imports: Vec<String>) -> Vec<String> {
  let mut temp = imports.clone();
  temp.sort();
  temp.dedup();
  temp
}

pub fn gen_imports(imports: Vec<String>, current: &String) -> String {
  let mut output = String::new();

  for to_import in sorter(imports) {
    // Do not import current interface
    if &to_import == current {
      continue;
    }

    output.push_str(&format!(
      "import type {{ {} }} from \"./{}\";\n",
      to_import, to_import
    ))
  }

  output
}

pub fn apply_name_attr(name: String, derive_attrs: &DeriveAttrs, config: &Config, attrs: &FieldAttributes) -> String {
  let field_name = if let Some(rename_all) = derive_attrs.get_rename_all() {
    rename_all.to_case(&name)
  } else if let Some(rename_all) = &config.default_case {
    rename_all.to_case(&name)
  } else {
    name
  };

  attrs.rename.as_ref().unwrap_or(&field_name).to_string()
}