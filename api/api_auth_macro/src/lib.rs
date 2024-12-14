use proc_macro::TokenStream;
use syn::{parse::Parse, parse_macro_input, visit_mut::VisitMut, ItemFn, LitStr};

struct Scope {
    scopes: Vec<String>,
}

impl Parse for Scope {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        syn::punctuated::Punctuated::<LitStr, syn::Token![,]>::parse_terminated(input).map(
            |scopes| Scope {
                scopes: scopes.into_iter().map(|s| s.value()).collect(),
            },
        )
    }
}

impl VisitMut for Scope {
    fn visit_item_fn_mut(&mut self, i: &mut syn::ItemFn) {
        i.block.stmts.insert(
            0,
            syn::parse_quote! {
                println!("Hello, world!");
            },
        );
    }
}

#[proc_macro_attribute]
pub fn require_scope(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemFn);
    let mut scope: Scope = parse_macro_input!(attr as Scope);
    scope.visit_item_fn_mut(&mut input);

    TokenStream::from(quote::quote! {
        #input
    })
}

#[cfg(test)]
mod tests {
    use super::*;
}
