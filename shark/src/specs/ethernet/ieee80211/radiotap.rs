use std::fmt::Display;

use anyhow::{bail, Result};
use pcap_derive::Packet2;
use crate::common::base::{BitFlag, BitType, FlagData, Frame, PacketBuilder, PacketContext, PacketOpt};

use crate::common::io::{AReader, Reader};
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Kind {
    TSFT,
    Flags,
    Rate,
    Channel,
    FHSS,
    AntennaSignal,
    AntennaNoise,
    LockQuality,
    TxAttenuation,
    TxAttenuationDb,
    TxPower,
    Antenna,
    AntennaSignalDb,
    AntennaNoiseDb,
    RxFlags,
    TxFlags,
    RTSRetries,
    DataRetries,
    XChannel,
    MCS,
    AMPDUStatus,
    VHT,
    Timestamp,
    VendorNamespace(u16),
    HeInformation,
    HeMuInfomation,
    PSDU,
    LSIG,
    TLV,
    RadiotapNamespace,
    S1G,
    US1G,
    EHT,
}


impl Kind {
    pub fn new(value: u8) -> Result<Kind> {
        Ok(match value {
            0 => Kind::TSFT,
            1 => Kind::Flags,
            2 => Kind::Rate,
            3 => Kind::Channel,
            4 => Kind::FHSS,
            5 => Kind::AntennaSignal,
            6 => Kind::AntennaNoise,
            7 => Kind::LockQuality,
            8 => Kind::TxAttenuation,
            9 => Kind::TxAttenuationDb,
            10 => Kind::TxPower,
            11 => Kind::Antenna,
            12 => Kind::AntennaSignalDb,
            13 => Kind::AntennaNoiseDb,
            14 => Kind::RxFlags,
            15 => Kind::TxFlags,
            16 => Kind::RTSRetries,
            17 => Kind::DataRetries,
            18 => Kind::XChannel,
            19 => Kind::MCS,
            20 => Kind::AMPDUStatus,
            21 => Kind::VHT,
            22 => Kind::Timestamp,
            23 => Kind::HeInformation,
            24 => Kind::HeMuInfomation,
            26 => Kind::PSDU,
            27 => Kind::LSIG,
            28 => Kind::TLV,
            29 => Kind::RadiotapNamespace,
            32 => Kind::S1G,
            33 => Kind::US1G,
            34 => Kind::EHT,
            _ => {
                bail!("");
            }
        })
    }

    /// Returns the align value for the field.
    pub fn align(self) -> u16 {
        match self {
            Kind::TSFT | Kind::Timestamp => 8,
            Kind::XChannel | Kind::AMPDUStatus | Kind::TLV => 4,
            Kind::Channel
            | Kind::FHSS
            | Kind::LockQuality
            | Kind::TxAttenuation
            | Kind::TxAttenuationDb
            | Kind::RxFlags
            | Kind::TxFlags
            | Kind::VHT
            | Kind::HeInformation
            | Kind::HeMuInfomation
            | Kind::LSIG
            | Kind::VendorNamespace(_) => 2,
            _ => 1,
        }
    }

    /// Returns the size of the field.
    pub fn size(self) -> usize {
        match self {
            Kind::VHT | Kind::Timestamp => 12,
            Kind::TSFT | Kind::AMPDUStatus | Kind::XChannel => 8,
            Kind::VendorNamespace(_) => 6,
            Kind::Channel => 4,
            Kind::MCS => 3,
            Kind::FHSS
            | Kind::LockQuality
            | Kind::TxAttenuation
            | Kind::TxAttenuationDb
            | Kind::RxFlags
            | Kind::TxFlags => 2,
            _ => 1,
        }
    }
}

