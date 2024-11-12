use anyhow::{bail, Result};
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