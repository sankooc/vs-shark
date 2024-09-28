use proc_macro::TokenStream;
use quote::quote;

fn impl_packet_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl PacketBuilder for #name {
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
    let ast = syn::parse(input).unwrap();
    impl_packet_macro(&ast)
}

#[proc_macro_derive(Packet2)]
pub fn packet2_macro_derive(input: TokenStream) -> TokenStream {
    let ast:syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl PacketBuilder for #name {
            fn new() -> Self {
                Self {
                    ..Default::default()
                }
            }
            fn summary(&self) -> String {
                self.to_string()
            }
        }
        impl #name {
            pub fn create(reader: &Reader, opt: Option<PacketOpt>) -> Result<PacketContext<Self>> {
                let packet: PacketContext<Self> = Frame::create_packet();
                let mut p = packet.get().borrow_mut();
                let rs = Self::_create(reader, &packet, &mut p, opt);
                drop(p);
                rs?;
                Ok(packet)
            }
        }
    };
    gen.into()
}


#[proc_macro_derive(Packet3)]
pub fn packet3_macro_derive(input: TokenStream) -> TokenStream {
    let ast:syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl PacketBuilder for #name {
            fn new() -> Self {
                Self {
                    ..Default::default()
                }
            }
            fn summary(&self) -> String {
                self.to_string()
            }
        }
        impl #name {
            pub fn create<T>(reader: &Reader, opt: Option<T>) -> Result<PacketContext<Self>> {
                let packet: PacketContext<Self> = Frame::create_packet();
                let mut p = packet.get().borrow_mut();
                let rs = Self::_create(reader, &packet, &mut p, opt);
                drop(p);
                rs?;
                Ok(packet)
            }
        }
    };
    gen.into()
}

fn impl_ninfo_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl crate::files::InfoPacket for #name {
            fn info(&self) -> String {
                self.to_string()
            }
            
            fn status(&self) -> FIELDSTATUS {
                FIELDSTATUS::INFO
            }
        }
    };
    gen.into()
}
#[proc_macro_derive(NINFO)]
pub fn ninfo_macro_derive(input: TokenStream) -> TokenStream {
    
    let ast = syn::parse(input).unwrap();
    impl_ninfo_macro(&ast)
}

#[proc_macro_derive(BerPacket)]
pub fn packet4_macro_derive(input: TokenStream) -> TokenStream {
    let ast:syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl PacketBuilder for #name {
            fn new() -> Self {
                Self {
                    ..Default::default()
                }
            }
            fn summary(&self) -> String {
                self._summary().into()
            }
        }
        impl #name {
            pub fn create(reader: &Reader, opt: Option<PacketOpt>) -> Result<PacketContext<Self>> {
                let packet: PacketContext<Self> = Frame::create_packet();
                let mut p = packet.get().borrow_mut();
                let rs = Self::_create(reader, &packet, &mut p, opt);
                drop(p);
                rs?;
                Ok(packet)
            }
            fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _len: Option<PacketOpt>) -> Result<()> {
                p._decode(packet, reader,_len.unwrap())?;
                Ok(())
            }
        }
    };
    gen.into()
}