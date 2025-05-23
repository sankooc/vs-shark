use std::{fmt::Formatter, net::Ipv4Addr};

use anyhow::Result;
use pcap_derive::{Packet, Visitor3};

use crate::common::io::Reader;
use crate::{
    common::base::{Frame, PacketContext},
    common::{io::AReader, MacAddress, Ref2, FIELDSTATUS},
    constants::{arp_hardware_type_mapper, dhcp_option_type_mapper, dhcp_type_mapper},
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
    client_address: Option<Ipv4Addr>,
    your_address: Option<Ipv4Addr>,
    next_server_address: Option<Ipv4Addr>,
    relay_address: Option<Ipv4Addr>,
    mac_address: Option<MacAddress>,
    options: Vec<Ref2<DHCPOption>>,
}

impl std::fmt::Display for DHCP {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_fmt(format_args!("Dynamic Host Configuration Protocol ({})", self._type()))
    }
}
impl crate::common::base::InfoPacket for DHCP {
    fn info(&self) -> String {
        format!("{} - Transaction ID {:#010x}", self._type(), self.transaction_id)
    }

    fn status(&self) -> FIELDSTATUS {
        FIELDSTATUS::INFO
    }
}
impl DHCP {
    fn _type(&self) -> &'static str {
        DHCP::dhcp_type(self._type)
    }
    fn op(&self) -> String {
        format!("Message type: {} ({})", self._type, self.op)
    }
    fn hardware_type(&self) -> &'static str {
        arp_hardware_type_mapper(self.hardware_type as u16)
    }
    fn hardware_type_desc(&self) -> String {
        format!("Hardware type: {} ({:#04x})", self.hardware_type(), self.hardware_type)
    }

    fn dhcp_type(code: u8) -> &'static str {
        dhcp_type_mapper(code)
    }
    fn dhcp_message_type_desc(code: u8) -> String {
        format!("DHCP: {} ({})", DHCP::dhcp_type(code), code)
    }
}
//https://www.rfc-editor.org/rfc/rfc1497.txt
#[derive(Default, Packet)]
struct DHCPOption {
    code: u8,
    len: u8,
    extension: DHCPExtention,
}
impl DHCPOption {
    fn _type(&self) -> &'static str {
        dhcp_option_type_mapper(self.code)
    }
}

#[derive(Default)]
pub enum DHCPExtention {
    DEFAULT(Vec<u8>),
    MESSAGETYPE(u8),
    PAD,
    END,
    #[default]
    NONE,
}
impl std::fmt::Display for DHCPOption {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        let code = self.code;
        fmt.write_fmt(format_args!("OPTION ({}) {}", code, self._type()))
    }
}
#[derive(Visitor3)]
pub struct DHCPVisitor;

impl DHCPVisitor {
    fn visit2(&self, reader: &Reader) -> Result<(ProtocolData, &'static str)> {
        let packet: PacketContext<DHCP> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        p.op = packet.build_lazy(reader, Reader::_read8, Some("dhcp.op"), DHCP::op)?;
        p.hardware_type = packet.build_lazy(reader, Reader::_read8, Some("dhcp.hardware.type"), DHCP::hardware_type_desc)?;

        p.hardware_len = packet.build_format(reader, Reader::_read8, Some("dhcp.hardware.address.len"), "Hardware Address Len: {}")?;
        p.hops = packet.build_format(reader, Reader::_read8, Some("dhcp.prorocol.len"), "Protocol size: {}")?;
        p.transaction_id = packet.build_format(reader, Reader::_read32_be, Some("dhcp.transaction.id"), "Transaction ID: {}")?;
        p.sec = packet.build_format(reader, Reader::_read16_be, Some("dhcp.second.elasped"), "Seconds elapsed: {}")?;
        p.flag = reader.read16(false)?;
        p.client_address = packet.build_format(reader, Reader::_read_ipv4, Some("dhcp.client.address"), "Client Address: {}").ok();
        p.your_address = packet.build_format(reader, Reader::_read_ipv4, Some("dhcp.your.address"), "Youre Address: {}").ok();
        p.next_server_address = packet.build_format(reader, Reader::_read_ipv4, Some("dhcp.next.server.address"), "Next Server Address: {}").ok();
        p.relay_address = packet.build_format(reader, Reader::_read_ipv4, Some("dhcp.relay.address"), "Relay Address: {}").ok();
        p.mac_address = packet.build_format(reader, Reader::_read_mac, Some("dhcp.mac.address"), "Mac Address: {}").ok();
        reader._move(10); //padding
        reader._move(64); //sname
        reader._move(128); //file
        reader.read32(false)?; // magic cookie
        loop {
            let option_packet: PacketContext<DHCPOption> = Frame::create_packet();
            let _option = option_packet.get();
            let mut m_option = _option.borrow_mut();
            m_option.code = reader.read8()?;

            match m_option.code {
                0 => m_option.extension = DHCPExtention::PAD,
                0xff => m_option.extension = DHCPExtention::END,
                53 => {
                    let len = option_packet.build_format(reader, Reader::_read8, Some("dhcp.option.len"), "Length: {}")?;
                    m_option.len = len;
                    p._type = option_packet.build_fn(reader, Reader::_read8, Some("dhcp.type"), DHCP::dhcp_message_type_desc)?;
                    m_option.extension = DHCPExtention::MESSAGETYPE(p._type);
                }
                _ => {
                    let len = option_packet.build_format(reader, Reader::_read8, Some("dhcp.option.len"), "Length: {}")?;
                    m_option.len = len;
                    m_option.extension = DHCPExtention::DEFAULT(reader.slice(len as usize).to_vec())
                }
            }
            drop(m_option);
            let option = _option.borrow();
            p.options.push(option_packet._clone_obj());
            if option.code == 0xff {
                break;
            }
        }
        drop(p);
        Ok((ProtocolData::DHCP(packet), "none"))
    }
}
