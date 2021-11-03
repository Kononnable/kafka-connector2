use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{Expr, Type};

use crate::utils::{parse_jni_like_definition, parse_jni_method_comment, JniMethodMetadata};
pub fn rust_property_getter_impl(input: TokenStream) -> TokenStream {
    let macro_args = parse_jni_like_definition(input);
    let JniMethodMetadata {
        class_underscore_notation: class,
        method,
        rust_return_type,
        value_getter,
        ..
    } = parse_jni_method_comment(&macro_args);
    let struct_name = macro_args
        .get("Struct")
        .expect("No Struct value")
        .to_owned();

    let default: Expr = if ["jobject", "jbyteArray"].contains(&rust_return_type.as_str()) {
        syn::parse_str("jni::objects::JObject::null().into_inner()").unwrap()
    } else {
        syn::parse_str("Default::default()").unwrap()
    };

    let struct_name: Type = syn::parse_str(&struct_name).unwrap();
    let function_name = Ident::new(&format!("Java_{}_{}", class, method), Span::call_site());
    let rust_field_name = Ident::new(&method.to_case(Case::Snake), Span::call_site());
    let rust_return_type = Ident::new(&rust_return_type, Span::call_site());

    let getter = Ident::new(&value_getter, Span::call_site());
    let getter = match value_getter.as_str() {
        "l" => quote! {ret?.#getter().expect("failed data extraction from field").into_inner()},
        _ => quote! {ret?.#getter().expect("failed data extraction from field")},
    };

    let expanded = quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "system" fn #function_name (
            env: jni::JNIEnv,
            obj: jni::objects::JObject,
        ) -> jni::sys::#rust_return_type {

            let result = || -> jni::errors::Result<_> {
                let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
                let rust_struct= unsafe { Box::from_raw(ptr as *mut #struct_name) };
                let ret = crate::clone_to_from_java::CloneToFromJava::clone_to_java(&rust_struct.#rust_field_name, env);
                let _ptr = Box::into_raw(rust_struct);
                let ret = #getter;

                Ok(ret)
            };

            match result() {
                Ok(v) => v,
                Err(jni::errors::Error::JavaException) => #default,
                _ => panic!(),
            }
        }
    };
    TokenStream::from(expanded)
}
