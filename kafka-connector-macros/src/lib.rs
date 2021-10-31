use java_enum::java_enum_impl;
use java_property_getter::java_property_getter_impl;
use proc_macro::TokenStream;

mod java_enum;
mod java_property_getter;
mod utils;

#[proc_macro]
pub fn java_property_getter(input: TokenStream) -> TokenStream {
    java_property_getter_impl(input)
}

#[proc_macro_derive(JavaEnum, attributes(java_class, java_variant))]
pub fn java_enum(input: TokenStream) -> TokenStream {
    java_enum_impl(input)
}
