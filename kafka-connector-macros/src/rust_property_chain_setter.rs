use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::Type;

use crate::utils::{parse_jni_like_definition, parse_jni_method_comment, JniMethodMetadata};
pub fn rust_property_chain_setter_impl(input: TokenStream) -> TokenStream {
    let macro_args = parse_jni_like_definition(input);
    let JniMethodMetadata {
        class_underscore_notation: class,
        method,
        rust_argument_types,
        ..
    } = parse_jni_method_comment(&macro_args);
    let struct_name = macro_args
        .get("Struct")
        .expect("No Struct value")
        .to_owned();
    let nullable = macro_args
        .get("Nullable")
        .cloned()
        .unwrap_or_default()
        .contains("True");
    let override_function_name = macro_args.get("Function");

    let struct_name: Type = syn::parse_str(&struct_name).unwrap();
    let function_name = match override_function_name {
        Some(override_name) => Ident::new(override_name, Span::call_site()),
        None => Ident::new(&format!("Java_{}_{}", class, method), Span::call_site()),
    };
    let rust_field_name = Ident::new(&method.to_case(Case::Snake), Span::call_site());
    let value_type = Ident::new(
        rust_argument_types.get(0).unwrap().as_str(),
        Span::call_site(),
    );
    let value_clone = if nullable {
        quote! {
            if val.is_null(){
                None
            } else {
                Some(crate::clone_from_java::CloneFromJava::clone_from_java(env,val.into())?)
            }
        }
    } else {
        quote! {crate::clone_from_java::CloneFromJava::clone_from_java(env,val.into())?}
    };

    let expanded = quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "system" fn #function_name (
            env: jni::JNIEnv,
            obj: jni::objects::JObject,
            val: jni::sys::#value_type,
        ) -> jni::sys::jobject {

            let result = || -> jni::errors::Result<_> {
                let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
                let mut rust_struct = unsafe { Box::from_raw(ptr as *mut #struct_name) };
                rust_struct.#rust_field_name =  #value_clone;
                let _ptr = Box::into_raw(rust_struct);

                Ok(obj.into_inner())
            }();

            match result {
                Ok(v) => v,
                Err(jni::errors::Error::JavaException) => jni::objects::JObject::null().into_inner(),
                _ =>  panic!("{:?}", result),
            }
        }
    };
    TokenStream::from(expanded)
}
