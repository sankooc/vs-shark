use std::fmt::{Binary, Display, Formatter};
use std::ops::BitAnd;

use anyhow::Result;
use cons::get_he;
use pcap_derive::{Packet2, Visitor3, NINFO};
use radiotap::Kind;
use crate::common::base::{PacketContext, PacketOpt};
use crate::common::io::AReader;
use crate::{
    common::base::Frame,
    common::io::Reader,
};
use crate::common::base::PacketBuilder;
use crate::specs::ProtocolData;
use crate::specs::FIELDSTATUS;
pub mod radiotap;
mod cons;


#[derive(Default, Packet2, NINFO)]
pub struct Radiotap {
    revision: u8,
    pad: u8,
    length: u16,
    mcs_known: u8,
}

impl Display for Radiotap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!("Radiotap Header v{}, Length {}", self.revision, self.length))
    }
}
impl Radiotap {
    fn mcs_known(&self) -> Option<PacketContext<BitFlag<u8>>> {
        BitFlag::make::<MCSKnown>(self.mcs_known)
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

        // present.reverse();

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
                    packet.build_format(reader, Reader::_read8, None, "Flags: {}")?;
                    // reader.read8()?;
                }
                Kind::Rate => {
                    let v = reader.read8()? as f32;
                    packet.build_backward(reader, 1, format!("Data Rate: {:.01} Mb/s",v/2.0));
                }
                Kind::Channel => {
                    packet.build_format(reader, Reader::_read16_ne, None, "Channel frequency: {}")?;
                    let c_flag = reader.read16(false)?;
                    let mut str = format!("Channel flags: {:#06x}", c_flag);
                    cons::get_flag_list(&mut str, c_flag);
                    packet.build_backward(reader, 2, str);
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
                    todo!("")
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
                    let _start = reader.cursor();

                    let _val = reader.read16(false)?;
                    if cons::bit_set(_val, 0x0001) {
                        packet._build(reader,_start, _start + 2, None,"Transmission failed due to excessive retries".into());
                    }
                    if cons::bit_set(_val, 0x0002) {
                        packet._build(reader,_start, _start + 2, None,"Transmission used CTS-to-self protection".into());
                    }
                    if cons::bit_set(_val, 0x0004) {
                        packet._build(reader,_start, _start + 2, None,"Transmission used RTS/CTS handshake".into());
                    }
                    if cons::bit_set(_val, 0x0008) {
                        packet._build(reader,_start, _start + 2, None,"Transmission shall not expect an ACK frame and not retry when no ACK is received".into());
                    }
                    if cons::bit_set(_val, 0x0010) {
                        packet._build(reader,_start, _start + 2, None,"Transmission includes a pre-configured sequence number that should not be changed by the driver’s TX handlers".into());
                    }
                    if cons::bit_set(_val, 0x0020) {
                        packet._build(reader,_start, _start + 2, None,"Transmission should not be reordered relative to other frames that have this flag set".into());
                    }
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
                    p.mcs_known = packet.build_packet_lazy(reader, Reader::_read8, None, Radiotap::mcs_known)?;
                    let flag = reader.read8()?;
                    let index = reader.read8()?;
                }
                Kind::AMPDUStatus => {
                    packet.build_format(reader, Reader::_read32_ne, None, "A-MPDU reference number: {}")?;
                    packet.build_format(reader, Reader::_read16_ne, None, "A-MPDU flags number: {}")?;
                    reader.read16(false)?; // crc + reserved 
                }
                Kind::VHT => {
                    let known = reader.read16(false)?;
                    let flags = reader.read8()?;
                    let bandwidth = reader.read8()?;
                    todo!("")
                }
                Kind::Timestamp => {
                    todo!("")
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
                    let data1 = reader.read16(false)?;
                    let data2 = reader.read16(false)?;
                    todo!("exp")
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


pub struct MCSKnown;

impl FlagData<u8> for MCSKnown {
    fn bits(inx: usize) -> Option<(u8, BitType<u8>)> {
        match inx {
            0 => {
                Some((0x01, BitType::ABSENT("Bandwidth")))
            }
            1 => {
                Some((0x02, BitType::ABSENT("MCS index")))
            }
            2 => {
                Some((0x04, BitType::ABSENT("Guard interval")))
            }
            3 => {
                Some((0x08, BitType::ABSENT("HT format")))
            }
            4 => {
                Some((0x10, BitType::ABSENT("FEC type")))
            }
            5 => {
                Some((0x20, BitType::ABSENT("STBC known")))
            }
            6 => {
                Some((0x40, BitType::ABSENT("Ness known")))
            }
            7 => {
                Some((0x80, BitType::ABSENT("Ness data")))
            }
            _ => None
        }
    }
}

#[derive(Clone)]
pub enum BitType<T> {
    ABSENT(&'static str),
    ONEoF(Vec<(T, &'static str)>)
}

pub trait FlagData<T> where T:Binary + Copy + BitAnd + PartialEq<<T as BitAnd>::Output>, <T as BitAnd>::Output: PartialEq<T> {
    fn bits(inx: usize) -> Option<(T, BitType<T>)>;
}
#[derive(Default)]
pub struct BitFlag<T> where T:Binary + Copy + BitAnd + PartialEq<<T as BitAnd>::Output>, <T as BitAnd>::Output: PartialEq<T>  {
    pub value: T
}

impl<T> std::fmt::Display for BitFlag<T> where T:Binary + Copy + BitAnd + PartialEq<<T as BitAnd>::Output>, <T as BitAnd>::Output: PartialEq<T> {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_str("BitFlag")
    }
}
impl<T>  PacketBuilder for BitFlag< T> where T: Default + Binary + Copy + BitAnd + PartialEq<<T as BitAnd>::Output>, <T as BitAnd>::Output: PartialEq<T> {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    fn summary(&self) -> String {
        self.to_string()
    }
}
impl<T>  BitFlag <T> where T:Default + Binary + Copy + BitAnd + PartialEq<<T as BitAnd>::Output> + 'static, <T as BitAnd>::Output: PartialEq<T> {
    fn make<F>(value: T) -> Option<PacketContext<Self>> where F: FlagData<T> {
        let packet: PacketContext<Self> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        p.value = value;
        let n = size_of_val(&value) * 8;
        for inx in 0..n {
            if let Some(info) = F::bits(inx) {
                match &info.1 {
                    BitType::ABSENT(desc) => {
                        let mask = info.0;
                        let (line, status): (String, &str) = BitFlag::print_line(mask, value);
                        packet.build_txt(format!("{} = {}: {}", line, *desc, status));
                    },
                    BitType::ONEoF(list) => {}
                }
            }
        }
        drop(p);
        Some(packet)
    }
    fn print_line(mask: T, value: T) -> (String, &'static str) where T:Binary + Copy + BitAnd, <T as BitAnd>::Output: PartialEq<T> {
        let len = size_of_val(&mask) * 8;
        let str = format!("{:0len$b}", mask);
        let mut str2 = str.replace("0", ".");
        for cur in 1..len/4 {
            str2.insert_str(len - cur * 4, " ");
        }
        if mask & value != mask {
            return (str2.replace("1", "0"), "Absent");
        }
        (str2, "Present")
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

