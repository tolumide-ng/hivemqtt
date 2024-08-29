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
            let field_names = names.named.iter().map(|f| f).collect::<Vec<_>>();


            for field in field_names {
                // println!("yessir");
                let is_field_optional = field_type("Option", &field.ty);
                // if let Type::Path(f_type_path) = &field.ty {
                //     let f_path = &f_type_path.path.segments[0];
                //     let f_type = &f_path.ident;
                //     let is_optional = f_type == "Option";
                //     println!("THE TYPE HERE IS {:#?} ------>>> {:#?}", f_type, is_optional);

                //     println!("{:#?}", f_path);
                // }

                let ident = &field.ident;

                
                let var = quote! { println!("{:#?}", #ident) };
                // println!("the identity HERE IS {:#?}", var);

                // println!("{:#?}", is_field_optional);

                let field_name = &field.ident;

                let vv = quote! {
                    if self.#field_name.is_some() {
                        println!("((((((((((((((((((((((((welcome)))))))))))))))))))))))) to some");
                        return 1;
                    } else {
                        println!("************************OHHHHH************************ it was a NONEYU");
                        return 2;
                    }
                };

                println!("THE [[[[[[[[VVVVVVVVV]]]]]]]] {:#?}", vv);

            }



            names.named.iter().for_each(|f| {
                let typei = &f.ty;
                let xx = if let Type::Path(p) = typei {
                    let x = p.clone().path;
                    let xident = x.segments.first().clone().unwrap();
                    let is_optional_field = xident.clone().ident;
                    println!("the path is :::: {:#?}", is_optional_field);
                    // is_optional_field == "Option";
                    // println!("======================================== {:#?}", is_optional_field);
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
    TokenStream::new()
    // xx
}


fn field_type(expected: &str, ty: &syn::Type) -> bool {
    if let Type::Path(ref f_type_path) = ty {
        let f_path = &f_type_path.path.segments[0];
        let f_type = &f_path.ident;
        let matches_expected = f_type == expected;
        return matches_expected;
    }
    return false;
}