extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn source(input: TokenStream) -> TokenStream {
    let input2 = syn::parse_macro_input!(input as syn::Expr);

    let source_text = proc_macro::Span::call_site().source_text().unwrap();

    let source_text = unindent::unindent(&source_text);

    let expanded = quote::quote!(
        (#input2,#source_text)
    );

    TokenStream::from(expanded)
}
