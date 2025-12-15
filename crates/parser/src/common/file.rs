// Copyright (c) 2025 sankooc
//
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

use serde::Serialize;

use crate::common::{
    LinkType, enum_def::{FileType, Protocol}, io::{IO, Reader}
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
    pub fn get(&self) -> Option<Metadata> {
        match self {
            FileMetadata::None => None,
            FileMetadata::Pcap(meta) => Some(Metadata::from(meta)),
            FileMetadata::PcapNg(meta) => Some(Metadata::from(meta))
        }
    }
    // pub fn to_json(&self) -> Option<String> {
    //     match self {
    //         FileMetadata::None => None,
    //         FileMetadata::Pcap(meta) => serde_json::to_string(&Metadata::from(meta)).ok(),
    //         FileMetadata::PcapNg(meta) => serde_json::to_string(&Metadata::from(meta)).ok(),
    //     }
    // }
}

impl FileMetadata {
    pub fn init_pcap(major: u16, minor: u16, snaplen: u32, link_type: LinkType, reader: &mut Reader) -> Self {
        let protocol = Pcap::protocol(link_type, reader);
        FileMetadata::Pcap(Pcap { major, minor, snaplen, link_type, protocol })
    }
    pub fn init_pcapng() -> Self {
        FileMetadata::PcapNg(PcapNg::default())
    }
}

#[derive(Default, Serialize)]
pub struct PcapNg {
    pub major: u16,
    pub minor: u16,
    pub capture: Option<CaptureInterface>,
    pub interfaces: Vec<InterfaceDescription>,
    pub statistics: Option<FileStatistics>,
}

impl PcapNg {
    pub fn _protocol(link_type: LinkType) -> Protocol {
        if let 0 = link_type {
            Protocol::Loopback
        } else {
            ethernet_protocol(link_type)
        }
    }
    pub fn protocol(&self, interface_id: usize) -> Protocol {
        if let Some(interface) = self.interfaces.get(interface_id) {
            PcapNg::_protocol(interface.link_type)
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
    fn parse_option(&mut self, option_code: u16, content: &[u8]);
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct CaptureInterface {
    hardware: Option<String>,
    os: Option<String>,
    application: Option<String>,
}

impl OptionParser for CaptureInterface {
    fn parse_option(&mut self, option_code: u16, data: &[u8]) {
        let content: std::borrow::Cow<'_, str> = String::from_utf8_lossy(data);
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
    pub protocol: Protocol,
}

impl Pcap {
    pub fn protocol(link_type: LinkType, reader: &mut Reader) -> Protocol {
        if link_type == 0 {
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
            ethernet_protocol(link_type)
        }
    }
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct InterfaceDescription {
    pub link_type: LinkType,
    pub protocol: Protocol,
    pub name: Option<String>,
    pub description: Option<String>,
    pub filter: Option<String>,
    pub os: Option<String>,
}

impl InterfaceDescription {
    pub fn new(link_type: LinkType, protocol: Protocol) -> Self {
        Self {
            link_type,
            protocol,
            ..Default::default()
        }
    }
}


impl OptionParser for InterfaceDescription {
    fn parse_option(&mut self, option_code: u16, data: &[u8]) {
        let content: std::borrow::Cow<'_, str> = String::from_utf8_lossy(data);
        match option_code {
            2 => {
                self.name = Some(content.to_string());
            }
            3 => {
                self.description = Some(content.to_string());
            }
            11 => {
                self.filter = Some(content.trim().to_string());
            }
            12 => {
                self.os = Some(content.to_string());
            }
            _ => {}
        }
    }
}

fn parse_ts(data: &[u8]) -> u64{
    if data.len() != 8 {
        return 0;
    }
    let mut ts = IO::read32(&data[..4],false).unwrap() as u64;
    let low_ts = IO::read32(&data[4..], false).unwrap() as u64;
    ts = (ts << 32) + low_ts;
    ts
}

#[derive(Debug, Default, Serialize)]
pub struct FileStatistics {
    isb_starttime: u64,
    isb_endtime: u64,
    isb_ifrecv: u64,
    isb_ifdrop: u64,
}
impl OptionParser for FileStatistics {
    fn parse_option(&mut self, option_code: u16, data: &[u8]) {
        match option_code {
            2 => {
                self.isb_starttime = parse_ts(data);
            },
            3 => {
                self.isb_endtime = parse_ts(data);
            },
            4 => {
                if data.len() != 8 {
                    return;
                }
                self.isb_ifrecv = IO::_read64(data, false).unwrap();
            },
            5 => {
                if data.len() != 8 {
                    return;
                }
                self.isb_ifdrop = IO::_read64(data, false).unwrap();
            },

            _ => {}
        }
    }
}



#[derive(Debug, Default, Serialize)]
pub struct Metadata {
    pub major: u16,
    pub minor: u16,
    pub start: Option<String>,
    pub end: Option<String>,
    pub elapsed: Option<String>,
    pub file_type: String,
    pub capture: Option<CaptureInterface>,
    pub interfaces: Vec<InterfaceDescription>,
}

impl From<&PcapNg> for Metadata {
    fn from(pcapng: &PcapNg) -> Self {
        Metadata {
            file_type: "PCAPNG".to_string(),
            major: pcapng.major,
            minor: pcapng.minor,
            capture: pcapng.capture.clone(),
            interfaces: pcapng.interfaces.clone(),
            ..Default::default()
        }
    }
}

impl From<&Pcap> for Metadata {
    fn from(pcap: &Pcap) -> Self {
        let lt = pcap.link_type;
        let protocol = PcapNg::_protocol(lt);
        let inter = InterfaceDescription::new(lt, protocol);
        Metadata {
            file_type: "PCAP".to_string(),
            major: pcap.major,
            minor: pcap.minor,
            capture: None,
            interfaces: vec![inter],
            ..Default::default()
        }
    }
}