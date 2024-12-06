use std::fmt::Display;

use anyhow::Result;
use cons::get_he;
use pcap_derive::{Packet2, Visitor3, NINFO};
use radiotap::Kind;
use crate::common::base::{BitFlag, PacketContext, PacketOpt};
use crate::common::io::AReader;
use crate::
    common::io::Reader
;
use crate::specs::ProtocolData;
pub mod radiotap;
pub mod i802;
mod cons;
mod mnt;
mod control;
mod data;


#[derive(Default, Packet2, NINFO)]
pub struct Radiotap {
    revision: u8,
    pad: u8,
    length: u16,
    flags: u8,
    tx_flags: u16,
}

impl Display for Radiotap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!("Radiotap Header v{}, Length {}", self.revision, self.length))
    }
}
impl Radiotap {
    fn flags(&self) -> Option<PacketContext<BitFlag<u8>>> {
        BitFlag::make::<radiotap::Flags>(self.flags)
    }
    fn tx_flags(&self) -> Option<PacketContext<BitFlag<u16>>> {
        BitFlag::make::<radiotap::TXFlags>(self.tx_flags)
    }
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _: Option<PacketOpt>) -> Result<()> {
        let start = reader.cursor();
        p.revision = packet.build_format(reader, Reader::_read8, Some("radiotap.header.revision"), "Header revision: {}")?;
        p.pad = packet.build_format(reader, Reader::_read8, Some("radiotap.header.pad"), "Header pad: {}")?;
        p.length = packet.build_format(reader, Reader::_read16_ne, Some("radiotap.header.len"), "Header length: {}")?;
        let finish = start + p.length as usize;
        let mut vendor_namespace = false;
        let is_bit_set = |val: u32, index: u8| val & (1 << index) > 0;
        let mut present = Vec::new();
        loop {
            let val = reader.read32(false)?;
            if !vendor_namespace {
                for inx in 0..29 {
                    if is_bit_set(val, inx) {
                        match Kind::new(inx) {
                            Ok(kind) => {
                                present.push(kind);
                            },
                            _ => {}
                        }
                    }
                }
            }
            if is_bit_set(val, 29) {
                //present_count = 0;
                //
                vendor_namespace = false;
            } else if is_bit_set(val, 30) {
                vendor_namespace = true;
            } else {
            }
            if !is_bit_set(val, 31) {
                break;
            }
        }


        for pre in present.iter() {
            if reader.cursor() >= finish {
                break;
            }
            let _align = pre.align() as usize;
            if _align > 1 {
                let _current = reader.cursor() - 1;
                let nc = (_current | (_align - 1)) + 1;
                reader._set(nc);
            }
            match pre {
                Kind::TSFT => {
                    packet.build_format(reader, Reader::_read64_ne, None, "MAC timestamp: {}")?;
                },
                Kind::Flags => {
                    p.flags = packet.build_packet_lazy(reader, Reader::_read8, None, Radiotap::flags)?;
                }
                Kind::Rate => {
                    let v = reader.read8()? as f32;
                    packet.build_backward(reader, 1, format!("Data Rate: {:.01} Mb/s",v/2.0));
                }
                Kind::Channel => {
                    packet.build_packet(reader, radiotap::Channel::create, None, None)?;
                    // packet.build_format(reader, Reader::_read16_ne, None, "Channel frequency: {}")?;
                    // let c_flag = reader.read16(false)?;
                    // let mut str = format!("Channel flags: {:#06x}", c_flag);
                    // cons::get_flag_list(&mut str, c_flag);
                    // packet.build_backward(reader, 2, str);
                }
                Kind::FHSS => {
                    packet.build_format(reader, Reader::_read8, None, "FHSS hop set: {}")?;
                    packet.build_format(reader, Reader::_read8, None, "FHSS hop pattern: {}")?;
                }
                Kind::LockQuality => {
                    packet.build_format(reader, Reader::_read16_ne, None, "Lock Quality: {}")?;
                }
                Kind::TxAttenuation => {
                    packet.build_format(reader, Reader::_read16_ne, None, "TX Attenuation: {}")?;
                }
                Kind::TxAttenuationDb => {
                    packet.build_format(reader, Reader::_read16_ne, None, "dB TX Attenuation: {}")?;
                }
                Kind::TxPower => {
                    packet.build_format(reader, Reader::_read8, None, "Transmit power: {} dBm")?;
                }
                Kind::AntennaSignal => {
                    packet.build_format(reader, Reader::_read_i8, None, "Antenna signal: {} dBm")?;
                }
                Kind::AntennaNoise => {
                    packet.build_format(reader, Reader::_read_i8, None, "Antenna noise: {} dBm")?;
                }
                Kind::AntennaSignalDb => {
                    packet.build_format(reader, Reader::_read_i8, None, "Antenna signal: {} dB")?;
                }
                Kind::AntennaNoiseDb => {
                    packet.build_format(reader, Reader::_read_i8, None, "Antenna noise: {} dB")?;
                }
                Kind::Antenna => {
                    packet.build_format(reader, Reader::_read_i8, None, "Antenna: {} dBm")?;
                }
                Kind::RxFlags => {
                    let _flag = reader.read16(false)?;
                    let checked = (_flag & 0x0002) > 0;
                    if !checked {
                        packet.build_backward(reader, 2, "PLCP CRC check failed".into());
                    }
                }
                Kind::TxFlags => {
                    p.tx_flags = packet.build_packet_lazy(reader, Reader::_read16_ne, None, Radiotap::tx_flags)?;
                }
                Kind::RTSRetries => {
                    // todo OpenBSD u16
                    packet.build_format(reader, Reader::_read8, None, "RTS retries: {}")?;
                }
                Kind::DataRetries => {
                    packet.build_format(reader, Reader::_read8, None, "Data retries: {}")?;
                }
                Kind::XChannel => {
                    // need sample
                    let _flag = reader.read32(false)?;
                    let mut builder = String::from("x-channel flag");
                    let _content = cons::_get_x_channel_flag(&mut builder, _flag);
                    // add flag
                    let freq = reader.read16(false)?;
                    packet.build_backward(reader, 2, format!("x-channel freq: {}", freq));
                    let channel = reader.read8()?;
                    packet.build_backward(reader, 1, format!("x-channel channel: {}", channel));
                    let maxpower = reader.read8()?;
                    packet.build_backward(reader, 1, format!("x-channel max power: {}", maxpower));
                }
                Kind::MCS => {
                    packet.build_packet(reader, radiotap::MCS::create, None, None)?;
                }
                Kind::AMPDUStatus => {
                    packet.build_packet(reader, radiotap::AMPDU::create, None, None)?;
                }
                Kind::VHT => {
                    packet.build_packet(reader, radiotap::VHT::create, None, None)?;
                }
                Kind::Timestamp => {
                    packet.build_packet(reader, radiotap::TimeStamp::create, None, None)?;
                }
                Kind::HeInformation => {
                    for index in 1..7 {
                        let data = reader.read16(false)?;
                        let mut str = format!("HE Data {}: {:#06x}", index, data);
                        get_he(index, &mut str, data);
                        packet.build_backward(reader, 2, str);
                    }
                }
                Kind::PSDU => {
                    let _type = reader.read8()?;
                    match _type {
                        0 => {
                            packet.build_backward(reader, 1, "sounding PPDU".into());
                        }
                        1 => {
                            packet.build_backward(reader, 1, "data not captured".into());
                        }
                        _ => {}
                    }
                }
                Kind::LSIG => {
                    let _data1 = reader.read16(false)?;
                    let _data2 = reader.read16(false)?;
                }
                Kind::TLV => {
                }
                _ => {
                    let _size = pre.size();
                    reader.slice(_size);
                }
            }
        }

        reader._set(finish);
        Ok(())
    }
}


#[derive(Visitor3)]
pub struct RadiotapVisitor;
impl RadiotapVisitor {
    pub fn visit2(&self, reader: &Reader) -> Result<(ProtocolData, &'static str)> {
        let packet = Radiotap::create(reader, None)?;
        Ok((ProtocolData::Radiotap(packet), "802.11"))
    }
}

