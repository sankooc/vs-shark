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
    
    fn to_desc(_:usize, buffer: &mut String, word: &str, status: bool) {
        buffer.push_str(word);
        if status {
            buffer.push_str(": Present");
        } else {
            buffer.push_str(": Absent");

        }
    }
    
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
    
    fn to_desc(_:usize, buffer: &mut String, word: &str, _: bool) {
        buffer.push_str(word);
    }
    
    fn summary(title: &mut String, value: u8) {
        title.push_str(format!("Known Flag: {:#04x}", value).as_str());
    }
    
    fn summary_ext(title: &mut String, desc: &str, status: bool) {
        if status {
            title.push_str(", ");
            title.push_str(desc);
        }
    }
}