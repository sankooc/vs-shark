use anyhow::{bail, Result};
use crate::{cache::{intern, intern_ip4}, common::{enum_def::Protocol, io::Reader, Frame, ProtocolElement}, constants::ip_protocol_type_mapper, field_back_format, field_back_format_fn, protocol::ip4_mapper, read_field_format, read_field_format_fn};


fn head_lenstr(head_len: u8) -> String {
    format!(".... {:04b} = Header Length: {} bytes ({})", head_len, head_len*4, head_len)
}
pub fn t_protocol(protocol_type: u8) -> String {
    format!("Protocol: {} ({:#06x})", ip_protocol_type_mapper(protocol_type as u16), protocol_type)
}

pub struct Visitor {

}

impl Visitor {
    
    pub fn parse( frame: &mut Frame, reader: &mut Reader) -> Result<(&'static str, ProtocolElement)> {
        let _start = reader.left();
        let mut fe = ProtocolElement::new(Protocol::IP4);
        let mut list = vec![];
        let head = reader.read8()?;
        let head_len =head & 0x0f;
        field_back_format!(list, reader, 1,"0100 .... = Version: 4".into());
        field_back_format_fn!(list, reader, 1, head_lenstr(head_len));
        reader.read8()?; // tos
        let total_len = read_field_format!(list, reader, reader.read16(true)?, "Total Length: {}");
        read_field_format!(list, reader, reader.read16(true)?, "Identification: {:#06x}");
        reader.read16(true)?;// flag TODO
        read_field_format!(list, reader, reader.read8()?, "Time To Live: {}");
        let protocol_type = read_field_format_fn!(list, reader, reader.read8()?, t_protocol);
        read_field_format!(list, reader, reader.read16(true)?, "Header Checksum: {}");
        let source = read_field_format!(list, reader, intern_ip4(reader)?, "Source Address: {}");
        let target = read_field_format!(list, reader, intern_ip4(reader)?, "Destination Address: {}");
        let ext = head_len - 5;
        if ext > 0 {
            reader.slice((ext * 4) as usize, true)?;
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
        frame.info.source = source;
        frame.info.dest = target;
        fe.element.title = intern(format!("Internet Protocol Version 4, Src: {}, Dst: {}", source, target));
        fe.element.children = Some(list);
        Ok((ip4_mapper(protocol_type), fe))
    }
}