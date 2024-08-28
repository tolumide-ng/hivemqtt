use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, token::Struct, Item};


#[proc_macro_derive(Length, attributes(byte))]
pub fn length_derive(input: TokenStream) -> TokenStream {
    let xx = input.clone();
    let item = parse_macro_input!(input as syn::Item);
    if let Item::Struct(s) = item {
        println!("ident :::::::::::: {:#?}", s.ident);
        for field in s.fields.iter() {
            // let span = field.span();

            // let fname = match &field.ident {
            //     Some(ident) => ident.to_string(),
            //     None => "unnamed".to_string(),
            // };
            // let ftype = &field.ty;
            // let xx = quote! {ftype};
            // println!("the item here >>>>>> {:#?} --->> {:#?}", fname, xx);

            println!("the field visibility:::::::::::::::::: {:#?}", &field.ident);
            for att in &field.attrs {
                println!("the attributes aare::::::||||| {:#?}", att);
            }
        }
        // let qitem = quote! { fields };
    };
    // let xxt = item.fields;

    TokenStream::new()
}