#[derive(Default, Packet2)]
pub struct MCS {
    known: u8,
    flag: u8,
    index: u8,
}
impl Display for MCS {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("MCS information")
    }
}
impl MCS {
    fn known(&self) -> Option<PacketContext<BitFlag<u8>>> {
        BitFlag::make::<MCSKnown>(self.known)
    }
    fn flag(&self) -> Option<PacketContext<BitFlag<u8>>> {
        BitFlag::make::<MCSFlag>(self.flag)
    }
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _count: Option<usize>) -> Result<()> {
        p.known = packet.build_packet_lazy(reader, Reader::_read8, None, MCS::known)?;
        p.flag = packet.build_packet_lazy(reader, Reader::_read8, None, MCS::flag)?;
        p.index = packet.build_format(reader, Reader::_read8, None, "MCS index: {}")?;
        Ok(())
    }
}

#[derive(Default, Packet2)]
pub struct Channel {
    frequency: u16,
    flag: u16,
}
impl Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("Channel")
    }
}
impl Channel {
    fn flag(&self) -> Option<PacketContext<BitFlag<u16>>> {
        BitFlag::make::<ChannelFlag>(self.flag)
    }
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _count: Option<usize>) -> Result<()> {
        p.frequency = packet.build_format(reader, Reader::_read16_ne, None, "Channel frequency: {}")?;
        p.flag = packet.build_packet_lazy(reader, Reader::_read16_ne, None, Channel::flag)?;
        Ok(())
    }
}

#[derive(Default, Packet2)]
pub struct VHT {
    known: u16,
    flags: u8,
    bandwidth: u8,
}
impl Display for VHT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("VHT")
    }
}
impl VHT {
    fn known(&self) -> Option<PacketContext<BitFlag<u16>>> {
        BitFlag::make::<VHTKnown>(self.known)
    }
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _count: Option<usize>) -> Result<()> {
        
        // let known = reader.read16(false)?;
        // let flags = reader.read8()?;
        // let bandwidth = reader.read8()?;
        p.known = packet.build_packet_lazy(reader, Reader::_read16_ne, None, VHT::known)?;
        // p.flags = packet.build_packet_lazy(reader, Reader::_read8, None, VHT::flags)?;
        p.flags = reader.read8()?;
        p.bandwidth = reader.read8()? & 0x1f;
        let band_txt = super::cons::get_vht_bandwidth(p.bandwidth);
        packet.build_backward(reader, 1, band_txt.into());
        
        // let mut mcs_nss:[u8; 4] = [0; 4];
        let mcs_nss = reader.read32(false)?;
        let coding = reader.read8()?;
        let group_id = reader.read8()?;
        let partial_aid = reader.read16(false)?;


        Ok(())
    }
}

#[derive(Default, Packet2)]
pub struct TimeStamp {
    ts: u64,
    accuracy: u16,
    unit_position: u8,
    flags: u8
}
impl Display for TimeStamp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("Timestamp")
    }
}
impl TimeStamp {
    // fn flags(&self) -> Option<PacketContext<BitFlag<u16>>> {
    //     BitFlag::make::<AMPDUFlag>(self.flags)
    // }
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _count: Option<usize>) -> Result<()> {
        p.ts = reader.read64(false)?;
        p.accuracy = reader.read16(false)?;
        p.unit_position = reader.read8()?;
        p.flags = reader.read8()?;
        // packet.build_format(reader, Reader::_read32_ne, None, "A-MPDU reference number: {}")?;
        // packet.build_format(reader, Reader::_read16_ne, None, "A-MPDU flags number: {}")?;
        // reader.read16(false)?; // crc + reserved 
        // p.reference = packet.build_format(reader, Reader::_read16_ne, None, "A-MPDU reference number: {}")?;
        // p.flags = packet.build_packet_lazy(reader, Reader::_read16_ne, None, AMPDU::flags)?;
        // reader.read16(false)?;
        Ok(())
    }
}



