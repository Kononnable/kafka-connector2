use std::collections::HashMap;

use proc_macro::{TokenStream, TokenTree};

pub struct JniMethodMetadata {
    pub class_underscore_notation: String,
    pub class_java_notation: String,
    pub method: String,
    pub parameters: Vec<String>,
    pub java_return_type: String,
    pub rust_return_type: String,
    pub value_getter: String,
}

pub fn parse_jni_method_comment(macro_args: &HashMap<String, String>) -> JniMethodMetadata {
    let class_underscore_notation = macro_args.get("Class").expect("No Class value").to_owned();
    let class_java_notation = class_underscore_notation.replace('_', "\\");
    let method = macro_args
        .get("Method")
        .expect("No Method value")
        .to_owned();
    let signature = macro_args.get("Signature").expect("No Signature value");

    let (parameters, java_return_type) = parse_signature(signature);

    let value_getter = java_return_type
        .chars()
        .next()
        .map(|ch| if ch == '[' { 'l' } else { ch })
        .unwrap()
        .to_lowercase()
        .to_string();

    let rust_return_type = match java_return_type.as_str() {
        "J" => "jlong",
        "I" => "jint",
        "Z" => "jboolean",
        "D" => "jdouble",
        "[]B" => "jbyteArray",
        "Ljava/lang/String" => "jstring",
        _ if java_return_type.starts_with('L') => "jobject",
        _ => todo!("Unknown return type"),
    }
    .to_owned();

    JniMethodMetadata {
        class_underscore_notation,
        class_java_notation,
        method,
        parameters,
        java_return_type,
        value_getter,
        rust_return_type,
    }
}

fn parse_signature(signature: &str) -> (Vec<String>, String) {
    let str: String = signature.chars().skip(1).collect();
    let (args, return_type) = str.split_once(')').unwrap();
    let parameters = vec![];
    if !args.is_empty() {
        todo!();
    }
    let java_return_type = return_type.to_owned();
    (parameters, java_return_type)
}

pub fn parse_jni_like_definition(input: TokenStream) -> HashMap<String, String> {
    input
        .into_iter()
        .fold(vec![], |mut acc, tt| {
            match tt {
                TokenTree::Punct(punct) if punct.as_char() == '*' => {
                    acc.push(vec![]);
                }
                _ => {
                    acc.last_mut().unwrap().push(tt);
                }
            }
            acc
        })
        .into_iter()
        .map(|x| {
            x.into_iter()
                .map(|tt| tt.to_string())
                .collect::<Vec<_>>()
                .join("")
                .split_once(':')
                .map(|r| (r.0.to_owned(), r.1.to_owned()))
                .unwrap()
        })
        .collect::<HashMap<_, _>>()
}
