use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;

use crate::utils::{parse_jni_like_definition, parse_jni_method_comment, JniMethodMetadata};
pub fn java_property_getter_impl(input: TokenStream) -> TokenStream {
    let macro_args = parse_jni_like_definition(input);
    let JniMethodMetadata {
        class_underscore_notation: class,
        method,
        value_getter,
        java_return_type,
        rust_return_type,
        ..
    } = parse_jni_method_comment(&macro_args);

    let function_name = Ident::new(&format!("Java_{}_{}", class, method), Span::call_site());
    let rust_return_type = Ident::new(&rust_return_type, Span::call_site());

    let getter = Ident::new(&value_getter, Span::call_site());
    let getter = match value_getter.as_str() {
        "l" => quote! {res.#getter().expect("failed data extraction from field").into_inner()},
        _ => quote! {res.#getter().expect("failed data extraction from field")},
    };

    let expanded = quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "system" fn #function_name (
            env: jni::JNIEnv,
            obj: jni::objects::JObject,
        ) -> jni::sys::#rust_return_type {
            let res = env.get_field(obj, #method, #java_return_type).expect(&format!("could not get field {} of type {}",#method,#java_return_type));

            #getter
        }
    };
    TokenStream::from(expanded)
}