#[derive(Default, Packet2)]
pub struct AMPDU {
    reference: u16,
    flags: u16,
}
impl Display for AMPDU {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("A-MPDU status")
    }
}
impl AMPDU {
    fn flags(&self) -> Option<PacketContext<BitFlag<u16>>> {
        BitFlag::make::<AMPDUFlag>(self.flags)
    }
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _count: Option<usize>) -> Result<()> {
        // packet.build_format(reader, Reader::_read32_ne, None, "A-MPDU reference number: {}")?;
        // packet.build_format(reader, Reader::_read16_ne, None, "A-MPDU flags number: {}")?;
        // reader.read16(false)?; // crc + reserved 
        p.reference = packet.build_format(reader, Reader::_read16_ne, None, "A-MPDU reference number: {}")?;
        p.flags = packet.build_packet_lazy(reader, Reader::_read16_ne, None, AMPDU::flags)?;
        reader.read16(false)?;
        Ok(())
    }
}

pub struct MCSKnown;

impl FlagData<u8> for MCSKnown {
    fn bits(inx: usize) -> Option<(u8, BitType<u8>)> {
        match inx {
            0 => {
                Some((0x01, BitType::ABSENT("Bandwidth Present", "Bandwidth Absent")))
            }
            1 => {
                Some((0x02, BitType::ABSENT("MCS index Present", "MCS index Absent")))
            }
            2 => {
                Some((0x04, BitType::ABSENT("Guard interval Present","Guard interval Absent")))
            }
            3 => {
                Some((0x08, BitType::ABSENT("HT format Present","HT format Absent")))
            }
            4 => {
                Some((0x10, BitType::ABSENT("FEC type Present","FEC type Absent")))
            }
            5 => {
                Some((0x20, BitType::ABSENT("STBC known Present","STBC known Absent")))
            }
            6 => {
                Some((0x40, BitType::ABSENT("Ness known Present","Ness known Absent")))
            }
            7 => {
                Some((0x80, BitType::ABSENT("Ness data Present","Ness data Absent")))
            }
            _ => None
        }
    }
    
    // fn to_desc(_:usize, buffer: &mut String, word: &str, status: bool) {
    //     buffer.push_str(word);
    //     if status {
    //         buffer.push_str(": Present");
    //     } else {
    //         buffer.push_str(": Absent");

    //     }
    // }
    
    fn summary(title: &mut String, value: u8) {
        title.push_str(format!("Known MCS information: {:#04x}", value).as_str());
    }
    
    fn summary_ext(title: &mut String, desc: &str, status: bool) {
        if status {
            title.push_str(", ");
            title.push_str(desc);
        }
    }
}

pub struct MCSFlag;
impl FlagData<u8> for MCSFlag {
    fn bits(inx: usize) -> Option<(u8, BitType<u8>)> {
        match inx {
            0 => {
                Some((0x03, BitType::ONEoF(vec![(0x00, "bandwidth: 20"), (0x01, "bandwidth: 40"), (0x02, "bandwidth: 20L"), (0x03, "bandwidth: 20U")])))
            }
            1 => {
                Some((0x04, BitType::ONEoF(vec![(0x00, "guard interval: long GI"), (0x04, "guard interval: short GI")])))
            }
            2 => {
                Some((0x08, BitType::ONEoF(vec![(0x00, "HT format: mixed"), (0x08, "HT format: greenfield")])))
            }
            3 => {
                Some((0x10, BitType::ONEoF(vec![(0x00, "FEC: BCC"), (0x10, "FEC: LDPC")])))
            }
            _ => None
        }
    }
    
    fn summary(title: &mut String, value: u8) {
        title.push_str(format!("Known Flag: {:#04x}", value).as_str());
    }
    
    fn summary_ext(_: &mut String, _: &str, _: bool) {
    }
}


pub struct Flags;

impl FlagData<u8> for Flags {
    fn bits(inx: usize) -> Option<(u8, BitType<u8>)> {
        match inx {
            0 => {
                Some((0x01, BitType::ABSENT("CFP: True", "CFP: False")))
            }
            1 => {
                Some((0x02, BitType::ABSENT("preamble: long", "preamble: short")))
            }
            2 => {
                Some((0x04, BitType::ABSENT("WEP: True", "WEP: False")))
            }
            3 => {
                Some((0x08, BitType::ABSENT("Fragmentation: True", "Fragmentation: False")))
            }
            4 => {
                Some((0x10, BitType::ABSENT("FCS at end: True","FCS at end: False")))
            }
            5 => {
                Some((0x20, BitType::ABSENT("Data Pad: True", "Data Pad: False")))
            }
            6 => {
                Some((0x40, BitType::ABSENT("Bad FCS: True", "Bad FCS: False")))
            }
            7 => {
                Some((0x80, BitType::ABSENT("Short GI: True", "Short GI: False")))
            }
            _ => None
        }
    }
    
