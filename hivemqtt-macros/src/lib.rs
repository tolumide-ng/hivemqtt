use proc_macro::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{parse_macro_input, spanned::Spanned, DeriveInput, Type, TypePath};


#[proc_macro_derive(Length, attributes(byte))]
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
                let op = data.named.iter().filter_map(|f| {
                    let field_name = &f.ident;
                    let field_type = &f.ty;
    
                    if let Some(f_name) = field_name {
                        if let syn::Type::Path(type_path) = field_type {
                            if type_path.path.is_ident("Option") {
                                return Some(quote! {
                                    if let Some(ref value) = self.#f_name {
                                        size += 1                               }
                                })
                            } else {
                                return Some(quote!{size += 1;});
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