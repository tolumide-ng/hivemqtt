pub(crate) mod detect;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};



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
        let add_id_to_length = !(has_bytes_attr.is_some_and(|token| token.to_string().as_str() == "no_id")); // check if the field has a no_id(no identifier attribute)

        field_lens.push(detect::calculate(&field, add_id_to_length));
    }

    let output = quote! {
        impl #struct_name {
            fn len(&self) -> usize {
                let mut size = 0;
                #( #field_lens )*
                return size;
            }

            fn variable_len(int: usize) -> usize {
                if int >= 2_097_152 { return 4 }
                if int >= 16_384 { return 3 }
                if int >= 128 { return 2 }
                return 1
            }
        }
    };


    TokenStream::from(output)
}