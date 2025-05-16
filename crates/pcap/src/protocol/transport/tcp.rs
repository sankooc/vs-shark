use crate::{
    cache::{intern, intern_ip6},
    common::{concept::Field, enum_def::Protocol, io::Reader, Context, Frame},
    constants::ip_protocol_type_mapper,
    protocol::ip4_mapper,
    read_field_format, read_field_format_fn,
};
use anyhow::Result;

pub struct Visitor {}
pub fn t_protocol(protocol_type: u8) -> String {
    format!("Protocol: {} ({:#06x})", ip_protocol_type_mapper(protocol_type as u16), protocol_type)
}
impl Visitor {
    pub fn parse(_: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        let _start = reader.left() as u16;
        let source_port = reader.read16(true)?;
        let target_port = reader.read16(true)?;
        let sequence = reader.read32(true)?;
        let ack = reader.read32(true)?;
        let head = reader.read16(true)?;
        let window = reader.read16(true)?;
        let crc = reader.read16(true)?;
        let urgent = reader.read16(true)?;
        let len = (head >> 12) & 0x0f;
        if len > 5 {
            let skip = (len - 5) * 4;
            reader.forward(skip as usize);
        }
        let mut left_size = reader.left() as u16;
        let iplen = frame.iplen;
        if iplen > 0 {
            if _start > iplen {
                left_size = iplen + left_size - _start;
            }
        }
        todo!()
    }
    pub fn detail(field: &mut Field, _: &Context, _: &Frame, reader: &mut Reader) -> Result<Protocol> {
        let _start = reader.left() as u16;
        let source_port = reader.read16(true)?;
        let target_port = reader.read16(true)?;
        let sequence = reader.read32(true)?;
        let ack = reader.read32(true)?;
        let head = reader.read16(true)?;
        let window = reader.read16(true)?;
        let crc = reader.read16(true)?;
        let urgent = reader.read16(true)?;
        let len = (head >> 12) & 0x0f;
        if len > 5 {
            let skip = (len - 5) * 4;
            reader.forward(skip as usize);
        }
        let left_size = reader.left() as u16;
        // let ip_packet = frame.get_ip();

        // let head = reader.
        // let ad = reader.
        // reader.
            Ok(Protocol::None)
    }
}
// let _start = reader.left() as u16;
// let packet: PacketContext<TCP> = Frame::create_packet();
// let mut p = packet.get().borrow_mut();
// p.frame_refer = frame.refer.clone();
// p.source_port = packet.build_lazy(reader, Reader::_read16_be, Some("tcp.source.port"), Description::source_port)?;
// p.target_port = packet.build_lazy(reader, Reader::_read16_be, Some("tcp.target.port"), Description::target_port)?;
// p.sequence = packet.build_lazy(reader, Reader::_read32_be, Some("tcp.sequence"), TCP::sequence_desc)?;
// p.acknowledge = packet.build_lazy(reader, Reader::_read32_be, Some("tcp.acknowledge"), TCP::acknowledge_desc)?;
// // let head = packet.build_lazy(reader, Reader::_read16_be, Some("tcp.head.len"), TCP::len_desc)?;
// let head= packet.build_packet_lazy(reader, Reader::_read16_be, None, TCP::flag)?;
// p.flag = head;
// p.window = packet.build_format(reader, Reader::_read16_be, Some("tcp.window.size"), "Window: {}")?;
// p.crc = packet.build_format(reader, Reader::_read16_be, None, "Checksum: {}")?;
// p.urgent = packet.build_format(reader, Reader::_read16_be, None, "Urgent Pointer: {}")?;
// p.set_head(head);
// let len = p.len;
// if len > 5 {
//     let options = packet.build_packet(reader, TCPVisitor::read_options, Some(len as usize), Some("Options".into()))?;
//     p.options = Some(options);
// }
// let left_size = reader.left() as u16;
// p.payload_len = left_size;
// let ip_packet = frame.get_ip();
// let unwap = ip_packet.deref().borrow();
// let _total = unwap.payload_len();
// if let Some(total) = _total {
//     if _start > total {
//         p.payload_len = total + left_size - _start;
//     }
// }
// if left_size > 0 {
//     packet._build(reader, reader.cursor(), p.payload_len.into(), Some(("tcp.playload.len", p.payload_len.to_string().leak())), format!("TCP payload ({} bytes)", left_size));
// }
// packet.build_packet_no_position_lazy(TCP::segments);
// frame.add_tcp(packet._clone_obj());
// drop(p);
// Ok((ProtocolData::TCP(packet), "none"))
