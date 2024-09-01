
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, FieldsNamed, Meta};


pub(crate) fn get_size(attrs: &Vec<Attribute>) -> Result<usize, syn::Error> {
    let Some(Attribute {meta, ..}) = attrs.first() else { return Ok(0); };
    if let Meta::List(meta_list) = meta {
        for seg in &meta_list.path.segments {
            let is_byte =seg.ident == "byte";

            if !is_byte {continue};
            let token = &meta_list.tokens.to_string();
            
            return match token.parse::<usize>() {
                Ok(t) => Ok(t),
                Err(_) =>  Err(syn::Error::new(seg.ident.span(), "The size provided is invalid"))
            };
        }
    }

    // Err(syn::Error::new(proc_macro2::Span::call_site(), "Attribute not found"))
    return Ok(0)
}

/// returns methods for calculation of field size on each field
pub(crate) fn field_lens(fields: FieldsNamed) -> Result<Vec<TokenStream>, Vec<syn::Error>> {
    let (oks, errs) = fields.named.iter().fold((Vec::new(), Vec::new()), |(mut oks, mut errs), f| {
        let field_name = &f.ident;
        let field_type = &f.ty;

        match get_size(&f.attrs) {
            Ok(length) => {
                let Some(f_name) = field_name else { oks.push(quote!{size += #length + 1;}); return (oks, errs); };
                let syn::Type::Path(type_path) = field_type else { return (oks, errs) };

                let syn::Path {segments, ..} = &type_path.path;
                let is_optional = &segments[0].ident == "Option";
                if is_optional { oks.push(quote! { if let Some(ref value) = self.#f_name { size += #length + 1 } }); }
            },
            Err(e) => errs.push(e)
        }

        return (oks, errs);
    });

    // let xxs = ffs.1.into_iter().map(|e| e.to_compile_error()).collect::<Vec<_>>();
    if errs.is_empty() { return Ok(oks) }
    return Err(errs);
}