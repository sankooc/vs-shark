use crate::{
    cache::intern,
    common::{
        concept::Field,
        connection::{TCPStat, TcpFlagField},
        enum_def::{PacketStatus, Protocol, TCPDetail},
        io::Reader,
        Context, Frame,
    },
    constants::ip_protocol_type_mapper,
    field_back_format, read_field_format,
};
use anyhow::Result;

pub struct Visitor {}
pub fn t_protocol(protocol_type: u8) -> String {
    format!("Protocol: {} ({:#06x})", ip_protocol_type_mapper(protocol_type as u16), protocol_type)
}
impl Visitor {
    pub fn parse(ctx: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        let start = reader.left() as u16;
        let source_port = reader.read16(true)?;
        let target_port = reader.read16(true)?;
        let sequence = reader.read32(true)?;
        let ack = reader.read32(true)?;
        let state = TcpFlagField::from(reader.read16(true)?);
        let _window = reader.read16(true)?;
        let crc = reader.read16(true)?;
        let _urgent = reader.read16(true)?;
        let len = state.head_len();
        if len > 5 {
            let skip = (len - 5) * 4;
            reader.forward(skip as usize);
        }
        let mut left_size = reader.left() as u16;
        let iplen = frame.iplen;
        if iplen > 0 {
            if start > iplen {
                left_size = iplen + left_size - start;
            }
        }
        let flags = state.list_str();
        // let _data_range = _start.._start + left_size;
        let tcp_state = TCPStat::new(sequence, ack, crc, state, left_size);
        let rely = ctx.get_connect(frame.source, source_port, frame.target, target_port, tcp_state);

        frame.info.status = match &rely.status {
            TCPDetail::NEXT | TCPDetail::KEEPALIVE => PacketStatus::NORNAL,
            _ => PacketStatus::ERROR,
        };
        frame.info.info = intern(format!("{} -> {} {} Seq={} Win={} Len={} ", source_port, target_port, flags, rely.seq, _window, left_size));
        frame.tcp_info = Some(rely);
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
        list.push(Field::label(intern(format!("[Next Sequence Number: {}    (relative sequence number)]", info.next)), 0, 0));
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
        field.summary = intern(format!("Transmission Control Protocol, Src Port: {}, Dst Port: {}, Len: {}", source_port, target_port, info.len));
        field.children = Some(list);
        Ok(Protocol::None)
    }
}
