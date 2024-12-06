use std::fmt::Display;
use anyhow::Result;

use pcap_derive::Packet2;
use crate::common::base::{PacketContext, PacketOpt};
use crate::common::io::Reader;
use crate::common::io::AReader;


pub enum ExtensionType {
    ServerName(Vec<String>),
    Negotiation(Vec<String>),
    Version(Vec<String>),
}

//rfc6066
#[derive(Default, Packet2)]
pub struct ServerName {
    list_len: u16,
    pub names: Vec<String>,
}
impl Display for ServerName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Server Name Indication extension")
    }
}

impl ServerName {
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _:Option<PacketOpt>) -> Result<()> {
        p.list_len = packet.build_format(reader, Reader::_read16_be, Some("tls.handshake.server.name.list"),"Server Name list length: {}")?;
        let finish = reader.cursor() + p.list_len as usize;
        loop {
            if reader.cursor() >= finish {
                break;
            }
            packet.build_format(reader, Reader::_read8, Some("tls.handshake.server.name.type"),"Server Name Type: host_name ({})")?;
            let len = packet.build_format(reader, Reader::_read16_be, Some("tls.handshake.server.name.length"),"Server Name Length: {}")?;
            let read_str = |reader: &Reader| reader.read_string(len as usize);
            let hostname = packet.build_format(reader, read_str, Some("tls.handshake.server"),"Server Name: {}")?;
            // info!("host: {}", hostname);
            p.names.push(hostname);
        }
        Ok(())
    }
}

#[derive(Default, Packet2)]
pub struct Negotiation {
    pub protocols: Vec<String>,
}


impl Display for Negotiation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ALPN Protocol")
    }
}

impl Negotiation {
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _:Option<PacketOpt>) -> Result<()> {
        let total = reader.read16(true)?;
        let finish = reader.cursor() + total as usize;
        loop {
            if reader.cursor() >= finish {
                break;
            }
            let next_len = reader.read8()? as usize;
            let str = reader.read_string(next_len)?;
            packet.build_backward(reader, next_len, format!("ALPN Next Protocol: {}", str));
            p.protocols.push(str);
        }
        Ok(())
    }
}


#[derive(Default, Packet2)]
pub struct Version {
    pub versions: Vec<String>,
}


impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("TLS Versions")
    }
}

impl Version {
    fn version_map(v: &str) -> Option<&str> {
        match v {
            "0x0301" => Some("TLS 1.0"),
            "0x0302" => Some("TLS 1.1"),
            "0x0303" => Some("TLS 1.2"),
            "0x0304" => Some("TLS 1.3"),
            _ => None
        }
    }
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, ll:Option<PacketOpt>) -> Result<()> {
        let _total = ll.unwrap();
        if _total == 2 {
            let next_len = reader.read16(true)? as usize;
            let str = format!("{:#06x}", next_len);
            if let Some(_v) = Version::version_map(&str) {
                packet.build_backward(reader, next_len, format!("Supported Version: {} ({})",_v, str));
                p.versions.push(_v.into());
            }
            return Ok(());
        }
        let total = reader.read8()?;
        let finish = reader.cursor() + total as usize;
        loop {
            if reader.cursor() >= finish {
                break;
            }
            let next_len = reader.read16(true)? as usize;
            let str = format!("{:#06x}", next_len);
            if let Some(_v) = Version::version_map(&str) {
                packet.build_backward(reader, next_len, format!("Supported Versions: {} ({})",_v, str));
                p.versions.push(_v.into());
            } else {
                packet.build_backward(reader, next_len, format!("Supported Versions: Reversed"));
            }
        }
        Ok(())
    }
}

// pub struct SignatureAlgorithms {
//     hash: u8,
//     algirithm: u8,
// }