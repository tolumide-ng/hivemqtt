pub(crate) mod data_size;
pub(crate) mod detect;


use data_size::{field_lens, get_size};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// Todo! Handle more varied input like --> #[byte(max(4))]
//    - this means that that expected max bytes is 4 bytes, and we should deduct the length from the input 
//          we can do something like value <= u8::MAX, value <= u16::MAX e.t.c to determine, the proper size of the input
// Error handling
// Provide support for fields inside a struct that may not use the #[byte(x)] attribute
#[proc_macro_derive(DataSize, attributes(bytes))]
pub fn length_derive(input: TokenStream) -> TokenStream {
    let vv = input.clone();
    let _input = parse_macro_input!(input as DeriveInput);
    let struct_name = _input.ident;

    let syn::Data::Struct(data_struct) = _input.data else {  return vv.into() };
    let syn::Fields::Named(named_data) = data_struct.fields else { return vv.into() };
    let token = match field_lens(named_data) {
        Ok(field_lens) => {
            quote! {
                impl #struct_name {
                    // pub(crate) fn len(&self) -> usize {
                    fn len(&self) -> usize {
                        let mut size = 0;
                        #( #field_lens )*
                        size
                    }
                }
            }
        }
        Err(e) => {
            // let xx = e.into_iter().map(|e| e.to_compile_error()).collect::<Vec<_>>()
            e[0].to_compile_error()
        }
    };
 


    TokenStream::from(token)
    // xx
}



#[proc_macro_derive(Length, attributes(bytes))]
pub fn derive_length(input: TokenStream) -> TokenStream {
    let data = input.clone();
    let dd = data.clone();
    let _input = parse_macro_input!(dd as DeriveInput);
    let struct_name = _input.ident;

    let syn::Data::Struct(data_struct) = &_input.data else { return data.into() };
    let syn::Fields::Named(fields) = &data_struct.fields else { return data.into() };

    let mut field_lens: Vec<proc_macro2::TokenStream> = vec![];

    for field in &fields.named {
        let attrs = &field.attrs;
        let has_bytes_attr = attrs.iter().find_map(|attr| {
            let syn::Attribute{meta, ..} = attr;
            let syn::Meta::List(syn::MetaList{path, tokens, ..}) = meta else {return None};
            let syn::Path{segments, ..} = path;
            if segments.first().is_some_and(|s| s.ident.to_string() == "bytes") { return Some(tokens) };
            return None;
        });

        if has_bytes_attr.is_some_and(|token| token.to_string().as_str() == "ignore") { continue }
        field_lens.push(detect::detect(&field));
    }

    let output = quote! {
        impl #struct_name {
            fn len(&self) -> usize {
                let mut size = 0;
                #( #field_lens )*
                return size;
            }
        }
    };


    TokenStream::from(output)
}