use proc_macro::TokenStream;
use quote::quote;
use syn::{
    braced,
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
    punctuated::Punctuated,
    Attribute, Ident, LitStr, Token,
};

struct Registry {
    name: Ident,
    items: Vec<RegistryItem>,
    attrs: Vec<Attribute>,
}

struct RegistryItem {
    attrs: Vec<Attribute>,
    name: Ident,
    id: String,
}

impl Parse for RegistryItem {
    // Air => "minecraft:air"
    fn parse(input: ParseStream) -> Result<Self> {
        // parse annotations like #[default]
        let attrs = input.call(Attribute::parse_outer).unwrap_or_default();

        let name = input.parse()?;
        input.parse::<Token![=>]>()?;
        let id = input.parse::<LitStr>()?.value();
        Ok(RegistryItem { attrs, name, id })
    }
}

impl Parse for Registry {
    fn parse(input: ParseStream) -> Result<Self> {
        // enum Block {
        //     Air => "minecraft:air",
        //     Stone => "minecraft:stone"
        // }

        // this also includes docs
        let attrs = input.call(Attribute::parse_outer).unwrap_or_default();

        input.parse::<Token![enum]>()?;
        let name = input.parse()?;
        let content;
        braced!(content in input);
        let items: Punctuated<RegistryItem, _> =
            content.parse_terminated(RegistryItem::parse, Token![,])?;

        Ok(Registry {
            name,
            items: items.into_iter().collect(),
            attrs,
        })
    }
}

#[proc_macro]
pub fn registry(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Registry);
    let name = input.name;
    let mut generated = quote! {};

    // enum Block {
    //     Air = 0,
    //     Stone,
    // }
    let mut enum_items = quote! {};
    for (i, item) in input.items.iter().enumerate() {
        let attrs = &item.attrs;
        let name = &item.name;
        let protocol_id = i as u32;
        enum_items.extend(quote! {
            #(#attrs)*
            #name = #protocol_id,
        });
    }
    let attributes = input.attrs;
    generated.extend(quote! {
        #(#attributes)*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, azalea_buf::AzBuf, simdnbt::ToNbtTag, simdnbt::FromNbtTag)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        #[repr(u32)]
        pub enum #name {
            #enum_items
        }
    });

    let max_id = input.items.len() as u32;

    let doc_0 = format!("Transmutes a u32 to a {name}.");
    let doc_1 = format!("The `id` should be at most {max_id}.");

    generated.extend(quote! {
        impl #name {
            #[doc = #doc_0]
            ///
            /// # Safety
            #[doc = #doc_1]
            #[inline]
            pub unsafe fn from_u32_unchecked(id: u32) -> Self {
                std::mem::transmute::<u32, #name>(id)
            }

            #[inline]
            pub fn is_valid_id(id: u32) -> bool {
                id <= #max_id
            }
        }
        impl Registry for #name {
            fn from_u32(value: u32) -> Option<Self> {
                if Self::is_valid_id(value) {
                    Some(unsafe { Self::from_u32_unchecked(value) })
                } else {
                    None
                }
            }
            fn to_u32(&self) -> u32 {
                *self as u32
            }
        }
    });

    let doc_0 = format!("Safely transmutes a u32 to a {name}.");

    generated.extend(quote! {
        impl TryFrom<u32> for #name {
            type Error = ();

            #[doc = #doc_0]
            fn try_from(id: u32) -> Result<Self, Self::Error> {
                if let Some(value) = Self::from_u32(id) {
                    Ok(value)
                } else {
                    Err(())
                }
            }
        }
    });

    // Display that uses registry ids
    let mut display_items = quote! {};
    let mut from_str_items = quote! {};
    for item in &input.items {
        let name = &item.name;
        let id = &item.id;
        display_items.extend(quote! {
            Self::#name => write!(f, #id),
        });
        from_str_items.extend(quote! {
            #id => Ok(Self::#name),
        });
    }
    generated.extend(quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #display_items
                }
            }
        }
        impl std::str::FromStr for #name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #from_str_items
                    _ => Err(format!("{s:?} is not a valid {name}", s = s, name = stringify!(#name))),
                }
            }
        }
    });

    generated.into()
}
