use proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn;
use syn::Data;

#[proc_macro_derive(Diagnostic, attributes(advice, category, label, ask))]
pub fn diagnostics_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_diagnostics_macro(ast)
}

fn impl_diagnostics_macro(ast: syn::DeriveInput) -> TokenStream {
    let name = ast.ident;

    match ast.data {
        Data::Enum(enm) => {
            let mut advices: HashMap<syn::Ident, String> = HashMap::new();
            let mut labels: HashMap<syn::Ident, String> = HashMap::new();
            let mut categories: HashMap<syn::Ident, syn::Ident> = HashMap::new();

            let mut externals: Vec<syn::Ident> = Vec::new();

            let variants = enm.variants;

            for variant in variants {
                /*  */

                for attr in variant.attrs {
                    /* match attr.parse_meta() {
                        Ok(meta) => match meta {
                            syn::Meta::NameValue(nv) => {
                                if nv.path.is_ident("path") {
                                    paths.insert(variant.ident.clone(), nv.lit.clone());
                                }

                                if nv.path.is_ident("host") {
                                    hosts.insert(variant.ident.clone(), nv.lit.clone());
                                }

                                if nv.path.is_ident("url") {
                                    urls.insert(variant.ident.clone(), nv.lit.clone());
                                }
                            }
                            _ => (),
                        },
                        Err(_) => {

                        }
                    } */

                    if attr.path.is_ident("category") {
                        let id: syn::Ident = attr.parse_args().unwrap();
                        categories.insert(variant.ident.clone(), id);
                    }

                    if attr.path.is_ident("advice") {
                        let string: syn::LitStr = attr.parse_args().unwrap();
                        advices.insert(variant.ident.clone(), string.value());
                    }

                    if attr.path.is_ident("label") {
                        let string: syn::LitStr = attr.parse_args().unwrap();
                        labels.insert(variant.ident.clone(), string.value());
                    }
                }

                for field in variant.fields {
                    for attr in field.attrs {
                        if attr.path.is_ident("use") {
                            externals.push(variant.ident.clone());
                        }
                    }
                }
            }

            let advice_keys = advices.keys();
            let advice_values = advices.values();

            let label_keys = labels.keys();
            let label_values = labels.values();

            let cat_keys = categories.keys();
            let cat_values = categories.values();

            let gen = quote! {
                impl Diagnostic for #name {
                    fn category(&self) -> DiagnosticCategory {
                        use #name::*;
                        match self {
                            #( #cat_keys => DiagnosticCategory::#cat_values,)*
                            #( #externals(err) => err.category(),)*
                            _ => DiagnosticCategory::Misc
                        }
                    }

                    fn label(&self) -> String {
                        use #name::*;
                        match self {
                            #( #label_keys => #label_values.into(),)*
                            #( #externals(err) => err.label(),)*
                            _ => "crate::label".into()
                        }
                    }

                    fn advice(&self) -> Option<String> {
                        use #name::*;
                        match self {
                            #( #advice_keys => Some(#advice_values.into()),)*
                            #( #externals(err) => err.advice(),)*
                            _ => None
                        }
                    }
                }
            };

            gen.into()
        }
        Data::Struct(_) => {
            let label_string = ast.attrs.iter().find_map(|a| {
                if a.path.is_ident("label") {
                    let string: syn::LitStr = a.parse_args().unwrap();
                    Some(string.value())
                } else {
                    None
                }
            });

            let advice_string = ast.attrs.iter().find_map(|a| {
                if a.path.is_ident("advice") {
                    let string: syn::LitStr = a.parse_args().unwrap();
                    Some(string.value())
                } else {
                    None
                }
            });

            let cat_id = ast.attrs.iter().find_map(|a| {
                if a.path.is_ident("category") {
                    let string: syn::Ident = a.parse_args().unwrap();
                    Some(string)
                } else {
                    None
                }
            });

            let gen = quote! {
                impl Diagnostic for #name {
                    fn category(&self) -> DiagnosticCategory {
                        DiagnosticCategory::#cat_id
                    }

                    fn label(&self) -> String {
                        #label_string.into()
                    }

                    fn advice(&self) -> Option<String> {
                        Some(#advice_string.into())
                    }
                }
            };

            gen.into()
        }
        _ => todo!(),
    }
}
