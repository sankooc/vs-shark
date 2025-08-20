// Copyright (c) 2025 sankooc
// 
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

//https://www.radiotap.org/
use crate::{
    add_field_backstep, add_field_format, add_field_label, add_sub_field,
    common::{concept::Field, core::Context, enum_def::Protocol, io::Reader, Frame},
};
use anyhow::Result;

// Radiotap presence flags
const RADIOTAP_TSFT: u32 = 1 << 0;
const RADIOTAP_FLAGS: u32 = 1 << 1;
const RADIOTAP_RATE: u32 = 1 << 2;
const RADIOTAP_CHANNEL: u32 = 1 << 3;
const RADIOTAP_FHSS: u32 = 1 << 4;
const RADIOTAP_DBM_ANTSIGNAL: u32 = 1 << 5;
const RADIOTAP_DBM_ANTNOISE: u32 = 1 << 6;
const RADIOTAP_LOCK_QUALITY: u32 = 1 << 7;
const RADIOTAP_TX_ATTENUATION: u32 = 1 << 8;
const RADIOTAP_DB_TX_ATTENUATION: u32 = 1 << 9;
const RADIOTAP_DBM_TX_POWER: u32 = 1 << 10;
const RADIOTAP_ANTENNA: u32 = 1 << 11;
const RADIOTAP_DB_ANTSIGNAL: u32 = 1 << 12;
const RADIOTAP_DB_ANTNOISE: u32 = 1 << 13;
const RADIOTAP_RX_FLAGS: u32 = 1 << 14;
const RADIOTAP_TX_FLAGS: u32 = 1 << 15;
const RADIOTAP_DATA_RETRIES: u32 = 1 << 17;
const RADIOTAP_CHANNEL_PLUS: u32 = 1 << 18;
const RADIOTAP_MCS: u32 = 1 << 19;

const RADIOTAP_AMPDU_STATUS: u32 = 1 << 20;
const RADIOTAP_VHT: u32 = 1 << 21;
const RADIOTAP_TIMESTAMP: u32 = 1 << 22;
const RADIOTAP_HE: u32 = 1 << 23;

const RADIOTAP_HE_MU: u32 = 1 << 24;
const RADIOTAP_0_LENGTH_PSDU: u32 = 1 << 26;
const RADIOTAP_L_SIG: u32 = 1 << 27;

// const RADIOTAP_TLV: u32 = 1 << 28;
// const RADIOTAP_RADIO_TAP_NS_NEXT: u32 = 1 << 29;
// const RADIOTAP_VENDOR_TAP_NS_NEXT: u32 = 1 << 30;
// const RADIOTAP_EXT: u32 = 1 << 31;

fn get_masked_value<T>(value: T, index: usize) -> (bool, String)
where
    T: std::ops::BitAnd<Output = T> + std::ops::Shr<usize, Output = T> + From<u8> + Copy + std::cmp::PartialEq,
{
    let bits = std::mem::size_of::<T>() * 8;
    let one: T = T::from(1);
    let bit = (value >> index) & one == one;
    let mut result = String::with_capacity(bits * 2);

    for i in (0..bits).rev() {
        if i == index {
            result.push(if bit { '1' } else { '0' });
        } else {
            result.push('.');
        }

        if i % 4 == 0 && i != 0 {
            result.push(' ');
        }
    }
    (bit, result)
}
fn get_mask<T>(head: T, index: usize, key: &str, values: &(&str, &str)) -> String
where
    T: std::ops::BitAnd<Output = T> + std::ops::Shr<usize, Output = T> + From<u8> + Copy + std::cmp::PartialEq,
{
    let (bit, mask) = get_masked_value(head, index);
    if bit {
        format!("{} = {}: {}", mask, key, values.0)
    } else {
        format!("{} = {}: {}", mask, key, values.1)
    }
}

