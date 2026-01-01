use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum RenameAll {
    CamelCase,
    SnakeCase,
    UpperCase,
    LowerCase,
    PascalCase,
    KebabCase,
}

impl RenameAll {
    pub fn to_case(&self, s: &str) -> String {
        match self {
            Self::CamelCase => s.to_case(Case::Camel),
            Self::SnakeCase => s.to_case(Case::Snake),
            Self::UpperCase => s.to_case(Case::Upper),
            Self::LowerCase => s.to_case(Case::Lower),
            Self::PascalCase => s.to_case(Case::Pascal),
            Self::KebabCase => s.to_case(Case::Kebab).replace("-", "_"),
        }
    }

    pub fn from_string(value: &str) -> Option<Self> {
        match value {
            "camelCase" => Some(RenameAll::CamelCase),
            "snake_case" => Some(RenameAll::SnakeCase),
            "UPPERCASE" => Some(RenameAll::UpperCase),
            "lowercase" => Some(RenameAll::LowerCase),
            "PascalCase" => Some(RenameAll::PascalCase),
            "kebab_case" => Some(RenameAll::KebabCase),
            _ => {
                panic!("Invalid attribute name: .{}.", value);
            }
        }
    }
}
