use java_enum::java_enum_impl;
use java_property_getter::java_property_getter_impl;
use proc_macro::TokenStream;
use rust_property_getter::rust_property_getter_impl;

mod java_enum;
mod java_property_getter;
mod rust_property_getter;
mod utils;

/// If Signature includes arrays(e.g. `[B`) delimiter must be closed to be parsed by rust correctly(`[]B`)
#[proc_macro]
pub fn java_property_getter(input: TokenStream) -> TokenStream {
    java_property_getter_impl(input)
}

#[proc_macro_derive(JavaEnum, attributes(java_class, java_variant))]
pub fn java_enum(input: TokenStream) -> TokenStream {
    java_enum_impl(input)
}

/// If Signature includes arrays(e.g. `[B`) delimiter must be closed to be parsed by rust correctly(`[]B`)
#[proc_macro]
pub fn rust_property_getter(input: TokenStream) -> TokenStream {
    rust_property_getter_impl(input)
}
