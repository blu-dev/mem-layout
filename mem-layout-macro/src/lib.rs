mod helpers;
use helpers::*;

use quote::{ToTokens, quote};
use proc_macro::TokenStream;

fn try_get_ta_attr(attribute: &syn::Attribute) -> Option<Helper> {
    match attribute.path.segments.last() {
        Some(seg) if seg.ident == "ta" => {},
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
        if let Some(Helper::Size(_, expr)) = try_get_ta_attr(attribute) {
            size = Some(Helper::Size(Some(item.ident.clone()), expr));
            break;
        }
    }

    let mut offset_checks = Vec::new();
    
    for field in item.fields.iter() {
        for attribute in field.attrs.iter() {
            if let Some(Helper::Offset(_, _, expr)) = try_get_ta_attr(attribute) {
                offset_checks.push(Helper::Offset(Some(item.ident.clone()), field.ident.clone(), expr));
                break;
            }
        }
    }

    let offset_checks = offset_checks.into_iter();

    if let Some(size) = size {
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
        
        quote!(
            #[allow(non_snake_case)]
            #[test]
            pub fn #offset_check_name() {
                #(
                    #offset_checks
                )*
            }
        ).into()
    }
}
