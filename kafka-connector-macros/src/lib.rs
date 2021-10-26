use phase_1::java_property_getter_impl;
use proc_macro::TokenStream;

mod phase_1;
mod utils;

#[proc_macro]
pub fn java_property_getter(input: TokenStream) -> TokenStream {
    java_property_getter_impl(input)
}
