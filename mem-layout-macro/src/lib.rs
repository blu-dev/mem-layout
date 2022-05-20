mod helpers;
use helpers::*;

use proc_macro2::Span;
use quote::{ToTokens, quote};
use proc_macro::TokenStream;

fn try_get_ta_attr(attribute: &syn::Attribute) -> Option<Helper> {
    match attribute.path.segments.last() {
        Some(seg) if seg.ident.to_string() == "ta" => {},
        _ => return None
    }

    syn::parse(attribute.tokens.to_token_stream().into())
        .ok()
}

#[proc_macro_derive(TypeAssert, attributes(ta))]
pub fn derive_type_assert(item: TokenStream) -> TokenStream {
   let item = syn::parse_macro_input!(item as syn::ItemStruct);

    let mut size = None;

    for attribute in item.attrs.iter() {
        match try_get_ta_attr(attribute) {
            Some(Helper::Size(_, expr)) => {
                size = Some(Helper::Size(Some(item.ident.clone()), expr));
                break;
            },
            _ => {}
        }
    }

    let mut offset_checks = Vec::new();
    let mut fields = Vec::new();
    
    for field in item.fields.iter() {
        for attribute in field.attrs.iter() {
            match try_get_ta_attr(attribute) {
                Some(h) => match h {
                    Helper::Offset(_, _, e) => {
                        offset_checks.push(Helper::Offset(Some(item.ident.clone()), field.ident.clone(), e));
                        break;
                    },
                    _ => {}
                },
                _ => {}
            }
        }
        fields.push(HelperOrField::Field(field));
    }

    let offset_checks = offset_checks.into_iter();

    if size.is_some() {
        let size = size.unwrap();

        let size_check_name = quote::format_ident!("{}_size_check", item.ident);
        let offset_check_name = quote::format_ident!("{}_offset_check", item.ident);

        quote!(
            #[allow(non_snake_case)]
            #[test]
            pub fn #size_check_name() {
                #size
            }

            #[allow(non_snake_case)]
            #[test]
            pub fn #offset_check_name() {
                #(
                    #offset_checks
                )*
            }
        ).into()
    } else {
        let offset_check_name = quote::format_ident!("{}_offset_check", item.ident);
        
        panic!("{}", quote!(
            #[allow(non_snake_case)]
            #[test]
            pub fn #offset_check_name() {
                #(
                    #offset_checks
                )*
            }
        ).to_token_stream().to_string());
    }
}