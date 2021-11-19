use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput};

pub fn java_enum_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_ident = input.ident;

    let class_name = input
        .attrs
        .iter()
        .find(|attr| attr.path.segments.last().unwrap().ident == "java_class")
        .map(|attr| attr.tokens.clone().into_iter().last().unwrap().to_string())
        .map(remove_first_and_last_characters)
        .expect("No java_class attribute found");

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields_from_jobject = generate_fields_from_jobject(&input.data, &enum_ident, &class_name);
    let fields_to_jobject = generate_fields_to_jobject(&input.data, &enum_ident, &class_name);

    let expanded = quote! {
        impl #impl_generics crate::clone_to_java::CloneToJava for #enum_ident #ty_generics #where_clause {
            fn clone_to_java<'a>(&self, env: jni::JNIEnv<'a>) -> jni::errors::Result<jni::objects::JValue<'a>> {
                let class = env.find_class(#class_name)?;
                let obj = match self {
                    #fields_to_jobject
                };
                Ok(jni::objects::JValue::Object(obj))
            }
        }
        impl #impl_generics crate::clone_from_java::CloneFromJava for #enum_ident #ty_generics #where_clause {

            fn clone_from_java(env: jni::JNIEnv, obj: jni::objects::JValue)-> jni::errors::Result<Self> {
                let obj = obj.l()?;
                let class = env.find_class(#class_name)?;
                assert!(env.is_instance_of(obj, class)?, "Wrong object class");
                #fields_from_jobject
                env.throw_new("java/lang/Exception", "Unknown enum value")?;
                return Err(jni::errors::Error::JavaException);
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
                    .find(|attr| attr.path.segments.last().unwrap().ident == "java_variant")
                    .map(|attr| attr.tokens.clone().into_iter().last().unwrap().to_string())
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
                    #enum_name::#name => env
                        .get_static_field(
                            class,
                            #java_variant,
                            #class,
                        )?
                        .l()?,
                }
            });
            quote! {
                #(#recurse)*
            }
        }
        Data::Struct(_) | Data::Union(_) => unimplemented!(),
    }
}

fn remove_first_and_last_characters(input: String) -> String {
    let mut chars = input.chars();
    chars.next();
    chars.next_back();
    chars.collect()
}
