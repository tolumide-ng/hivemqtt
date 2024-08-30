
use syn::{Attribute, Meta};

#[derive(Clone, Debug, thiserror::Error)]
pub(crate)  enum LengthError {
    #[error("The length provided is an invalid usize: {0}")]
    MalformedLength(String),
    #[error("Byte attribute not found on this field")]
    AttributeNotFound
}

pub(crate) fn get_size(attrs: &Vec<Attribute>) -> Result<usize, syn::Error> {

    // println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! {:#?}", attrs);
    let Attribute {meta, ..} = attrs.first().ok_or_else(|| syn::Error::new(proc_macro2::Span::call_site(), ""))?;
    if let Meta::List(meta_list) = meta {
        // println!("LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL");
        for seg in &meta_list.path.segments {
            let is_byte =seg.ident == "byte";

            if !is_byte {continue};
            let token = &meta_list.tokens.to_string();
            // println!("the token is {:#?}", token);
            
            return match token.parse::<usize>() {
                Ok(t) => Ok(t),
                Err(_) =>  Err(syn::Error::new(seg.ident.span(), "The size provided is invalid"))
            };
        }
    }

    Err(syn::Error::new(proc_macro2::Span::call_site(), "Attribute not found"))
}