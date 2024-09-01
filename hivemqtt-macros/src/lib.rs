pub(crate) mod data_size;


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
                    pub fn len(&self) -> usize {
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
