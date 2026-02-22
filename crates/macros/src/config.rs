use crate::rename_all::RenameAll;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

#[derive(Serialize, Deserialize)]
pub struct Config {
    ts_code_gen_path: Option<String>,
    pub default_case: Option<RenameAll>,
    pub enum_type_export: Option<bool>
}

static CONFIG: OnceLock<Config> = OnceLock::new();

impl Config {
    pub fn load() -> &'static Config {
        CONFIG.get_or_init(|| {
            let text = fs::read_to_string("ts-bind.toml").expect("failed to read ts-bind config");
            toml::from_str(&text).expect("invalid ts-bind config")
        })
    }

    pub fn create_path(&self, path_buf: &PathBuf) -> PathBuf {
        if self.ts_code_gen_path.is_none() {
            return path_buf.clone();
        }

        PathBuf::from(self.ts_code_gen_path.as_ref().unwrap()).join(path_buf)
    }
}
