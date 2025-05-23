use std::net::Ipv4Addr;

use crate::{
    common::{concept::Field, core::Context, enum_def::{IpField, Protocol}, io::Reader, Frame},
    constants::ip_protocol_type_mapper,
    field_back_format, field_back_format_fn,
    protocol::ip4_mapper,
    read_field_format, read_field_format_fn,
};
use anyhow::{bail, Result};

pub fn head_lenstr(head_len: u8) -> String {
    format!(".... {:04b} = Header Length: {} bytes ({})", head_len, head_len * 4, head_len)
}
pub fn t_protocol(protocol_type: u8) -> String {
    format!("Protocol: {} ({:#06x})", ip_protocol_type_mapper(protocol_type as u16), protocol_type)
}

pub struct Visitor {}

impl Visitor {
    pub fn info(_: &Context, frame: &Frame) -> Option<String> {
        if let IpField::IPv4(s, t) = &frame.ip_field {
            return Some(format!("Internet Protocol Version 4, Src: {}, Dst: {}", s, t));
        }
        None
    }
    pub fn parse(_: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        let _start = reader.left();
        let head = reader.read8()?;
        let head_len = head & 0x0f;
        reader.read8()?; // tos
        let total_len = reader.read16(true)?;
        reader.read16(true)?; //
        reader.read16(true)?; // flag TODO
        reader.read8()?; // ttl
        let protocol_type = reader.read8()?; // protocol
        reader.read16(true)?; // checksum
        let _data = reader.slice(8, true)?;
        let source = Ipv4Addr::from(<[u8; 4]>::try_from(&_data[..4])?);
        let target = Ipv4Addr::from(<[u8; 4]>::try_from(&_data[4..])?);
        frame.ip_field = IpField::IPv4(source, target);
        
        let ext = head_len - 5;
        if ext > 0 {
            reader.slice((ext * 4) as usize, true)?;
        }
        let _stop = reader.left();
        if head_len == 0 {
            //  payload_len is None;
        } else {
            if total_len < (_start - _stop) as u16 {
                return Ok(Protocol::None);
            }
            frame.iplen = total_len - (_start - _stop) as u16;
        }

        Ok(ip4_mapper(protocol_type))
    }
    pub fn detail(field: &mut Field, _: &Context, _: &Frame, reader: &mut Reader) -> Result<Protocol> {
        let _start = reader.left();
        let mut list = vec![];
        let head = reader.read8()?;
        let head_len = head & 0x0f;
        field_back_format!(list, reader, 1, "0100 .... = Version: 4".into());
        field_back_format_fn!(list, reader, 1, head_lenstr(head_len));
        reader.read8()?; // tos
        let total_len = read_field_format!(list, reader, reader.read16(true)?, "Total Length: {}");
        read_field_format!(list, reader, reader.read16(true)?, "Identification: {:#06x}");
        reader.read16(true)?; // flag TODO
        read_field_format!(list, reader, reader.read8()?, "Time To Live: {}");
        let protocol_type = read_field_format_fn!(list, reader, reader.read8()?, t_protocol);
        read_field_format!(list, reader, reader.read16(true)?, "Header Checksum: {}");

        let source = read_field_format!(list, reader, reader.read_ip4()?, "Source Address: {}");
        let target = read_field_format!(list, reader, reader.read_ip4()?, "Destination Address: {}");
        let ext = head_len - 5;
        if ext > 0 {
            reader.forward((ext * 4) as usize);
        }
        let _stop = reader.left();
        if total_len == 0 {
            //  payload_len is None;
        } else {
            if total_len < (_start - _stop) as u16 {
                bail!("error_len");
            }
            // let payload_len = Some(total_len - (_start - _stop) as u16);
        }

        field.summary = format!("Internet Protocol Version 4, Src: {}, Dst: {}", source, target);
        field.children = Some(list);
        Ok(ip4_mapper(protocol_type))
    }
}
