// Copyright (c) 2025 sankooc
//
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

use serde::Serialize;

use crate::common::{
    enum_def::{FileType, Protocol},
    io::Reader,
    LinkType,
};

pub fn ethernet_protocol(link_type: LinkType) -> Protocol {
    match link_type {
        0 => Protocol::Loopback,
        127 => Protocol::RADIOTAP,
        113 => Protocol::SSL,
        _ => Protocol::ETHERNET,
    }
}

#[derive(Default, Serialize)]
pub enum FileMetadata {
    #[default]
    None,
    Pcap(Pcap),
    PcapNg(PcapNg),
}

impl FileMetadata {
    pub fn file_type(&self) -> FileType {
        match self {
            FileMetadata::None => FileType::NONE,
            FileMetadata::Pcap(_) => FileType::PCAP,
            FileMetadata::PcapNg(_) => FileType::PCAPNG,
        }
    }
    pub fn link_type(&self) -> Protocol {
        Protocol::None
    }
}

impl FileMetadata {
    pub fn init_pcap(major: u16, minor: u16, snaplen: u32, link_type: LinkType) -> Self {
        FileMetadata::Pcap(Pcap { major, minor, snaplen, link_type })
    }
    pub fn init_pcapng() -> Self {
        FileMetadata::PcapNg(PcapNg::default())
    }
}

#[derive(Default, Serialize)]
pub struct PcapNg {
    pub captrue: Option<CaptureInterface>,
    pub interfaces: Vec<InterfaceDescription>,
}

impl PcapNg {
    pub fn _protocol(&self, link_type: LinkType) -> Protocol {
        if let 0 = link_type {
            Protocol::Loopback
        } else {
            ethernet_protocol(link_type)
        }
    }
    pub fn protocol(&self, interface_id: usize) -> Protocol {
        if let Some(interface) = self.interfaces.get(interface_id) {
            self._protocol(interface.link_type)
        } else {
            Protocol::ETHERNET
        }
    }
}


impl PcapNg {
    pub fn add_interface(&mut self, interface: InterfaceDescription) {
        self.interfaces.push(interface);
    }
}

pub trait OptionParser {
    fn parse_option(&mut self, option_code: u16, content: std::borrow::Cow<'_, str>);
}

#[derive(Debug, Default, Serialize)]
pub struct CaptureInterface {
    hardware: Option<String>,
    os: Option<String>,
    application: Option<String>,
}

impl OptionParser for CaptureInterface {
    fn parse_option(&mut self, option_code: u16, content: std::borrow::Cow<'_, str>) {
        match option_code {
            2 => {
                self.hardware = Some(content.to_string().trim().to_string());
            }
            3 => {
                self.os = Some(content.to_string());
            }
            4 => {
                self.application = Some(content.to_string());
            }
            _ => {}
        }
    }
} 

#[derive(Debug, Serialize)]
pub struct Pcap {
    pub major: u16,
    pub minor: u16,
    pub snaplen: u32,
    pub link_type: LinkType,
}

impl Pcap {
    pub fn protocol(&self, reader: &mut Reader) -> Protocol {
        if self.link_type == 0 {
            let _head = reader.slice(16, false).unwrap();
            if _head[0] == 0 && _head[5] == 6 {
                let lat = &_head[14..16];
                let _flag = u16::from_be_bytes(lat.try_into().unwrap());
                return match _flag {
                    0x0806 | 0x0800 | 0x86dd | 0x8864 => Protocol::SSL,
                    _ => Protocol::ETHERNET,
                };
            }
            Protocol::ETHERNET
        } else {
            ethernet_protocol(self.link_type)
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct InterfaceDescription {
    pub link_type: LinkType,
    pub name: Option<String>,
    pub description: Option<String>,
    pub filter: Option<String>,
    pub os: Option<String>,
}

impl InterfaceDescription {
    pub fn new(link_type: LinkType) -> Self {
        Self {
            link_type,
            ..Default::default()
        }
    }
}


impl OptionParser for InterfaceDescription {
    fn parse_option(&mut self, option_code: u16, content: std::borrow::Cow<'_, str>) {
        match option_code {
            2 => {
                self.name = Some(content.to_string());
            }
            3 => {
                self.description = Some(content.to_string());
            }
            11 => {
                self.filter = Some(content.to_string());
            }
            12 => {
                self.os = Some(content.to_string());
            }
            _ => {}
        }
    }
}