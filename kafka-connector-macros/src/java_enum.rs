use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, quote_spanned};
use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, Data, DeriveInput, GenericParam, Generics,
};

fn remove_first_and_last_characters(input: String) -> String {
    let mut chars = input.chars();
    chars.next();
    chars.next_back();
    chars.collect()
}
pub fn java_enum_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_ident = input.ident;

    let class_name = input
        .attrs
        .iter()
        .find(|a| a.path.segments.last().unwrap().ident == "java_class")
        .map(|x| x.tokens.clone().into_iter().last().unwrap().to_string())
        .map(remove_first_and_last_characters)
        .expect("No java_class attribute found");

    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields_from_jobject = generate_fields_from_jobject(&input.data, &enum_ident, &class_name);
    let fields_to_jobject = generate_fields_to_jobject(&input.data, &enum_ident, &class_name);

    let expanded = quote! {
        impl #impl_generics crate::FromJObject for #enum_ident #ty_generics #where_clause {
            type EnumType = #enum_ident;
            fn from_jobject(env: jni::JNIEnv, obj: jni::objects::JObject)-> jni::errors::Result<#enum_ident> {
                let class = env.find_class(#class_name)?;
                assert!(env.is_instance_of(obj, class)?, "Wrong object class");
                #fields_from_jobject
                panic!("Unknown enum value")
            }
        }
        impl #impl_generics crate::ToJObject for #enum_ident #ty_generics #where_clause {
            fn to_jobject(&self, env: jni::JNIEnv) -> jni::errors::Result<jni::sys::jobject> {
                let class = env.find_class(#class_name)?;
                match self {
                    #fields_to_jobject
                }
            }
        }
    };
    TokenStream::from(expanded)
}

fn generate_fields_from_jobject(
    data: &syn::Data,
    enum_name: &Ident,
    class_name: &str,
) -> quote::__private::TokenStream {
    let class = format!("L{};", class_name);
    match *data {
        Data::Enum(ref data) => {
            let recurse = data.variants.iter().map(|variant| {
                let name = &variant.ident;
                let v_name = name.to_string();
                let java_variant = variant
                    .attrs
                    .iter()
                    .find(|a| a.path.segments.last().unwrap().ident == "java_variant")
                    .map(|x| x.tokens.clone().into_iter().last().unwrap().to_string())
                    .map(remove_first_and_last_characters)
                    .unwrap_or_else(|| panic!("No java_variant attribute on variant {}", v_name));
                quote_spanned! { variant.span() =>
                    let variant = env
                        .get_static_field(
                            class,
                            #java_variant,
                            #class,
                        )?
                        .l()?;
                    if env.is_same_object(obj, variant)? {
                        return Ok(#enum_name::#name);
                    }
                }
            });
            quote! {
                #(#recurse)*
            }
        }
        Data::Struct(_) | Data::Union(_) => unimplemented!(),
    }
}

fn generate_fields_to_jobject(
    data: &syn::Data,
    enum_name: &Ident,
    class_name: &str,
) -> quote::__private::TokenStream {
    let class = format!("L{};", class_name);
    match *data {
        Data::Enum(ref data) => {
            let recurse = data.variants.iter().map(|variant| {
                let name = &variant.ident;
                let v_name = name.to_string();
                let java_variant = variant
                    .attrs
                    .iter()
                    .find(|a| a.path.segments.last().unwrap().ident == "java_variant")
                    .map(|x| x.tokens.clone().into_iter().last().unwrap().to_string())
                    .map(remove_first_and_last_characters)
                    .unwrap_or_else(|| panic!("No java_variant attribute on variant {}", v_name));
                quote_spanned! { variant.span() =>
                    #enum_name::#name => Ok(env
                        .get_static_field(
                            class,
                            #java_variant,
                            #class,
                        )?
                        .l()?
                        .into_inner()),
                }
            });
            quote! {
                #(#recurse)*
            }
        }
        Data::Struct(_) | Data::Union(_) => unimplemented!(),
    }
}

pub fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(FromBytes));
        }
    }
    generics
}
