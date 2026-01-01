use quote::{quote, ToTokens};
use std::iter;
use syn::{Expr, ExprLit, Lit, PathArguments, Type, TypeArray, TypePath};

pub fn ts_rs_map(token: &Type, imports: &mut Vec<String>) -> String {
    let mapped = match token {
        Type::Path(type_path) if is_option(type_path) => {
            let inner = unwrap_option(type_path);
            format!("{} | null", ts_rs_map(&inner, imports))
        }
        Type::Path(type_path) if type_path.path.is_ident("String") => "string".to_string(),
        Type::Reference(type_ref) => {
            let ty = type_ref.elem.as_ref();

            match ty {
                Type::Path(type_path) if type_path.path.is_ident("str") => "string".to_string(),
                _ => ts_rs_map(ty, imports),
            }
        }
        Type::Path(type_path)
            if [
                "i8", "i16", "i32", "i64", "i128", "u8", "u16", "u32", "u64", "u128", "f32", "f64",
                "usize", "isize",
            ]
            .contains(
                &type_path
                    .path
                    .get_ident()
                    .map(|i| i.to_string())
                    .unwrap_or_default()
                    .as_str(),
            ) =>
        {
            "number".to_string()
        }
        Type::Path(type_path) if type_path.path.is_ident("bool") => "boolean".to_string(),
        Type::Path(type_path) if is_vec(type_path) => {
            let inner = read_last_seg(type_path).expect("Could not get last segment");
            format!("({})[]", ts_rs_map(&inner, imports))
        }
        Type::Array(type_path) => format!("[{}]", unwrapped_fixed_array(type_path, imports)),
        Type::Path(type_path) if is_hashmap(type_path) => {
            let (key, value) = unwrap_hashmap(type_path);

            format!(
                "{{ [key: {}]: {} }}",
                ts_rs_map(&key, imports),
                ts_rs_map(&value, imports)
            )
        }
        _ => {
            let value = quote! {#token}.to_string();
            imports.push(value.clone());
            value
        }
    };

    mapped
}

fn is_option(type_path: &TypePath) -> bool {
    type_path
        .path
        .segments
        .last()
        .map(|seg| seg.ident == "Option")
        .unwrap_or(false)
}

fn unwrap_option(type_path: &TypePath) -> Type {
    read_last_seg(type_path).expect("Could not unwrap option")
}

fn is_vec(type_path: &TypePath) -> bool {
    type_path
        .path
        .segments
        .last()
        .map(|seg| seg.ident == "Vec")
        .unwrap_or(false)
}

fn read_last_seg(type_path: &TypePath) -> Option<Type> {
    let last_segment = type_path
        .path
        .segments
        .last()
        .expect("Could not get last segment");
    if let syn::PathArguments::AngleBracketed(args) = &last_segment.arguments {
        if let Some(syn::GenericArgument::Type(ty)) = args.args.first() {
            return Some(ty.clone());
        }
    }
    None
}

fn is_hashmap(type_path: &TypePath) -> bool {
    type_path
        .path
        .segments
        .last()
        .map(|seg| seg.ident == "HashMap")
        .unwrap_or(false)
}

fn unwrap_hashmap(type_path: &TypePath) -> (Type, Type) {
    let last_segment = type_path
        .path
        .segments
        .last()
        .expect("Could not get last segment");
    if let PathArguments::AngleBracketed(args) = &last_segment.arguments {
        if args.args.len() == 2 {
            if let (
                Some(syn::GenericArgument::Type(key)),
                Some(syn::GenericArgument::Type(value)),
            ) = (args.args.first(), args.args.get(1))
            {
                return (key.clone(), value.clone());
            }
        }
    }
    panic!("Could not unwrap HashMap");
}

fn unwrapped_fixed_array(type_array: &TypeArray, imports: &mut Vec<String>) -> String {
    iter::repeat_n(ts_rs_map(type_array.elem.as_ref(), imports), expr_to_usize(&type_array.len).unwrap())
        .collect::<Vec<String>>()
        .join(", ")
        .to_string()
}

fn expr_to_usize(expr: &Expr) -> syn::Result<usize> {
    if let Expr::Lit(ExprLit {
        lit: Lit::Int(lit_int),
        ..
    }) = expr
    {
        Ok(lit_int.base10_parse::<usize>()?)
    } else {
        Err(syn::Error::new_spanned(expr, "Expected integer literal"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::{parse_quote, Type};

    #[test]
    fn test_handle_option() {
        let ty: Type = parse_quote!(Option<String>);
        let imports = &mut Vec::new();
        let result = ts_rs_map(&ty, imports);
        assert_eq!(result, "string | null");

        let ty: Type = parse_quote!(Option<Option<String>>);
        let result = ts_rs_map(&ty, imports);
        assert_eq!(result, "string | null | null");
    }

    #[test]
    fn test_unwrap_option() {
        let ty: Type = parse_quote!(Option<String>);
        let type_path = match ty {
            Type::Path(type_path) => type_path,
            _ => panic!("Could not get type path"),
        };
        let result = unwrap_option(&type_path);
        if let Type::Path(type_path) = result {
            assert_eq!(type_path.path.get_ident().unwrap().to_string(), "String");
        } else {
            panic!("Could not get type path");
        }
    }

    #[test]
    fn test_is_option() {
        let ty: Type = parse_quote!(Option<String>);
        let type_path = match ty {
            Type::Path(type_path) => type_path,
            _ => panic!("Could not get type path"),
        };
        let result = is_option(&type_path);
        assert_eq!(result, true);

        let ty: Type = parse_quote!(String);
        let type_path = match ty {
            Type::Path(type_path) => type_path,
            _ => panic!("Could not get type path"),
        };
        let result = is_option(&type_path);
        assert_eq!(result, false);
    }

    #[test]
    fn test_vector_map() {
        let ty: Type = parse_quote!(Vec<String>);
        let imports = &mut Vec::new();
        let result = ts_rs_map(&ty, imports);
        assert_eq!(result, "(string)[]");
    }

    #[test]
    fn test_map_map() {
        let ty: Type = parse_quote!(std::collections::HashMap<String, String>);
        let imports = &mut Vec::new();
        let result = ts_rs_map(&ty, imports);
        assert_eq!(result, "{ [key: string]: string }");
    }

    #[test]
    fn test_str_ref() {
        {
            let ty: Type = parse_quote!(&str);
            let imports = &mut Vec::new();
            let result = ts_rs_map(&ty, imports);
            assert_eq!(result, "string");
        }

        {
            // with lifetime
            let ty: Type = parse_quote!(&'a str);
            let imports = &mut Vec::new();
            let result = ts_rs_map(&ty, imports);
            assert_eq!(result, "string");
        }
    }

    #[test]
    fn test_imports() {
        {
            let ty: Type = parse_quote!(Vec<Posts>);
            let imports = &mut Vec::new();
            ts_rs_map(&ty, imports);
            assert_eq!(imports, &mut vec!["Posts".to_string()]);
        }

        {
            let ty: Type = parse_quote!(Option<String>);
            let imports = &mut Vec::new();
            ts_rs_map(&ty, imports);

            assert_eq!(imports, &mut Vec::<String>::new());
        }

        {
            let ty: Type = parse_quote!(Users);
            let imports = &mut Vec::new();
            ts_rs_map(&ty, imports);
            assert_eq!(imports, &mut vec!["Users".to_string()]);
        }
    }
}
