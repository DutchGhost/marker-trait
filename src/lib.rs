extern crate proc_macro;
extern crate syn;
extern crate quote;

use self::proc_macro::TokenStream;
use self::syn::{parse, ItemTrait};
use self::quote::quote;

#[proc_macro_attribute]
pub fn marker_trait(_: TokenStream, item: TokenStream) -> TokenStream {
    match parsey(item) {
        Ok(parsed) => parsed,
        Err(e) => {
            quote!(compile_error!(#e);).into()
        }
    }
}

fn parsey(item: TokenStream) -> Result<TokenStream, &'static str> {
    let parsed = parse::<ItemTrait>(item).map_err(|_| {
        "#[marker_trait] only works on traits."
    })?;

    if !parsed.generics.params.is_empty() {
        return Err("#[marker_trait] may not contain any generic parameters.");
    }

    if !parsed.items.is_empty() {
        return Err("#[marker_trait] may not contain any functions, associated types, or associated constants.");
    }

    Ok(quote!(#parsed).into())
}