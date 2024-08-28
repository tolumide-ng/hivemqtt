use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Type, TypePath};


#[proc_macro_derive(Length, attributes(byte))]
pub fn length_derive(input: TokenStream) -> TokenStream {
    let xx = input.clone();
    // let item = parse_macro_input!(input as syn::Item);
    // let call_ident = Ident::new("calligraphy", Span::call_site());
    // println!("{}", call_ident);
    let _input = parse_macro_input!(input as DeriveInput);
    let name = _input.ident;
    if let syn::Data::Struct(data) = _input.data {
        // if let syn::Fields::Named(fields) = data,fields {}

        if let syn::Fields::Named(names) = data.fields {
            // let field_names = names.named.iter().map(|f| &f.ident).collect::<Vec<_>>();
            // // let field_name_strings: Vec<String> = field_names.iter().map(|ident| ident.to_string()).collect();

            // // let expanded = quote! { }

            // println!("{:#?}", field_names);

            // dbg!(&field_names);

            // for x in field_names {
            //     if let Some(idd) = x {
            //         // let xxx = stringify!(#idd);
            //         // println!("the iudd {:#?}", xxx);
            //         // // if let Ident {name, span} = idd {}

            //         // let q = quote! { #idd };
            //         let x = &idd;
            //         println!("the qqqqq {:#?}", idd);
            //     }
            // }



            names.named.iter().for_each(|f| {
                let typei = &f.ty;
                let xx = if let Type::Path(p) = typei {
                    let x = p.clone().path;
                    let xident = x.segments.first().clone().unwrap();
                    let is_optional_field = xident.clone().ident;
                    println!("the path is :::: {:#?}", is_optional_field);
                    is_optional_field == "Option"
                    // if is_optional_field && #ident.
                } else {unimplemented!()};
                

                let ident = &f.ident.clone().unwrap();
                let ident_str = &ident.to_string();
                let result = quote! {
                    if xx && self.#ident.is_some() {
                        println!("yaaaahhhh");
                    }
                };
                // result
                // let id = &f.ident.clone().unwrap();
                // println!("the ty is {:?}", id.into_token_stream());
                // println!("the ident {:#?}", id);
                // quote!{#id};
            });
        
        }
        // if let Fields::Named(fields) = x {0};
        // data.fields.iter().for_each(|f| {
        //     let name = &f.ident;
        //     let ty = &f.ty;

        //     // let sp = f.span();
        //     // ty.span()
        //     // let xx = quote! {
        //     //     #name: Option<#ty>
        //     // };
        //     // println!("fields and value {:#?}", xx);
        // });
    };



    // if let syn::Data(data) = input.

    
    // }
    // if let Item::Struct(s) = item {
    //     println!("ident :::::::::::: {:#?}", s.ident);
    //     for field in s.fields.iter() {
    //         // let span = field.span();

    //         // let fname = match &field.ident {
    //         //     Some(ident) => ident.to_string(),
    //         //     None => "unnamed".to_string(),
    //         // };
    //         // let ftype = &field.ty;
    //         // let xx = quote! {ftype};
    //         // println!("the item here >>>>>> {:#?} --->> {:#?}", fname, xx);

    //         println!("the field visibility:::::::::::::::::: {:#?}", &field.ident);
    //         for att in &field.attrs {
    //             println!("the attributes aare::::::||||| {:#?}", att);
    //         }
    //     }
    //     // let qitem = quote! { fields };
    // };
    // let xxt = item.fields;

    TokenStream::new()
    // xx
}