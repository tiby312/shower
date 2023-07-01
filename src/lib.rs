extern crate proc_macro;
use std::str::FromStr;

use proc_macro::TokenStream;
use syn::DeriveInput;

// #[proc_macro]
// pub fn make_answer(_item: TokenStream) -> TokenStream {
//     "fn answer() -> u32 { 42 }".parse().unwrap()
// }

#[proc_macro]
pub fn source(input: TokenStream) -> TokenStream {
    let source_text = proc_macro::Span::call_site().source_text().unwrap();

    let source_text = unindent::unindent(&source_text);

    let input2 = syn::parse_macro_input!(input as syn::Expr);

    let expanded = quote::quote!(
        (#input2,#source_text)
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
