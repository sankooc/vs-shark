use std::cell::RefMut;
use std::fmt::Display;
use anyhow::Result;

use log::info;
use pcap_derive::Packet;
use crate::files::{Frame, Initer, PacketContext};
use crate::common::Reader;


pub enum ExtensionType {
    ServerName(ServerName)
}

//rfc6066
#[derive(Default, Clone, Packet)]
pub struct ServerName {
    list_len: u16,
    names: Vec<String>,
}
impl Display for ServerName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl ServerName {
    pub fn create(reader: &Reader) -> Result<PacketContext<Self>> {
        let packet: PacketContext<Self> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        Self::_create(reader, &packet, &mut p)?;
        drop(p);
        Ok(packet)
    }
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>) -> Result<()> {
        p.list_len = packet.build_format(reader, Reader::_read16_be, "Server Name list length: {}")?;
        let finish = reader.cursor() + p.list_len as usize;
        loop {
            if reader.cursor() >= finish {
                break;
            }
            packet.build_format(reader, Reader::_read8, "Server Name Type: host_name ({})")?;
            let len = packet.build_format(reader, Reader::_read16_be, "Server Name Length: {}")?;
            // if len + 3 != p.list_len{
            // }
            let read_str = |reader: &Reader| reader.read_string(len as usize);
            let hostname = packet.build_format(reader, read_str, "Server Name: {}")?;
            // info!("host: {}", hostname);
            p.names.push(hostname);
        }
        Ok(())
    }
}