fn field_present_flag(head: u32, field: &mut Field) -> Result<u32> {
    field.summary = format!("Present Flags: 0x{:08x}", head);
    let vs = ("Present", "Absent");
    add_field_label!(field, get_mask(head, 0, "TSFT", &vs));
    add_field_label!(field, get_mask(head, 1, "Flags", &vs));
    add_field_label!(field, get_mask(head, 2, "Rate", &vs));
    add_field_label!(field, get_mask(head, 3, "Channel", &vs));

    add_field_label!(field, get_mask(head, 4, "FHSS", &vs));
    add_field_label!(field, get_mask(head, 5, "dBm Antenna Signal", &vs));
    add_field_label!(field, get_mask(head, 6, "dBm Antenna Noise", &vs));
    add_field_label!(field, get_mask(head, 7, "Lock Quality", &vs));

    add_field_label!(field, get_mask(head, 8, "TX Attenuation", &vs));
    add_field_label!(field, get_mask(head, 9, "dB TX Attenuation", &vs));
    add_field_label!(field, get_mask(head, 10, "dBm TX Power", &vs));
    add_field_label!(field, get_mask(head, 11, "Antenna", &vs));

    add_field_label!(field, get_mask(head, 12, "dB Antenna Signal", &vs));
    add_field_label!(field, get_mask(head, 13, "dB Antenna Noise", &vs));
    add_field_label!(field, get_mask(head, 14, "RX Flags", &vs));
    add_field_label!(field, get_mask(head, 15, "TX Flags", &vs));

    add_field_label!(field, get_mask(head, 17, "data retries", &vs));
    add_field_label!(field, get_mask(head, 18, "Channel+", &vs));
    add_field_label!(field, get_mask(head, 19, "MCS information", &vs));

    add_field_label!(field, get_mask(head, 20, "A-MPDU Status", &vs));
    add_field_label!(field, get_mask(head, 21, "VHT information", &vs));
    add_field_label!(field, get_mask(head, 22, "frame timestamp", &vs));
    add_field_label!(field, get_mask(head, 23, "HE information", &vs));

    add_field_label!(field, get_mask(head, 24, "HE-MU information", &vs));
    add_field_label!(field, get_mask(head, 25, "Reserved", &vs));
    add_field_label!(field, get_mask(head, 26, "0 Length PSDU", &vs));
    add_field_label!(field, get_mask(head, 27, "L-SIG", &vs));

    add_field_label!(field, get_mask(head, 28, "TLVs", &vs));
    add_field_label!(field, get_mask(head, 29, "RadioTap NS next", &vs));
    add_field_label!(field, get_mask(head, 30, "Vendor NS next", &vs));
    add_field_label!(field, get_mask(head, 31, "Ext", &vs));
    Ok(head)
}

fn field_flag_flag(head: u8, field: &mut Field) -> Result<u8> {
    field.summary = format!("Flags: 0x{:04x}", head);
    let vs = ("True", "False");
    add_field_label!(field, get_mask(head, 0, "CFP", &vs));
    add_field_label!(field, get_mask(head, 1, "Preamble", &vs));
    add_field_label!(field, get_mask(head, 2, "WEP", &vs));
    add_field_label!(field, get_mask(head, 3, "Fragmentation", &vs));
    add_field_label!(field, get_mask(head, 4, "FCS", &vs));
    add_field_label!(field, get_mask(head, 5, "Data Pad", &vs));
    add_field_label!(field, get_mask(head, 6, "Bad FCS", &vs));
    add_field_label!(field, get_mask(head, 7, "Short GI", &vs));

    Ok(head)
}

fn field_channel_flag(head: u16, field: &mut Field) -> Result<u16> {
    field.summary = format!("Channel Flags: 0x{:04x}", head);
    let vs = ("True", "False");
    add_field_label!(field, get_mask(head, 0, "700 MHz spectrum", &vs));
    add_field_label!(field, get_mask(head, 1, "800 MHz spectrum", &vs));
    add_field_label!(field, get_mask(head, 2, "900 MHz spectrum", &vs));

    add_field_label!(field, get_mask(head, 4, "Turbo", &vs));
    add_field_label!(field, get_mask(head, 5, "Complimentary CCK", &vs));
    add_field_label!(field, get_mask(head, 6, "Orthogonal Frequency Division Multiplexing (OFDM)", &vs));
    add_field_label!(field, get_mask(head, 7, "2 GHz spectrum", &vs));

    add_field_label!(field, get_mask(head, 8, "5 GHz spectrum", &vs));
    add_field_label!(field, get_mask(head, 9, "Passive", &vs));
    add_field_label!(field, get_mask(head, 10, "Dynamic CCK-OFDM", &vs));
    add_field_label!(field, get_mask(head, 11, "Gaussian Preamble Shift Keying (GPSK)", &vs));

    add_field_label!(field, get_mask(head, 12, "GSM (900 MHz)", &vs));
    add_field_label!(field, get_mask(head, 13, "Static Turbo", &vs));
    add_field_label!(field, get_mask(head, 14, "Half Rate Channel (10MHz)", &vs));
    add_field_label!(field, get_mask(head, 15, "Quarter Rate Channel (5MHz Channel Width)", &vs));

    Ok(head)
}
pub struct Visitor;

