use quote::{ quote, ToTokens };
use proc_macro2::{ Span, TokenStream };
use proc_macro_crate::FoundCrate;
use syn::{parse::{Parse, ParseStream}, parenthesized};

mod kw {
    syn::custom_keyword!(size);
    syn::custom_keyword!(offset);
    syn::custom_keyword!(off);
}

#[allow(dead_code)]
struct AttrItem<Keyword: Parse, Item: Parse> {
    keyword: Keyword,
    item: Item
}

impl<Keyword: Parse, Item: Parse> Parse for AttrItem<Keyword, Item> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let keyword = input.parse()?;
        let _: syn::Token![=] = input.parse().unwrap();
        let item = input.parse()?;
        Ok(Self {
            keyword,
            item
        })
    }
}


pub enum Helper {
    Size(Option<syn::Ident>, syn::Expr),
    Offset(Option<syn::Ident>, Option<syn::Ident>, syn::Expr)
}

impl Helper {
    fn size_to_tokens(&self, tokens: &mut TokenStream) {
        let (id, expr) = match self { 
            Self::Size(id, expr) => (id, expr),
            _ => unimplemented!()
        };

        if id.is_none() {
            syn::Error::new(Span::call_site(), "size is not associated with a type")
                .into_compile_error()
                .to_tokens(tokens);
            return;
        }

        let id = id.as_ref().unwrap();
        
        quote!(
            assert_eq!(
                std::mem::size_of::<#id>(),
                #expr,
                "The size of {} must be {} but it is {}",
                stringify!(#id),
                #expr,
                std::mem::size_of::<#id>()
            );
        ).to_tokens(tokens)
    }

    fn offset_to_tokens(&self, tokens: &mut TokenStream) {
        let (ty_id, id, expr) = match self {
            Self::Offset(ty_id, id, expr) => (ty_id, id, expr),
            _ => unimplemented!()
        };

        if ty_id.is_none() {
            syn::Error::new(Span::call_site(), "offset is not associated with a type")
                .into_compile_error()
                .to_tokens(tokens);
            return;
        }

        if id.is_none() {
            syn::Error::new(Span::call_site(), "offset is not associated with a field")
                        .into_compile_error()
                        .to_tokens(tokens);
            return;
        }

        let ty_id = ty_id.as_ref().unwrap();
        let id = id.as_ref().unwrap();

        let mem_layout_crate = proc_macro_crate::crate_name("mem-layout");

        match mem_layout_crate {
            Ok(FoundCrate::Itself) => {
                quote!(
                    assert_eq!(
                        ::mem_layout::offset_of!(#ty_id, #id),
                        #expr,
                        "The offset of {}.{} must be {} but it is {}",
                        stringify!(#ty_id),
                        stringify!(#id),
                        #expr,
                        ::mem_layout::offset_of!(#ty_id, #id)
                    );
                ).to_tokens(tokens)
            },
            Ok(FoundCrate::Name(new_name)) => {
                let crate_ident = syn::Ident::new(new_name.as_str(), Span::call_site());
                quote!(
                    assert_eq!(
                        #crate_ident::offset_of!(#ty_id, #id),
                        #expr,
                        "The offset of {}.{} must be {} but it is {}",
                        stringify!(#ty_id),
                        stringify!(#id),
                        #expr,
                        #crate_ident::offset_of!(#ty_id, #id)
                    );
                ).to_tokens(tokens)
            },
            Err(e) => {
                syn::Error::new(Span::call_site(), e).into_compile_error().to_tokens(tokens)
            }
        }
    }

    fn parse_size(input: ParseStream) -> syn::Result<Self> {
        let AttrItem::<kw::size, syn::Expr> { item: size, .. } = input.parse()?;
        Ok(Self::Size(None, size))
    }

    fn parse_offset(input: ParseStream) -> syn::Result<Self> {
        let offset = if let Ok(AttrItem::<kw::off, syn::Expr> { item: offset, .. }) = input.parse() {
            offset
        } else {
            let AttrItem::<kw::offset, syn::Expr> { item: offset, .. } = input.parse()?;
            offset
        };

        Ok(Self::Offset(None, None, offset))
    }
}

impl Parse for Helper {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let _ = parenthesized!(content in input);
        if content.peek(kw::size) {
            Self::parse_size(&content)
        } else {
            Self::parse_offset(&content)
        }
    }
}

impl ToTokens for Helper {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Size(_, _) => self.size_to_tokens(tokens),
            Self::Offset(_, _, _) => self.offset_to_tokens(tokens)
        }
    }
}