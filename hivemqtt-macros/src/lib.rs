pub(crate) mod ty_attr;


use ty_attr::get_size;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Type};

// Todo! Handle more varied input like --> #[byte(max(4))]
//    - this means that that expected max bytes is 4 bytes, and we should deduct the length from the input 
//          we can do something like value <= u8::MAX, value <= u16::MAX e.t.c to determine, the proper size of the input
// Error handling
// Provide support for fields inside a struct that may not use the #[byte(x)] attribute
#[proc_macro_derive(Eleniyan, attributes(byte))]
pub fn length_derive(input: TokenStream) -> TokenStream {
    let xx = input.clone();
    // let item = parse_macro_input!(input as syn::Item);
    // let call_ident = Ident::new("calligraphy", Span::call_site());
    // println!("{}", call_ident);
    let _input = parse_macro_input!(input as DeriveInput);
    let struct_name = _input.ident;
    
    
    let field_lens = match _input.data {
        syn::Data::Struct(data_struct) => {
            if let syn::Fields::Named(data) = data_struct.fields {
                // let weights = _input.attrs;
                let op = data.named.iter().filter_map(|f| {
                    let field_name = &f.ident;
                    let field_type = &f.ty;
                    
                        let length =  get_size(&f.attrs).unwrap();

                        if let Some(f_name) = field_name {
                            if let syn::Type::Path(type_path) = field_type {
                                let syn::Path {segments, ..} = &type_path.path;
                                let is_optional = &segments[0].ident == "Option";
                                
                                if is_optional {
                                    return Some(quote! {
                                    if let Some(ref value) = self.#f_name {
                                        size += #length + 1
                                    }
                                })
                            } else {
                                // println!("((((((((((((((((((((((the value here)))))))))))))))))))))) {:#?}", type_path.path);
                                return Some(quote!{size += #length + 1;});
                            }
                        }
                    }
                    None
                }).collect::<Vec<_>>();
                op
            } else {
                let xx: Vec<proc_macro2::TokenStream> = Vec::with_capacity(0);
                xx
            }
        }
        _ => {
            let xx: Vec<proc_macro2::TokenStream> = Vec::with_capacity(0);
            xx
        }
    };



    let expanded = quote! {
        impl #struct_name {
            pub fn len(&self) -> usize {
                let mut size = 0;
                #( #field_lens )*
                size
            }
        }
    };


    TokenStream::from(expanded)
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




// fn get_size(attrs: &Vec<Attribute>) -> Result<usize, syn::Error> {

//     // println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! {:#?}", attrs);
//     let Attribute {meta, ..} = attrs.first().ok_or_else(|| syn::Error::new(proc_macro2::Span::call_site(), ""))?;
//     if let Meta::List(meta_list) = meta {
//         // println!("LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL");
//         for seg in &meta_list.path.segments {
//             let is_byte =seg.ident == "byte";

//             if !is_byte {continue};
//             let token = &meta_list.tokens.to_string();
//             // println!("the token is {:#?}", token);
            
//             return match token.parse::<usize>() {
//                 Ok(t) => Ok(t),
//                 Err(_) =>  Err(syn::Error::new(seg.ident.span(), "The size provided is invalid"))
//             };
//         }
//     }

//     Err(syn::Error::new(proc_macro2::Span::call_site(), "Attribute not found"))
// }