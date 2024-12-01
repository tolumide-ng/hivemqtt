use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, FieldsNamed, Meta};


/// max: means the size of the item in this field is dynamic but it is a max of `y` bytes (i.e. `max_y`)
/// wl: means(with length) that after the provided/suggested bytes is used, add the length of the content in this field
///     .e.g if field `content_type` has this: `#[bytes(wl_4)]` means that the size of this field is: `4 + content_type.len()`` 
#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Prefix {
    /// With length
    Wl = 0,
    Max,
    /// KeyVal (like Vec<(k, v)>)
    Kv,
    Unknown,
}

impl From<String> for Prefix {
    fn from(value: String) -> Self {
        let value = value.to_lowercase();

        match value.as_str() {
            "wl" => Prefix::Wl,
            "max" => Prefix::Max,
            "kv" => Prefix::Kv,
            _ => Prefix::Unknown,
        }
    }
}

impl std::fmt::Display for Prefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Prefix::Wl => write!(f, "wl"),
            Prefix::Max => write!(f, "max"),
            Prefix::Kv => write!(f, "kv"),
            Prefix::Unknown => write!(f, "")
        }
    }
}


pub(crate) fn get_size(attrs: &Vec<Attribute>) -> Result<(Option<Prefix>, usize), syn::Error> {
    let Some(Attribute {meta, ..}) = attrs.first() else { return Ok((None, 0)); };
    if let Meta::List(meta_list) = meta {
        for seg in &meta_list.path.segments {
            let is_byte =seg.ident == "bytes";

            
            if !is_byte {continue};
            let mut token = (&meta_list.tokens.to_string()).clone();
            let mut prefix: Option<Prefix> = None;

            if token.starts_with(&Prefix::Max.to_string()) || token.starts_with(&Prefix::Wl.to_string()) || token.starts_with(&Prefix::Kv.to_string()) {
                let splits = token.split('_').collect::<Vec<_>>();
                prefix = Some(String::from(splits[0]).into()); 
                token = String::from(splits[1]);
            }
            
            return match token.parse::<usize>() {
                Ok(t) => Ok((prefix, t)),
                Err(_) => Err(syn::Error::new(seg.ident.span(), "The size provided is invalid"))
            };
        }
    }
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
                let prefix = p.unwrap_or_else(|| Prefix::Unknown); 
                let dynamic_length = prefix == Prefix::Wl;
                let calc_max = prefix == Prefix::Max;
                let keyval = prefix == Prefix::Kv;

                let syn::Path {segments, ..} = &type_path.path;
                let is_optional = &segments[0].ident == "Option";

                if length > 0 { // if the length is 0, it means the field does not have a byte attribute,
                    // so we can ignore it
                    if is_optional && dynamic_length {
                        oks.push(quote! {
                            if let Some(ref value) = self.#f_name {
                                size += #length + 1 + value.len();
                            }
                        })
                    } else if is_optional && calc_max {
                        // to be added later
                    } else if keyval {
                        oks.push(quote! {
                            for (key, value) in self.#f_name.iter() {
                                size += 1 + #length + key.len() + #length + value.len();
                            }
                        })
                    } else if is_optional {
                        oks.push(quote! {
                            if let Some(ref value) = self.#f_name {
                                size += #length + 1;
                            }
                        })
                    } else if dynamic_length {
                        oks.push(quote! {
                            size += #length + 1 + self.#f_name.len();
                        })
                    } else {
                        oks.push(quote! {
                            size += #length + 1;
                        })
                    }
                }

            },
            Err(e) => errs.push(e)
        }

        
        return (oks, errs);
    });
    
    // let xxs = ffs.1.into_iter().map(|e| e.to_compile_error()).collect::<Vec<_>>();
    if errs.is_empty() { return Ok(oks) }
    return Err(errs);
}
