extern crate proc_macro;
use std::str::FromStr;

use proc_macro::TokenStream;
use syn::parse::Parse;
// #[proc_macro]
// pub fn make_answer(_item: TokenStream) -> TokenStream {
//     "fn answer() -> u32 { 42 }".parse().unwrap()
// }

use syn::{Expr, Ident, Token};

struct MacroInput {
    a: Ident,
    comma1: Token![,],
    b: Ident,
    comma2: Token![,],
    c: Expr,
}

impl Parse for MacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            a: input.parse()?,
            comma1: input.parse()?,
            b: input.parse()?,
            comma2: input.parse()?,
            c: input.parse()?,
        })
    }
}

use syn::spanned::Spanned;
#[proc_macro]
pub fn source(input: TokenStream) -> TokenStream {
    let source_text = proc_macro::Span::call_site().source_text().unwrap();

    let input2 = syn::parse_macro_input!(input as MacroInput);
    let a = input2.a;
    let b = input2.b;
    let c = input2.c;
    //let source_text = input2.span().source_text().unwrap();

    let expanded = quote::quote!(
        let #a = #c;
        let #b = #source_text;
    );

    TokenStream::from(expanded)

    // //let source_text = source_text.trim_start_matches("shower::python!(|| {");
    // //let source_text = source_text.trim_end_matches("})");

    // let source = format!("{source_text:?}");
    // let k: TokenStream = source.parse().unwrap();

    // let code = format!("{}{}{}", "{", input.to_string(), "}");

    // let open =
    //     TokenStream::from_str(&format!("{}({},{}){}", "{", code, k.to_string(), "}")).unwrap();

    // open
}

// #[proc_macro]
// pub fn flaap(mut item: TokenStream) -> TokenStream {

//     let expr = syn::parse_str::<syn::Expr>(&item.to_string()).unwrap();

//     let expanded = quote::quote! {
//         #expr
//     };

//     TokenStream::from(expanded)
//     // let k=format!("{:#?}",expr);

//     // expr.try_into().unwrap()
//     // let mut s=item.to_string();
//     // //s.retain(|a|a!='\"' && a!='\"' && a!='\"');
//     // s.
//     // format!("{}",s).parse().unwrap()

//     // let j:TokenStream=format!("let foo=r##\"{}\"##;",item.clone().to_string()).parse().unwrap();
//     // //let j=item.clone().to_string();
//     // //let jj:TokenStream=format!("let j=\"{}\"",j).parse().unwrap();
//     // //"struct Foo;".parse().unwrap()
//     // //item.extend(jj);
//     // item.extend(j);
//     // item
// }
