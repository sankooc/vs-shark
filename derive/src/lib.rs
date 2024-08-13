use proc_macro::TokenStream;
use quote::quote;

fn impl_packet_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Initer for #name {
            fn new(protocol: Protocol) -> Self {
                Self {
                    protocol,
                    ..Default::default()
                }
            }

            fn summary(&self) -> String {
                self._summary()
            }
        }
        impl ContainProtocol for #name {
            fn get_protocol(&self) -> Protocol {
              self.protocol.clone()
            }
        }
    };
    gen.into()
}
#[proc_macro_derive(Packet)]
pub fn packet_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_packet_macro(&ast)
}
