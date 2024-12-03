use std::fmt::Display;

use crate::constants::ieee802_mnt_tags_mapper;
use crate::{common::io::AReader, constants::ieee802_mnt_cat_mapper};
use crate::common::base::PacketOpt;
use anyhow::{bail, Result};
use pcap_derive::{Packet, Packet2};

use crate::{
    common::base::{Frame, PacketBuilder, PacketContext},
    common::io::Reader,
};

use super::i802::IEE80211;

#[derive(Default)]
#[allow(dead_code)]
pub enum TagData {
    SSID(String),
    UNKNOWN(Vec<u8>),
    #[default]
    DEF
}

#[derive(Default, Packet2)]
pub struct ManagementTag {
    _type: u8,
    _type_str: &'static str,
    len: u8,
    data: TagData,
}
impl Display for ManagementTag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!("Tag: {}", self._type_str))
    }
}

impl ManagementTag {
    /// Parse a single tagged parameter from a management frame.
    ///
    /// `reader` is the `Reader` containing the bytes of the tagged parameter.
    /// `packet` is the `PacketContext` containing the parsed packet.
    /// `p` is the mutable reference to the `ManagementTag` struct to be populated.
    /// `_` is an unused `Option<usize>`.
    ///
    /// This function returns a `Result` containing the parsed `ManagementTag` struct.
    /// If the length of the tagged parameter is invalid, or if the type of the tag is
    /// not recognized, this function returns an error.
    ///
    /// The `TagData` field is populated as follows:
    ///
    /// - If the tag is an SSID tag, the `TagData` field is populated with a `String`
    ///   containing the SSID.
    /// - Otherwise, the `TagData` field is populated with `UNKNOWN`.
    pub fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _: Option<usize>) -> Result<()> {
        
        // ieee802_mnt_tags_map
        p._type = reader.read8()?;
        p._type_str = ieee802_mnt_tags_mapper(p._type).leak();
        packet.build_backward(reader, 1, format!("Tag Number: {} ({})", p._type_str, p._type));
        if !reader.has() {
            bail!("");
        }
        p.len = packet.build_format(reader, Reader::_read8, None, "Tag length: {}")?;
        
        if reader.left() < p.len as usize {
            bail!("");
        }
        match p._type {
            0 => {
                let _read = |reader: &Reader| {
                    reader.read_string(p.len as usize)
                };
                p.data = TagData::SSID(packet.build_format(reader, _read, None, "SSID: {}")?);
            }
            _ => {
                let data = reader.slice(p.len as usize).to_vec();
                p.data = TagData::UNKNOWN(data);
            }
        }
        Ok(())
    }
}



#[derive(Default, Packet)]
pub struct Management;

impl Display for Management {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("IEEE 802.11 Wireless Management")
    }
}

impl Management {
    fn resolve_tag(reader: &Reader, packet: &PacketContext<Self>) {
        loop {
            if !reader.has() {
                break;
            }
            if let Err(_) = packet.build_packet(reader, ManagementTag::create, None, None) {
                break;
            }
        }
    }
    pub fn create(reader: &Reader, sup: &IEE80211) -> Result<PacketContext<Self>> {
        let packet: PacketContext<Self> = Frame::create_packet();
        match sup.sub_type {
            0 => {
                packet.build_format(reader, Reader::_read16_ne, None, "Beacon Interval: {}")?;
                packet.build_format(reader, Reader::_read16_ne, None, "Capability Information: {}")?;
                Management::resolve_tag(reader, &packet);
            }
            1 => {
                packet.build_format(reader, Reader::_read16_ne, None, "Capability Information: {}")?;
                packet.build_format(reader, Reader::_read16_ne, None, "Status Code: {}")?;
                packet.build_format(reader, Reader::_read16_ne, None, "Association ID: {}")?;
                Management::resolve_tag(reader, &packet);
            }
            4 => {
                Management::resolve_tag(reader, &packet);
            }
            5 => {
                packet.build_format(reader, Reader::_read64_ne, None, "Timestamp: {}")?;
                packet.build_format(reader, Reader::_read16_ne, None, "Beacon Interval: {}")?;
                packet.build_format(reader, Reader::_read16_ne, None, "Capability Information: {}")?;
                Management::resolve_tag(reader, &packet);
            }
            8 => {
                packet.build_format(reader, Reader::_read64_ne, None, "Timestamp: {}")?;
                packet.build_format(reader, Reader::_read16_ne, None, "Beacon Interval: {}")?;
                packet.build_format(reader, Reader::_read16_ne, None, "Capability Information: {}")?;
                Management::resolve_tag(reader, &packet);
            }
            11 => {
                let auth_method = match reader.read16(false)? {
                    1 => "WEP Shared Key",
                    _ => "Open System",
                };
                packet.build_backward(reader, 1, format!("Authentication Algorithm: {}", auth_method));
                packet.build_format(reader, Reader::_read16_ne, None, "Authentication SEQ: {}")?;
                let status_code = match reader.read16(false)? {
                    1 => "Unspecified failures",
                    _ => "Successful",
                };
                packet.build_backward(reader, 1, format!("Status Code: {}", status_code));
            }
            13 => {
                let category_code = reader.read8()?;
                packet.build_backward(reader, 1, format!("Category code: {} ({})", ieee802_mnt_cat_mapper(category_code), category_code));
                // 
            }
            
            _ => {},
        };
        //
        // drop(p);
        Ok(packet)
    }
}
