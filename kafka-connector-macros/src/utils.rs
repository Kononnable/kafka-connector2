use proc_macro::TokenStream;

pub struct JniMethodMetadata {
    pub class: String,
    pub method: String,
    pub parameters: Vec<String>,
    pub java_return_type: String,
    pub rust_return_type: String,
    pub value_getter: String,
}

pub fn parse_jni_method_comment(input: TokenStream) -> JniMethodMetadata {
    let input = input.to_string().replace('\n', "");
    let mut x = input
        .split('*')
        .filter(|x| !x.trim().is_empty())
        .map(|z| z.split(':').nth(1).unwrap().replace(" ", ""));

    let class = x.next().unwrap();
    let method = x.next().unwrap();
    let java_type = x.next().unwrap();

    let s: String = java_type.chars().skip(1).collect();
    let mut parts = s.split(')');

    let args = parts.next().unwrap();
    let parameters = vec![];
    if !args.is_empty() {
        todo!();
    }

    let java_return_type = parts.next().unwrap().to_string();
    let value_getter = java_return_type
        .chars()
        .next()
        .unwrap()
        .to_lowercase()
        .to_string();

    let rust_return_type = match java_return_type.as_str() {
        "J" => "jlong",
        "I" => "jint",
        "Ljava/lang/String" => "jstring",
        _ if java_return_type.starts_with('L') => "jobject",
        _ => todo!("Unknown return type"),
    }
    .to_owned();

    JniMethodMetadata {
        class,
        method,
        parameters,
        java_return_type,
        value_getter,
        rust_return_type,
    }
}
