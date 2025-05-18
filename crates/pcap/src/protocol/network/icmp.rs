use anyhow::Result;
use crate::{cache::intern, common::{enum_def::Protocol, io::Reader, Frame, ProtocolElement}, constants::icmpv6_type_mapper, field_back_format, read_field_format};

fn _type(itype: u8, code: u8) -> String {
    let _t = itype;
    let def = format!("type:{}", _t);
    match _t {
        0 => "Echo reply".into(),
        3 => match code {
            0 => "Destination network unreachableunknow".into(),
            1 => "Destination host unreachable".into(),
            2 => "Destination protocol unreachable".into(),
            3 => "Destination port unreachable".into(),
            4 => "Fragmentation required, and DF flag set".into(),
            5 => "Source route failed".into(),
            6 => "Destination network unknown".into(),
            7 => "Destination host unknown".into(),
            8 => "Source host isolated".into(),
            9 => "Network administratively prohibited".into(),
            10 => "Host administratively prohibited".into(),
            11 => "Network unreachable for ToS".into(),
            12 => "Host unreachable for ToS".into(),
            13 => "Communication administratively prohibited".into(),
            14 => "Host Precedence Violation".into(),
            15 => "Precedence cutoff in effect".into(),
            _ => def,
        },
        4 => "Source quench".into(),
        5 => match code {
            0 => "Redirect datagrams for the Network".into(),
            1 => "Redirect datagrams for the Host".into(),
            2 => "Redirect datagrams for the Type of Service and Network".into(),
            3 => "Redirect datagrams for the Type of Service and Host".into(),
            _ => def,
        },
        8 => "Echo request".into(),
        9 => "Router Advertisement".into(),
        10 => "Router discovery/selection/solicitation".into(),
        11 => match code {
            0 => "TTL expired in transit".into(),
            1 => "Fragment reassembly time exceeded".into(),
            _ => def,
        },
        12 => match code {
            0 => "pointer indicates the error".into(),
            _ => def,
        },
        13 => "timestamp message".into(),
        14 => "timestamp reply message".into(),
        15 => "Information Request".into(),
        16 => "Information Reply".into(),
        43 => match code {
            0 => "No Error".into(),
            1 => "Malformed Query".into(),
            2 => "No Such Interface".into(),
            3 => "No Such Table Entry".into(),
            4 => "Multiple Interfaces Satisfy Query".into(),
            _ => def,
        },
        _ => def,
    }
}

fn type_desc(itype: u8, code: u8) -> String {
    format!("Type: {} ({})", code, _type(itype, code))
}
fn type6_desc(itype: u8, code: u8) -> String {
    format!("Type: {} ({})", code, icmpv6_type_mapper(itype as u16))
}

pub struct V4Visitor {

}

impl V4Visitor {
    
    pub fn parse(_: &mut Frame, reader: &mut Reader) -> Result<(NString, ProtocolElement)> {
        let mut fe = ProtocolElement::new(Protocol::ICMP);
        let mut list = vec![];
        let icpm_type = reader.read8()?;
        let code = reader.read8()?;
        field_back_format!(list, reader, 2, type_desc(icpm_type, code));
        field_back_format!(list, reader, 2, format!("Code {}", code));
        read_field_format!(list, reader, reader.read16(true)?, "Checksum: {:#06x}");
        match icpm_type {
            0x00 | 0x08 => {
                read_field_format!(list, reader, reader.read16(true)?, "Identifier: {}");
                read_field_format!(list, reader, reader.read16(true)?, "Sequence Number: {}");
            }
            _ => {}
        }
        fe.element.title = intern("Internet Control Message Protocol".into());
        fe.element.children = Some(list);
        Ok(("none", fe))
    }

    pub fn parse_v6(_: &mut Frame, reader: &mut Reader) -> Result<(NString, ProtocolElement)> {
        let mut fe = ProtocolElement::new(Protocol::ICMP);
        let mut list = vec![];
        let icpm_type = reader.read8()?;
        let code = reader.read8()?;
        field_back_format!(list, reader, 2, type6_desc(icpm_type, code));
        field_back_format!(list, reader, 2, format!("Code {}", code));
        read_field_format!(list, reader, reader.read16(true)?, "Checksum: {:#06x}");
        fe.element.title = intern("Internet Control Message Protocol v6".into());
        fe.element.children = Some(list);
        Ok(("none", fe))
        
    }
}