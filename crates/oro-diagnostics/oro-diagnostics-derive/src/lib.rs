use proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn;
use syn::Data;

#[proc_macro_derive(Diagnostic, attributes(advice, category, label))]
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

            let variants = enm.variants;

            for variant in variants {
                for attr in variant.attrs {
                    if attr.path.is_ident("advice") {
                        let string: syn::LitStr = attr.parse_args().unwrap();
                        advices.insert(variant.ident.clone(), string.value());
                    }

                    if attr.path.is_ident("label") {
                        let string: syn::LitStr = attr.parse_args().unwrap();
                        labels.insert(variant.ident.clone(), string.value());
                    }
                }
            }

            dbg!(&advices);

            let advice_keys = advices.keys();
            let advice_values = advices.values();

            let label_keys = labels.keys();
            let label_values = labels.values();

            let gen = quote! {
                impl Diagnostic for #name {
                    fn category(&self) -> DiagnosticCategory {
                        DiagnosticCategory::Misc
                    }

                    fn label(&self) -> String {
                        use #name::*;
                        match self {
                            #( #label_keys => #label_values.into(),)*
                            _ => "crate::label".into()
                        }
                    }

                    fn advice(&self) -> Option<String> {
                        use #name::*;
                        match self {
                            #( #advice_keys => Some(#advice_values.into()),)*
                            _ => None
                        }
                    }
                }
            };

            dbg!(&gen);

            gen.into()
        }
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
