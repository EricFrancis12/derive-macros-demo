use proc_macro::TokenStream;

use deluxe::ExtractAttributes;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(ExtractAttributes)]
#[deluxe(attributes(friendly_name))]
struct FriendlyNameAttribute(String);

#[proc_macro_derive(Identify, attributes(friendly_name))]
pub fn derive_identify(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input);
    let type_name = input.ident.clone();
    let type_name_string = type_name.clone().to_string();

    let FriendlyNameAttribute(friendly_name_string) =
        deluxe::extract_attributes(&mut input).unwrap();

    quote! {
        impl crate::Identify for #type_name {
            fn type_name(&self) -> &'static str {
               #type_name_string
            }

            fn friendly_type_name(&self)-> &'static str {
                #friendly_name_string
            }
        }
    }
    .into()
}
