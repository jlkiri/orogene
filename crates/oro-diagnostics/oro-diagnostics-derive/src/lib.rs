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

            let variants = enm.variants;

            for variant in variants {
                for attr in variant.attrs {
                    if attr.path.is_ident("advice") {
                        let string: syn::LitStr = attr.parse_args().unwrap();
                        advices.insert(variant.ident.clone(), string.value());
                    }
                }
            }

            dbg!(&advices);

            let keys = advices.keys();
            let values = advices.values();

            let gen = quote! {
                impl Diagnostic for #name {
                    fn category(&self) -> DiagnosticCategory {
                        DiagnosticCategory::Misc
                    }

                    fn label(&self) -> String {
                        "crate::label".into()
                    }

                    fn advice(&self) -> Option<String> {
                        use #name::*;
                        match self {
                            #( #keys => Some(#values.into()),)*
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