impl Visitor {
    pub fn info(_: &Context, _: &Frame) -> Option<String> {
        Some("Radiotap Header v0".into())
    }

    pub fn parse(_: &mut Context, _frame: &mut Frame, _reader: &mut Reader) -> Result<Protocol> {
        let _header_revision = _reader.read8()?;
        let _header_pad = _reader.read8()?;
        let header_length = _reader.read16(false)?;
        _reader.forward(header_length as usize - 4);
        // let mut reader = _reader.slice_as_reader(header_length as usize - 4)?;

        // let present_flags = reader.read32(false)?;
        Ok(Protocol::IEEE802_11)
    }

    pub fn detail(field: &mut Field, _: &Context, _: &Frame, _reader: &mut Reader) -> Result<Protocol> {
        // Parse Radiotap header
        let _header_revision = add_field_format!(field, _reader, _reader.read8()?, "Header Revision: {}");
        let _header_pad = add_field_format!(field, _reader, _reader.read8()?, "Header Pad: {}");
        let header_length = add_field_format!(field, _reader, _reader.read16(false)?, "Header Length: {} bytes");
        let mut reader = _reader.slice_as_reader(header_length as usize - 4)?;
        
        field.summary = format!("Radiotap Header v0, Length {}", header_length);

        let present_flags = add_sub_field!(field, &mut reader, reader.read32(false)?, field_present_flag);

        // Process Radiotap data fields based on present flags
        if present_flags & RADIOTAP_TSFT != 0 {
            add_field_format!(field, reader, reader.read64(false)?, "TSFT: {} μs");
        }

        if present_flags & RADIOTAP_FLAGS != 0 {
            add_sub_field!(field, &mut reader, reader.read8()?, field_flag_flag);
            reader.forward(1);
        }

        if present_flags & RADIOTAP_RATE != 0 {
            let rate = reader.read8()?;
            add_field_backstep!(field, reader, 1, format!("Rate: {:.1} Mbps", rate as f32 / 2.0));
        }

        if present_flags & RADIOTAP_CHANNEL != 0 {
            add_field_format!(field, reader, reader.read16(false)?, "Channel frequency: {}");
            add_sub_field!(field, &mut reader, reader.read16(false)?, field_channel_flag);
        }

        if present_flags & RADIOTAP_FHSS != 0 {
            let fhss = reader.read16(false)?;
            add_field_backstep!(field, reader, 2, format!("FHSS: Hop Set {}, Pattern {}", fhss & 0x00FF, (fhss >> 8) & 0x00FF));
        }

        if present_flags & RADIOTAP_DBM_ANTSIGNAL != 0 {
            let signal = reader.read8()?;
            add_field_backstep!(field, reader, 1, format!("Antenna Signal: {} dBm", signal as i8));
        }

        if present_flags & RADIOTAP_DBM_ANTNOISE != 0 {
            let noise = reader.read8()?;
            add_field_backstep!(field, reader, 1, format!("Antenna Noise: {} dBm", noise as i8)); 
        }

        //https://www.radiotap.org/fields/Lock%20quality.html
        if present_flags & RADIOTAP_LOCK_QUALITY != 0 {
            add_field_format!(field, reader, reader.read16(false)?, "Lock Quality: {}");
        }

        //https://www.radiotap.org/fields/TX%20attenuation.html
        if present_flags & RADIOTAP_TX_ATTENUATION != 0 {
            let tx_attenuation = reader.read16(false)?;
            add_field_backstep!(field, reader, 2, format!("TX Attenuation: {}", tx_attenuation));
        }

        //https://www.radiotap.org/fields/dB%20TX%20attenuation.html
        if present_flags & RADIOTAP_DB_TX_ATTENUATION != 0 {
            let tx_attenuation = reader.read16(false)?;
            add_field_backstep!(field, reader, 2, format!("dB TX Attenuation: {}", tx_attenuation));
        }

        //https://www.radiotap.org/fields/dBm%20TX%20power.html
        if present_flags & RADIOTAP_DBM_TX_POWER != 0 {
            let tx_power = reader.read8()?;
            add_field_backstep!(field, reader, 1, format!("TX Power: {} dBm", tx_power as i8));
        }

        if present_flags & RADIOTAP_ANTENNA != 0 {
            add_field_format!(field, reader, reader.read8()?, "Antenna: {}");
        }

        if present_flags & RADIOTAP_DB_ANTSIGNAL != 0 {
            let signal = reader.read8()?;
            add_field_backstep!(field, reader, 1, format!("Antenna Signal: {} dBm", signal as i8));
        }

        if present_flags & RADIOTAP_DB_ANTNOISE != 0 {
            let noise = reader.read8()?;
            add_field_backstep!(field, reader, 1, format!("Antenna Noise: {} dBm", noise as i8));
        }

        //https://www.radiotap.org/fields/RX%20flags.html
        if present_flags & RADIOTAP_RX_FLAGS != 0 {
            add_field_format!(field, reader, reader.read16(false)?, "RX Flags: {}"); //TODO
        }

        //https://www.radiotap.org/fields/RX%20flags.html
        if present_flags & RADIOTAP_TX_FLAGS != 0 {
            add_field_format!(field, reader, reader.read16(false)?, "TX Flags: {}"); //TODO
        }

        //https://www.radiotap.org/fields/data%20retries.html
        if present_flags & RADIOTAP_DATA_RETRIES != 0 {
            add_field_format!(field, reader, reader.read8()?, "Data Retries: {}"); //TODO
        }
        //https://www.radiotap.org/fields/XChannel.html
        if present_flags & RADIOTAP_CHANNEL_PLUS != 0 {
            reader.forward(8);
            // add_field_format!(field, reader, reader.read8()?, "Channel Plus: {}"); //TODO
        }
        //https://www.radiotap.org/fields/MCS.html
        if present_flags & RADIOTAP_MCS != 0 {
            reader.forward(3);
            // add_field_format!(field, reader, reader.read8()?, "MCS: {}"); //TODO
        }

        //https://www.radiotap.org/fields/A-MPDU%20status.html
        if present_flags & RADIOTAP_AMPDU_STATUS != 0 {
            reader.forward(8);
            // add_field_format!(field, reader, reader.read8()?, "AMPDU Status: {}"); //TODO
        }
        //https://www.radiotap.org/fields/VHT.html
        if present_flags & RADIOTAP_VHT != 0 {
            reader.forward(12);
            // add_field_format!(field, reader, reader.read8()?, "VHT: {}"); //TODO
        }
        //https://www.radiotap.org/fields/timestamp.html
        if present_flags & RADIOTAP_TIMESTAMP != 0 {
            reader.forward(12);
            // add_field_format!(field, reader, reader.read8()?, "Timestamp: {}"); //TODO
        }
        //https://www.radiotap.org/fields/HE.html
        if present_flags & RADIOTAP_HE != 0 {
            reader.forward(12);
            // add_field_format!(field, reader, reader.read8()?, "HE: {}"); //TODO
        }
        if present_flags & RADIOTAP_HE_MU != 0 {
            reader.forward(12);
            // add_field_format!(field, reader, reader.read8()?, "HE MU: {}"); //TODO
        }
        if present_flags & RADIOTAP_0_LENGTH_PSDU != 0 {
            reader.forward(1);
            // add_field_format!(field, reader, reader.read8()?, "0 Length PSDU: {}"); //TODO
        }
        if present_flags & RADIOTAP_L_SIG != 0 {
            reader.forward(4);
            // add_field_format!(field, reader, reader.read8()?, "L-SIG: {}"); //TODO
        }
        //https://www.radiotap.org/fields/TLV.html
        // if present_flags & RADIOTAP_TLV != 0 {
        //     // add_field_format!(field, reader, reader.read8()?, "TLV: {}"); //TODO
        // }
        // if present_flags & RADIOTAP_RADIO_TAP_NS_NEXT != 0 {
        //     add_field_format!(field, reader, reader.read8()?, "RadioTap NS Next: {}"); //TODO
        // }

        Ok(Protocol::IEEE802_11)
    }
}
