use crate::{
    common::{
        concept::Field,
        connection::{TCPStat, TcpFlagField},
        core::Context,
        enum_def::{PacketStatus, Protocol, TCPDetail},
        io::Reader,
        Frame,
    }, constants::ip_protocol_type_mapper, field_back_format, field_forward_format, read_field_format
};
use anyhow::Result;

pub struct Visitor;
pub fn t_protocol(protocol_type: u8) -> String {
    format!("Protocol: {} ({:#06x})", ip_protocol_type_mapper(protocol_type as u16), protocol_type)
}
impl Visitor {
    pub fn info(_: &Context, frame: &Frame) -> Option<String> {
        if let Some(stat) = &frame.tcp_info {
            let mut source_port = 0;
            let mut target_port = 0;
            if let Some(ports) = &frame.ports {
                source_port = ports.0;
                target_port = ports.1;
            }
            let state = TcpFlagField::from(stat.flag_bit);
            return Some(format!("{} -> {} {} Seq={} Len={} ", source_port, target_port, state.list_str(), stat.seq, stat.len));
        }
        None
    }
    pub fn parse(ctx: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        let index = frame.info.index;
        let start = reader.left();
        let source_port = reader.read16(true)?;
        let target_port = reader.read16(true)?;
        let sequence = reader.read32(true)?;
        let ack = reader.read32(true)?;
        let flag_bit = reader.read16(true)?;
        let state = TcpFlagField::from(flag_bit);
        let _window = reader.read16(true)?;
        let crc = reader.read16(true)?;
        let _urgent = reader.read16(true)?;
        let len = state.head_len();
        if len > 5 {
            let skip = (len - 5) * 4;
            reader.forward(skip as usize);
        }
        let mut left_size = reader.left();
        let iplen = frame.iplen as usize;
        if iplen > 0 {
            if start > iplen {
                left_size = iplen + left_size - start;
            }
        }
        let ds = reader.ds();
        let range = reader.cursor..reader.cursor + left_size;
        let tcp_state = TCPStat::new(index, sequence, ack, crc, state, left_size as u16);
        
        if let Ok(mut tcp_info) = ctx.get_connect(frame, source_port, target_port, tcp_state, ds, range) {
            tcp_info.flag_bit = flag_bit;
            frame.info.status = match &tcp_info.status {
                TCPDetail::NEXT | TCPDetail::KEEPALIVE => PacketStatus::NORNAL,
                _ => PacketStatus::ERROR,
            };
            frame.ports = Some((source_port, target_port));
            let next = tcp_info.next_protocol;
            frame.tcp_info = Some(tcp_info);
            return Ok(next)
        }
        Ok(Protocol::None)
    }
    pub fn detail(field: &mut Field, _: &Context, frame: &Frame, reader: &mut Reader) -> Result<Protocol> {
        let mut list = Vec::new();

        let source_port = read_field_format!(list, reader, reader.read16(true)?, "Source Port: {}");
        let target_port = read_field_format!(list, reader, reader.read16(true)?, "Destination Port: {}");

        let info = frame.tcp_info.as_ref().unwrap();

        let sequence = reader.read32(true)?;
        field_back_format!(list, reader, 4, format!("Sequence Number: {}    (relative sequence number)", info.seq));
        field_back_format!(list, reader, 4, format!("Sequence Number (raw): {}", sequence));
        list.push(Field::label(format!("[Next Sequence Number: {}    (relative sequence number)]", info.next), 0, 0));
        let ack = reader.read32(true)?;
        field_back_format!(list, reader, 4, format!("Acknowledgment Number: {}    (relative ack number)", info.ack));
        field_back_format!(list, reader, 4, format!("Acknowledgment Number (raw): {}", ack));
        // let _state = TcpFlagField::from(reader.read16(true)?);
        let state = read_field_format!(list, reader, TcpFlagField::from(reader.read16(true)?), "{}");
        read_field_format!(list, reader, reader.read16(true)?, "Window: {}");
        read_field_format!(list, reader, reader.read16(true)?, "Checksum: {:#06x} [unverified]");
        read_field_format!(list, reader, reader.read16(true)?, "Urgent Pointer: {}");
        let len = state.head_len();
        if len > 5 {
            let skip = (len - 5) * 4;
            reader.forward(skip as usize);
        }
        let payload_len = info.len as usize;
        field_forward_format!(list, reader, payload_len, format!("TCP payload ({} bytes)", payload_len));
        
        field.summary = format!("Transmission Control Protocol, Src Port: {}, Dst Port: {}, Len: {}", source_port, target_port, info.len);
        field.children = Some(list);
        Ok(info.next_protocol)
    }
}
