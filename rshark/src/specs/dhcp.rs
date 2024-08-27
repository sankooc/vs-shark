use std::fmt::Formatter;

use pcap_derive::Packet;
use anyhow::Result;

use crate::{
    common::{IPv4Address, MacAddress, Reader}, constants::arp_hardware_type_mapper, files::{Frame, Initer, PacketContext}
};

use super::ProtocolData;

#[derive(Default, Packet)]
pub struct DHCP {
    
    op: u8,
    _type: u8,
    hardware_type: u8,
    hardware_len: u8,
    hops: u8,
    transaction_id: u32,
    sec: u16,
    flag: u16,
    client_address: Option<IPv4Address>,
    your_address: Option<IPv4Address>,
    next_server_address: Option<IPv4Address>,
    relay_address: Option<IPv4Address>,
    mac_address: Option<MacAddress>,
}


impl std::fmt::Display for DHCP {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_str("Address Resolution Protocol")
    }
}
impl crate::files::InfoPacket for DHCP {
    fn info(&self) -> String {
        self.to_string()
    }
}
impl DHCP {
    fn _info(&self) -> String {
        return self.to_string()
    }
    fn _summary(&self) -> String {
        format!("Address Resolution Protocol ")
    }
    fn op(&self) -> String {
        format!("Message type: {} ({})", self._type, self.op)
    }
    fn hardware_type(&self) -> String {
        arp_hardware_type_mapper(self.hardware_type as u16)
    }
    fn hardware_type_desc(&self) -> String {
        format!("Hardware type: {} ({:#04x})", self.hardware_type(), self.hardware_type)
    }
}
//https://www.rfc-editor.org/rfc/rfc1497.txt
#[derive(Default, Packet)]
struct DHCPOption {
    code: u8,
    len: u8,
    extension: DHCPExtention
}

#[derive(Default)]
enum DHCPExtention {
    DEFAULT(Vec<u8>),
    MESSAGETYPE(u8),
    PAD,
    END,
    #[default]
    NONE,
}

impl DHCPOption {
    // fn create(reader: &Reader) -> Result<DHCPOption> {
    //     let code = reader.read8()?;
    //     if code == 0 {
    //         return Ok(DHCPOption{code, len: 0, extension: DHCPExtention::PAD});
    //     }
    //     if code == 0xff {
    //         return Ok(DHCPOption{code, len: 0, extension: DHCPExtention::END});
    //     }
    //     let len = reader.read8()?;
    //     let opt = match code {
    //         53 => {
    //             reader.read8();
    //         },
    //         _ => {
    //             let data = reader.slice(len as usize).to_vec();
    //             DHCPOption{code, len, extension: DHCPExtention::DEFAULT(data)}
    //         }
    //     }
    //     Ok(opt)
    // }
}
impl std::fmt::Display for DHCPOption {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        let code = self.code;
        // match self {
        //     DHCPExtention::END => fmt.write_str("OPTION (255) END"),
        //     DHCPExtention::PAD => fmt.write_str("OPTION (0) PAD"),
        //     DHCPExtention::DEFAULT(code, _ , _) => {
        //         fmt.write_fmt(format_args!("OPTION ({})", code))
        //     },
        // }
        fmt.write_fmt(format_args!("OPTION ({})", code))
    }
}

pub struct DHCPVisitor;

impl crate::files::Visitor for DHCPVisitor {
    fn visit(&self, frame: &Frame, reader: &Reader) -> Result<()> {
        let packet: PacketContext<DHCP> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        p.op = packet.read_with_string(reader, Reader::_read8, DHCP::op)?;
        p.hardware_type = packet.read_with_string(reader, Reader::_read8, DHCP::hardware_type_desc)?;

        p.hardware_len = packet._read_with_format_string_rs(reader, Reader::_read8, "Hardware Address Len: {}")?;
        p.hops = packet._read_with_format_string_rs(reader, Reader::_read8, "Protocol size: {}")?;
        p.transaction_id = packet._read_with_format_string_rs(reader, Reader::_read32_be, "")?;
        p.sec = packet._read_with_format_string_rs(reader, Reader::_read16_be, "")?;
        p.flag = reader.read16(false)?;
        // p.fla = packet._read_with_format_string_rs(reader, Reader::_read16_be, "")?;
        p.client_address = packet._read_with_format_string_rs(reader, Reader::_read_ipv4, "Client Address: {}").ok();
        p.your_address = packet._read_with_format_string_rs(reader, Reader::_read_ipv4, "Client Address: {}").ok();
        p.next_server_address = packet._read_with_format_string_rs(reader, Reader::_read_ipv4, "Client Address: {}").ok();
        p.relay_address = packet._read_with_format_string_rs(reader, Reader::_read_ipv4, "Client Address: {}").ok();
        p.mac_address =  packet._read_with_format_string_rs(reader, Reader::_read_mac, "Client Address: {}").ok();
        reader._move(10);//padding
        reader._move(64);//sname
        reader._move(128);//file
        reader.read32(false);// magic cookie
        // loop {
        //     let option: PacketContext<DHCPOption> = Frame::create_packet();
            
        // }
        // p.target_ip = packet._read_with_format_string_rs(reader, Reader::_read_ipv4, "Target IP address: {}").ok();
        drop(p);
        frame.add_element(ProtocolData::DHCP(packet));
        Ok(())
    }
}
