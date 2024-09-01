
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, FieldsNamed, Meta};


/// max: means the size of the item in this field is dynamic but it is a max of `y` bytes (i.e. `max_y`)
/// wl: means(with length) that after the provided/suggested bytes is used, add the length of the content in this field
///     .e.g if field `content_type` has this: `#[bytes(wl_4)]` means that the size of this field is: `4 + content_type.len()`` 
const PREFIXES: [&str; 2] = ["wl", "max"];



pub(crate) fn get_size(attrs: &Vec<Attribute>) -> Result<(Option<String>, usize), syn::Error> {
    let Some(Attribute {meta, ..}) = attrs.first() else { return Ok((None, 0)); };
    if let Meta::List(meta_list) = meta {
        for seg in &meta_list.path.segments {
            let is_byte =seg.ident == "bytes";

            if !is_byte {continue};
            
            let mut token = (&meta_list.tokens.to_string()).clone();
            let mut prefix: Option<String> = None;

            if token.starts_with(PREFIXES[0]) || token.starts_with(PREFIXES[1]) {
                let splits = token.split('_').collect::<Vec<_>>();
                prefix = Some(String::from(splits[0])); 
                token = String::from(splits[1]);
            }
            
            return match token.parse::<usize>() {
                Ok(t) => Ok((prefix, t)),
                Err(_) =>  Err(syn::Error::new(seg.ident.span(), "The size provided is invalid"))
            };
        }
    }

    // Err(syn::Error::new(proc_macro2::Span::call_site(), "Attribute not found"))
    return Ok((None, 0))
}

/// returns methods for calculation of field size on each field
pub(crate) fn field_lens(fields: FieldsNamed) -> Result<Vec<TokenStream>, Vec<syn::Error>> {
    let (oks, errs) = fields.named.iter().fold((Vec::new(), Vec::new()), |(mut oks, mut errs), f| {
        let field_name = &f.ident;
        let field_type = &f.ty;

        match get_size(&f.attrs) {
            Ok((p, length)) => {
                // let Some(f_name) = field_name else { oks.push(quote!{ size += #length + 1; }); return (oks, errs); };
                let Some(f_name) = field_name else { return (oks, errs); };
                let syn::Type::Path(type_path) = field_type else { return (oks, errs) };

                // println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ the received prefix is {:#?}", p);
                let syn::Path {segments, ..} = &type_path.path;
                let is_optional = &segments[0].ident == "Option";
                if is_optional { oks.push(quote! { 
                    if let Some(ref value) = self.#f_name {
                        size += #length + 1;
                        if let Some(ref prefix) = #p {
                            // Example logic (this is buggy for reason I can't figure out yet)
                            if prefix == PREFIXES[0] { size += value.len(); }
                            if prefix == PREFIXES[1] {} // do the max calculation here
                        }
                    }
                })}
            },
            Err(e) => errs.push(e)
        }

        return (oks, errs);
    });

    // let xxs = ffs.1.into_iter().map(|e| e.to_compile_error()).collect::<Vec<_>>();
    if errs.is_empty() { return Ok(oks) }
    return Err(errs);
}