    fn summary(title: &mut String, value: u8) {
        title.push_str(format!("Flags: {:#04x}", value).as_str());
    }
    
    fn summary_ext(_: &mut String, _: &str, _: bool) {
    }
}

pub struct ChannelFlag;

impl FlagData<u16> for ChannelFlag {
    fn bits(inx: usize) -> Option<(u16, BitType<u16>)> {
        match inx {
            0 => {
                Some((0x0001, BitType::ABSENT("700MHz spectrum: True", "700MHz spectrum: False")))
            }
            1 => {
                Some((0x0002, BitType::ABSENT("800MHz spectrum: True", "800MHz spectrum: False")))
            }
            2 => {
                Some((0x0004, BitType::ABSENT("900MHz spectrum: True", "900MHz spectrum: False")))
            }
            4 => {
                Some((0x0010, BitType::ABSENT("Turbo: True","Turbo: False")))
            }
            5 => {
                Some((0x0020, BitType::ABSENT("Complementary Code Keying (CCK): True", "Complementary Code Keying (CCK): False")))
            }
            6 => {
                Some((0x0040, BitType::ABSENT("Orthogonal Frequency-Division Multiplexing (OFDM): True", "Orthogonal Frequency-Division Multiplexing (OFDM): False")))
            }
            7 => {
                Some((0x0080, BitType::ABSENT("2 GHz spectrum: True", "2 GHz spectrum: False")))
            }
            8 => {
                Some((0x0100, BitType::ABSENT("5 GHz spectrum: True", "5 GHz spectrum: False")))
            }
            9 => {
                Some((0x0200, BitType::ABSENT("passive: True", "passive: False")))
            }
            10 => {
                Some((0x0400, BitType::ABSENT("Dynamic CCK-OFDM: True", "Dynamic CCK-OFDM: False")))
            }
            11 => {
                Some((0x0800, BitType::ABSENT("Gaussian Frequency Shift Keying (GFSK): True", "Gaussian Frequency Shift Keying (GFSK): False")))
            }
            12 => {
                Some((0x1000, BitType::ABSENT("GSM(900MHz): True", "GSM(900MHz): False")))
            }
            13 => {
                Some((0x2000, BitType::ABSENT("Static Turbo: True", "Static Turbo: False")))
            }
            14 => {
                Some((0x4000, BitType::ABSENT("Half Rate Channel (10MHz Channel Width): True", "Half Rate Channel (10MHz Channel Width): False")))
            }
            15 => {
                Some((0x8000, BitType::ABSENT("Quarter Rate Channel (5MHz Channel Width): True", "Quarter Rate Channel (5MHz Channel Width): False")))
            }
            _ => None
        }
    }
    
    fn summary(title: &mut String, value: u16) {
        title.push_str(format!("Channel Flag: {:#06x}", value).as_str());
    }
    
    fn summary_ext(_: &mut String, _: &str, _: bool) {
    }
}

pub struct TXFlags;

impl FlagData<u16> for TXFlags {
    fn bits(inx: usize) -> Option<(u16, BitType<u16>)> {
        match inx {
            0 => {
                Some((0x0001, BitType::ABSENT("Transmission failed due to excessive retries: True", "Transmission failed due to excessive retries: False")))
            }
            1 => {
                Some((0x0002, BitType::ABSENT("Transmission used CTS-to-self protection: True", "Transmission used CTS-to-self protection: False")))
            }
            2 => {
                Some((0x0004, BitType::ABSENT("Transmission used RTS/CTS handshake: True", "Transmission used RTS/CTS handshake: False")))
            }
            4 => {
                Some((0x0008, BitType::ABSENT("Transmission shall not expect an ACK frame: True","Transmission shall not expect an ACK frame: False")))
            }
            5 => {
                Some((0x0010, BitType::ABSENT("Transmission includes a pre-configured sequence number: True", "Transmission includes a pre-configured sequence number: False")))
            }
            6 => {
                Some((0x0020, BitType::ABSENT("Transmission should not be reordered: True", "Transmission should not be reordered: False")))
            }
            _ => None
        }
    }
    
