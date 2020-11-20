use proc_macro::TokenStream;
use quote::quote;
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
            let variants = enm.variants;

            let cat_arms = variants.iter().map(|variant| {
                let id = &variant.ident;

                let cat = variant.attrs.iter().find_map(|a| {
                    if a.path.is_ident("category") {
                        let id: syn::Ident = a.parse_args().unwrap();
                        Some(id)
                    } else {
                        None
                    }
                });

                let has_ask_attr: Vec<bool> = variant
                    .fields
                    .iter()
                    .map(|field| field.attrs.iter().any(|attr| attr.path.is_ident("ask")))
                    .collect();
                let should_ask = has_ask_attr.contains(&true);

                match variant.fields {
                    syn::Fields::Unit => {
                        let cat_arms = cat.map(|c| {
                            quote! {
                                #id => DiagnosticCategory::#c,
                            }
                        });

                        cat_arms
                    }
                    syn::Fields::Named(_) => {
                        let cat_arms = cat.map(|c| {
                            quote! {
                                #id {..} => DiagnosticCategory::#c,
                            }
                        });

                        cat_arms
                    }
                    syn::Fields::Unnamed(_) => {
                        if should_ask {
                            return Some(quote! {
                                #id(err) => err.category(),
                            });
                        }

                        let cat_arms = cat.map(|c| {
                            quote! {
                                #id(..) => DiagnosticCategory::#c,
                            }
                        });

                        cat_arms
                    }
                }
            });

            let label_arms = variants.iter().map(|variant| {
                let id = &variant.ident;

                let labels = variant.attrs.iter().find_map(|a| {
                    if a.path.is_ident("label") {
                        let string: syn::LitStr = a.parse_args().unwrap();
                        Some(string.value())
                    } else {
                        None
                    }
                });

                let has_ask_attr: Vec<bool> = variant
                    .fields
                    .iter()
                    .map(|field| field.attrs.iter().any(|attr| attr.path.is_ident("ask")))
                    .collect();
                let should_ask = has_ask_attr.contains(&true);

                match variant.fields {
                    syn::Fields::Unit => {
                        let label_arms = labels.map(|l| {
                            quote! {
                                #id => #l.into(),
                            }
                        });

                        label_arms
                    }
                    syn::Fields::Named(_) => {
                        let label_arms = labels.map(|l| {
                            quote! {
                                #id {..} => #l.into(),
                            }
                        });

                        label_arms
                    }
                    syn::Fields::Unnamed(_) => {
                        if should_ask {
                            return Some(quote! {
                                #id(err) => err.label(),
                            });
                        }

                        let label_arms = labels.map(|l| {
                            quote! {
                                #id(..) => #l.into(),
                            }
                        });

                        label_arms
                    }
                }
            });

            let advice_arms = variants.iter().map(|variant| {
                let id = &variant.ident;

                let advices = variant.attrs.iter().find_map(|a| {
                    if a.path.is_ident("advice") {
                        let string: syn::LitStr = a.parse_args().unwrap();
                        Some(string.value())
                    } else {
                        None
                    }
                });

                let has_ask_attr: Vec<bool> = variant
                    .fields
                    .iter()
                    .map(|field| field.attrs.iter().any(|attr| attr.path.is_ident("ask")))
                    .collect();
                let should_ask = has_ask_attr.contains(&true);

                println!("{} : {}", id, should_ask);

                match variant.fields {
                    syn::Fields::Unit => {
                        let advices = advices.map(|a| {
                            quote! {
                                #id => Some(#a.into()),
                            }
                        });

                        advices
                    }
                    syn::Fields::Named(_) => {
                        let advices = advices.map(|a| {
                            quote! {
                                #id {..} => Some(#a.into()),
                            }
                        });

                        advices
                    }
                    syn::Fields::Unnamed(_) => {
                        if should_ask {
                            return Some(quote! {
                                #id(err) => err.advice(),
                            });
                        };

                        let advices = advices.map(|a| {
                            quote! {
                                #id(..) => Some(#a.into()),
                            }
                        });

                        advices
                    }
                }
            });

            let gen = quote! {
                impl Diagnostic for #name {
                    fn category(&self) -> DiagnosticCategory {
                        use #name::*;
                        match self {
                             #(#cat_arms)*
                            _ => DiagnosticCategory::Misc
                        }
                    }

                    fn label(&self) -> String {
                        use #name::*;
                        match self {
                            #(#label_arms)*
                            _ => "crate::label".into()
                        }
                    }

                    fn advice(&self) -> Option<String> {
                        use #name::*;
                        match self {
                            #(#advice_arms)*
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

            println!("{} : {:?}", name, advice_string);

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
