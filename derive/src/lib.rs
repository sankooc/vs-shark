use proc_macro::TokenStream;
use quote::quote;

fn impl_packet_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Initer for #name {
            fn new() -> Self {
                Self {
                    ..Default::default()
                }
            }

            fn summary(&self) -> String {
                self.to_string()
            }
        }
    };
    gen.into()
}
#[proc_macro_derive(Packet, attributes(show_streams))]
pub fn packet_macro_derive(input: TokenStream) -> TokenStream {
    // println!("input: \"{input}\"");
    let ast = syn::parse(input).unwrap();
    impl_packet_macro(&ast)
}
// #[proc_macro_attribute]
// pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
//     attr.to_string()
//     // println!("attr: \"{attr}\"");
//     // println!("item: \"{item}\"");
//     println!("attr");
//     item
// }

fn impl_ninfo_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl crate::files::InfoPacket for #name {
            fn info(&self) -> String {
                self.to_string()
            }
            
            fn status(&self) -> String {
                "info".into()
            }
        }
    };
    gen.into()
}
#[proc_macro_derive(NINFO)]
pub fn ninfo_macro_derive(input: TokenStream) -> TokenStream {
    // println!("input: \"{input}\"");
    let ast = syn::parse(input).unwrap();
    impl_ninfo_macro(&ast)
}