    fn summary(title: &mut String, value: u16) {
        title.push_str(format!("TX Flags: {:#06x}", value).as_str());
    }
    
    fn summary_ext(_: &mut String, _: &str, _: bool) {
    }
}

pub struct AMPDUFlag;

impl FlagData<u16> for AMPDUFlag {
    fn bits(inx: usize) -> Option<(u16, BitType<u16>)> {
        match inx {
            0 => {
                Some((0x0001, BitType::ABSENT("Driver reports 0-length subframes in this A-MPDU: True", "Driver reports 0-length subframes in this A-MPDU: False")))
            }
            1 => {
                Some((0x0002, BitType::ABSENT("This is a 0-length subframe: True", "This is a 0-length subframe: False")))
            }
            2 => {
                Some((0x0004, BitType::ABSENT("Last subframe of this A-MPDU is known: True", "Last subframe of this A-MPDU is known: False")))
            }
            4 => {
                Some((0x0008, BitType::ABSENT("This is the last subframe of this A-MPDU: True","This is the last subframe of this A-MPDU: False")))
            }
            5 => {
                Some((0x0010, BitType::ABSENT("Delimiter CRC error on this subframe: True", "Delimiter CRC error on this subframe: False")))
            }
            6 => {
                Some((0x0040, BitType::ABSENT("EOF on this subframe: True", "EOF on this subframe: False")))
            }
            7 => {
                Some((0x0080, BitType::ABSENT("EOF of this A-MPDU is known: True", "EOF of this A-MPDU is known: False")))
            }
            _ => None
        }
    }
    
    fn summary(title: &mut String, value: u16) {
        title.push_str(format!("A-MPDU Flags: {:#06x}", value).as_str());
    }
    
    fn summary_ext(_: &mut String, _: &str, _: bool) {
    }
}

pub struct VHTKnown;

impl FlagData<u16> for VHTKnown {
    fn bits(inx: usize) -> Option<(u16, BitType<u16>)> {
        match inx {
            0 => {
                Some((0x0001, BitType::ABSENT("STBC: True", "STBC: False")))
            }
            1 => {
                Some((0x0002, BitType::ABSENT("TXOP_PS_NOT_ALLOWED: True", "TXOP_PS_NOT_ALLOWED: False")))
            }
            2 => {
                Some((0x0004, BitType::ABSENT("Guard interval: True", "Guard interval: False")))
            }
            4 => {
                Some((0x0008, BitType::ABSENT("Short GI NSYM disambiguation: True","Short GI NSYM disambiguation: False")))
            }
            5 => {
                Some((0x0010, BitType::ABSENT("LDPC extra OFDM symbol: True", "LDPC extra OFDM symbol: False")))
            }
            6 => {
                Some((0x0020, BitType::ABSENT("Beamformed known/applicable: True", "Beamformed known/applicable: False")))
            }
            7 => {
                Some((0x0040, BitType::ABSENT("Bandwidth known: True", "Bandwidth known: False")))
            }
            8 => {
                Some((0x0080, BitType::ABSENT("Group ID known: True", "Group ID known: False")))
            }
            9 => {
                Some((0x0100, BitType::ABSENT("Partial AID known/applicable: True", "Partial AID known/applicable: False")))
            }
            _ => None
        }
    }
    
    fn summary(title: &mut String, value: u16) {
        title.push_str(format!("VHT Known: {:#06x}", value).as_str());
    }
    
    fn summary_ext(_: &mut String, _: &str, _: bool) {
    }
}

