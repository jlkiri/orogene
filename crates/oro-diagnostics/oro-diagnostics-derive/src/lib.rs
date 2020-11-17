use proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Diagnostics)]
pub fn diagnostics_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_diagnostics_macro(&ast)
}

fn impl_diagnostics_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Diagnostic for #name {
            fn category(&self) -> DiagnosticCategory {
                DiagnosticCategory::Misc
            }

            fn label(&self) -> String {
                "crate::label".into()
            }

            fn advice(&self) -> Option<String> {
                None
            }
        }
    };
    gen.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
