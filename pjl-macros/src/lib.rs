use proc_macro::*;

#[proc_macro]
pub fn concat(_item: TokenStream) -> TokenStream {
    let t = TokenTree::Ident(Ident::new("test", Span::call_site()));
    TokenStream::from(t)
}
