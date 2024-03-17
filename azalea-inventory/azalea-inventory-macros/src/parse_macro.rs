use syn::{
    braced,
    parse::{Parse, ParseStream, Result},
    Ident, LitInt, Token,
};

/// An identifier, colon, and number
/// `craft_result: 1`
pub struct Field {
    pub name: Ident,
    pub length: usize,
}
impl Parse for Field {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<Ident>()?;
        let _ = input.parse::<Token![:]>()?;
        let length = input.parse::<LitInt>()?.base10_parse()?;
        Ok(Self { name, length })
    }
}

/// An identifier and a list of `Field` in curly brackets
/// ```rust,ignore
/// Player {
///     craft_result: 1,
///     ...
/// }
/// ```
pub struct Menu {
    /// The menu name, e.g. `Player`
    pub name: Ident,
    pub fields: Vec<Field>,
}

impl Parse for Menu {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<Ident>()?;

        let content;
        braced!(content in input);
        let fields = content
            .parse_terminated(Field::parse, Token![,])?
            .into_iter()
            .collect();

        Ok(Self { name, fields })
    }
}

/// A list of `Menu`s
/// ```rust,ignore
/// Player {
///     craft_result: 1,
///     ...
/// },
/// ...
/// ```
pub struct DeclareMenus {
    pub menus: Vec<Menu>,
}
impl Parse for DeclareMenus {
    fn parse(input: ParseStream) -> Result<Self> {
        let menus = input
            .parse_terminated(Menu::parse, Token![,])?
            .into_iter()
            .collect();
        Ok(Self { menus })
    }
}
