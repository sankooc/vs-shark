use crate::{
    common::{
        concept::Field,
        core::Context,
        enum_def::{Protocol, ProtocolInfoField},
        io::Reader,
        Frame,
    },
    field_back_format,
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
const RADIOTAP_RTS_RETRIES: u32 = 1 << 16;
const RADIOTAP_DATA_RETRIES: u32 = 1 << 17;
const RADIOTAP_MCS: u32 = 1 << 19;
const RADIOTAP_AMPDU_STATUS: u32 = 1 << 20;
const RADIOTAP_VHT: u32 = 1 << 21;
const RADIOTAP_TIMESTAMP: u32 = 1 << 22;

// IEEE 802.11 frame types
const IEEE80211_TYPE_MANAGEMENT: u8 = 0;
const IEEE80211_TYPE_CONTROL: u8 = 1;
const IEEE80211_TYPE_DATA: u8 = 2;

// IEEE 802.11 frame subtypes
const IEEE80211_SUBTYPE_ASSOC_REQ: u8 = 0;
const IEEE80211_SUBTYPE_ASSOC_RESP: u8 = 1;
const IEEE80211_SUBTYPE_REASSOC_REQ: u8 = 2;
const IEEE80211_SUBTYPE_REASSOC_RESP: u8 = 3;
const IEEE80211_SUBTYPE_PROBE_REQ: u8 = 4;
const IEEE80211_SUBTYPE_PROBE_RESP: u8 = 5;
const IEEE80211_SUBTYPE_BEACON: u8 = 8;
const IEEE80211_SUBTYPE_ATIM: u8 = 9;
const IEEE80211_SUBTYPE_DISASSOC: u8 = 10;
const IEEE80211_SUBTYPE_AUTH: u8 = 11;
const IEEE80211_SUBTYPE_DEAUTH: u8 = 12;
const IEEE80211_SUBTYPE_ACTION: u8 = 13;

// Helper function to get frame type name
fn get_frame_type_name(frame_type: u8) -> &'static str {
    match frame_type {
        IEEE80211_TYPE_MANAGEMENT => "Management",
        IEEE80211_TYPE_CONTROL => "Control",
        IEEE80211_TYPE_DATA => "Data",
        _ => "Unknown",
    }
}

// Helper function to get frame subtype name
fn get_frame_subtype_name(frame_type: u8, frame_subtype: u8) -> &'static str {
    match frame_type {
        IEEE80211_TYPE_MANAGEMENT => match frame_subtype {
            IEEE80211_SUBTYPE_ASSOC_REQ => "Association Request",
            IEEE80211_SUBTYPE_ASSOC_RESP => "Association Response",
            IEEE80211_SUBTYPE_REASSOC_REQ => "Reassociation Request",
            IEEE80211_SUBTYPE_REASSOC_RESP => "Reassociation Response",
            IEEE80211_SUBTYPE_PROBE_REQ => "Probe Request",
            IEEE80211_SUBTYPE_PROBE_RESP => "Probe Response",
            IEEE80211_SUBTYPE_BEACON => "Beacon",
            IEEE80211_SUBTYPE_ATIM => "ATIM",
            IEEE80211_SUBTYPE_DISASSOC => "Disassociation",
            IEEE80211_SUBTYPE_AUTH => "Authentication",
            IEEE80211_SUBTYPE_DEAUTH => "Deauthentication",
            IEEE80211_SUBTYPE_ACTION => "Action",
            _ => "Unknown Management Frame",
        },
        IEEE80211_TYPE_CONTROL => match frame_subtype {
            0 => "Reserved",
            1 => "Reserved",
            2 => "Trigger",
            3 => "TACK",
            4 => "Beamforming Report Poll",
            5 => "VHT/HE NDP Announcement",
            6 => "Control Frame Extension",
            7 => "Control Wrapper",
            8 => "Block Ack Request",
            9 => "Block Ack",
            10 => "PS-Poll",
            11 => "RTS",
            12 => "CTS",
            13 => "ACK",
            14 => "CF-End",
            15 => "CF-End + CF-Ack",
            _ => "Unknown Control Frame",
        },
        IEEE80211_TYPE_DATA => match frame_subtype {
            0 => "Data",
            1 => "Data + CF-Ack",
            2 => "Data + CF-Poll",
            3 => "Data + CF-Ack + CF-Poll",
            4 => "Null (no data)",
            5 => "CF-Ack (no data)",
            6 => "CF-Poll (no data)",
            7 => "CF-Ack + CF-Poll (no data)",
            8 => "QoS Data",
            9 => "QoS Data + CF-Ack",
            10 => "QoS Data + CF-Poll",
            11 => "QoS Data + CF-Ack + CF-Poll",
            12 => "QoS Null (no data)",
            13 => "Reserved",
            14 => "QoS CF-Poll (no data)",
            15 => "QoS CF-Ack + CF-Poll (no data)",
            _ => "Unknown Data Frame",
        },
        _ => "Unknown",
    }
}

// Helper function to format MAC address
fn format_mac_address(bytes: [u8; 6]) -> String {
    format!(
        "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]
    )
}

pub struct Visitor;

impl Visitor {
    pub fn info(_: &Context, frame: &Frame) -> Option<String> {
        if let ProtocolInfoField::IEEE80211(frame_type, frame_subtype, signal_dbm) = &frame.protocol_field {
            let type_name = get_frame_type_name(*frame_type);
            let subtype_name = get_frame_subtype_name(*frame_type, *frame_subtype);
            
            if *signal_dbm != 0 {
                return Some(format!("IEEE 802.11 {} ({}), Signal: {} dBm", 
                                 type_name, subtype_name, *signal_dbm as i8));
            } else {
                return Some(format!("IEEE 802.11 {} ({})", type_name, subtype_name));
            }
        }
        None
    }

    pub fn parse(_: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        // Parse Radiotap header
        let header_revision = reader.read_u8()?;
        let header_pad = reader.read_u8()?;
        let header_length = reader.read_u16()?;
        let present_flags = reader.read_u32()?;
        
        // Variable to store signal strength if available
        let mut signal_dbm: u8 = 0;
        
        // Skip to the end of the Radiotap header while extracting relevant information
        let radiotap_data_start = reader.cursor;
        
        // Process Radiotap data fields based on present flags
        if present_flags & RADIOTAP_TSFT != 0 {
            reader.skip(8)?; // TSFT is 8 bytes
        }
        
        if present_flags & RADIOTAP_FLAGS != 0 {
            reader.skip(1)?; // Flags is 1 byte
        }
        
        if present_flags & RADIOTAP_RATE != 0 {
            reader.skip(1)?; // Rate is 1 byte
        }
        
        if present_flags & RADIOTAP_CHANNEL != 0 {
            reader.skip(4)?; // Channel is 4 bytes (2 for freq, 2 for flags)
        }
        
        if present_flags & RADIOTAP_FHSS != 0 {
            reader.skip(2)?; // FHSS is 2 bytes
        }
        
        if present_flags & RADIOTAP_DBM_ANTSIGNAL != 0 {
            signal_dbm = reader.read_u8()?; // Signal is 1 byte
        } else {
            // Skip to the end of the Radiotap header
            reader.cursor = radiotap_data_start + (header_length as usize - 8); // 8 bytes for the header
        }
        
        // Now we're at the IEEE 802.11 frame
        // Parse the frame control field
        let frame_control = reader.read_u16()?;
        
        // Extract frame type and subtype
        let frame_subtype = ((frame_control >> 4) & 0x0F) as u8;
        let frame_type = ((frame_control >> 2) & 0x03) as u8;
        
        // Store IEEE 802.11 information in the frame
        frame.protocol_field = ProtocolInfoField::IEEE80211(frame_type, frame_subtype, signal_dbm);
        
        // Skip the rest of the IEEE 802.11 header and payload
        // This is a simplified implementation; a complete parser would need to handle
        // different frame formats based on type and subtype
        
        Ok(Protocol::None)
    }

    pub fn detail(field: &mut Field, _: &Context, _: &Frame, reader: &mut Reader) -> Result<Protocol> {
        let mut list = vec![];
        
        // Save the initial cursor position
        let start_pos = reader.cursor;
        
        // Parse Radiotap header
        let header_revision = reader.read_u8()?;
        let header_pad = reader.read_u8()?;
        let header_length = reader.read_u16()?;
        let present_flags = reader.read_u32()?;
        
        // Add Radiotap header fields
        field_back_format!(list, reader, 1, format!("Header Revision: {}", header_revision));
        field_back_format!(list, reader, 1, format!("Header Pad: {}", header_pad));
        field_back_format!(list, reader, 2, format!("Header Length: {} bytes", header_length));
        field_back_format!(list, reader, 4, format!("Present Flags: 0x{:08x}", present_flags));
        
        // Process Radiotap data fields based on present flags
        if present_flags & RADIOTAP_TSFT != 0 {
            let tsft = reader.read_u64()?;
            field_back_format!(list, reader, 8, format!("TSFT: {} μs", tsft));
        }
        
        if present_flags & RADIOTAP_FLAGS != 0 {
            let flags = reader.read_u8()?;
            field_back_format!(list, reader, 1, format!("Flags: 0x{:02x}", flags));
            
            // Decode individual flags
            if flags & 0x01 != 0 { field_back_format!(list, reader, 0, "  CFP: During CFP".to_string()); }
            if flags & 0x02 != 0 { field_back_format!(list, reader, 0, "  Preamble: Short".to_string()); }
            if flags & 0x04 != 0 { field_back_format!(list, reader, 0, "  WEP: Encrypted".to_string()); }
            if flags & 0x08 != 0 { field_back_format!(list, reader, 0, "  Fragmentation: Fragmented".to_string()); }
            if flags & 0x10 != 0 { field_back_format!(list, reader, 0, "  FCS: Present at end".to_string()); }
            if flags & 0x20 != 0 { field_back_format!(list, reader, 0, "  Data Pad: Present".to_string()); }
            if flags & 0x40 != 0 { field_back_format!(list, reader, 0, "  Bad FCS: Failed check".to_string()); }
            if flags & 0x80 != 0 { field_back_format!(list, reader, 0, "  Short GI: Used".to_string()); }
        }
        
        if present_flags & RADIOTAP_RATE != 0 {
            let rate = reader.read_u8()?;
            field_back_format!(list, reader, 1, format!("Rate: {:.1} Mbps", rate as f32 / 2.0));
        }
        
        if present_flags & RADIOTAP_CHANNEL != 0 {
            let frequency = reader.read_u16()?;
            let channel_flags = reader.read_u16()?;
            field_back_format!(list, reader, 4, format!("Channel: {} MHz, Flags: 0x{:04x}", frequency, channel_flags));
            
            // Decode channel flags
            let band = if channel_flags & 0x0001 != 0 { "2.4 GHz" } else if channel_flags & 0x0002 != 0 { "5 GHz" } else { "Unknown" };
            field_back_format!(list, reader, 0, format!("  Band: {}", band));
            
            if channel_flags & 0x0010 != 0 { field_back_format!(list, reader, 0, "  Turbo: Active".to_string()); }
            if channel_flags & 0x0020 != 0 { field_back_format!(list, reader, 0, "  CCK: Active".to_string()); }
            if channel_flags & 0x0040 != 0 { field_back_format!(list, reader, 0, "  OFDM: Active".to_string()); }
            if channel_flags & 0x0080 != 0 { field_back_format!(list, reader, 0, "  2 GHz Spectrum: Active".to_string()); }
            if channel_flags & 0x0100 != 0 { field_back_format!(list, reader, 0, "  5 GHz Spectrum: Active".to_string()); }
            if channel_flags & 0x0200 != 0 { field_back_format!(list, reader, 0, "  Passive: Active".to_string()); }
            if channel_flags & 0x0400 != 0 { field_back_format!(list, reader, 0, "  Dynamic CCK-OFDM: Active".to_string()); }
            if channel_flags & 0x0800 != 0 { field_back_format!(list, reader, 0, "  GFSK: Active".to_string()); }
        }
        
        if present_flags & RADIOTAP_FHSS != 0 {
            let fhss = reader.read_u16()?;
            field_back_format!(list, reader, 2, format!("FHSS: Hop Set {}, Pattern {}", fhss & 0x00FF, (fhss >> 8) & 0x00FF));
        }
        
        if present_flags & RADIOTAP_DBM_ANTSIGNAL != 0 {
            let signal = reader.read_u8()?;
            field_back_format!(list, reader, 1, format!("Signal: {} dBm", signal as i8));
        }
        
        if present_flags & RADIOTAP_DBM_ANTNOISE != 0 {
            let noise = reader.read_u8()?;
            field_back_format!(list, reader, 1, format!("Noise: {} dBm", noise as i8));
        }
        
        // Skip to the end of the Radiotap header
        reader.cursor = start_pos + header_length as usize;
        
        // Now we're at the IEEE 802.11 frame
        // Parse the frame control field
        let frame_control = reader.read_u16()?;
        
        // Extract frame type and subtype
        let frame_subtype = ((frame_control >> 4) & 0x0F) as u8;
        let frame_type = ((frame_control >> 2) & 0x03) as u8;
        
        // Extract other frame control bits
        let to_ds = (frame_control >> 8) & 0x01 != 0;
        let from_ds = (frame_control >> 9) & 0x01 != 0;
        let more_frag = (frame_control >> 10) & 0x01 != 0;
        let retry = (frame_control >> 11) & 0x01 != 0;
        let power_mgmt = (frame_control >> 12) & 0x01 != 0;
        let more_data = (frame_control >> 13) & 0x01 != 0;
        let protected = (frame_control >> 14) & 0x01 != 0;
        let order = (frame_control >> 15) & 0x01 != 0;
        
        // Add IEEE 802.11 frame fields
        field_back_format!(list, reader, 2, format!("Frame Control: 0x{:04x}", frame_control));
        field_back_format!(list, reader, 0, format!("  Type: {} ({})", get_frame_type_name(frame_type), frame_type));
        field_back_format!(list, reader, 0, format!("  Subtype: {} ({})", get_frame_subtype_name(frame_type, frame_subtype), frame_subtype));
        field_back_format!(list, reader, 0, format!("  To DS: {}", to_ds));
        field_back_format!(list, reader, 0, format!("  From DS: {}", from_ds));
        field_back_format!(list, reader, 0, format!("  More Fragments: {}", more_frag));
        field_back_format!(list, reader, 0, format!("  Retry: {}", retry));
        field_back_format!(list, reader, 0, format!("  Power Management: {}", power_mgmt));
        field_back_format!(list, reader, 0, format!("  More Data: {}", more_data));
        field_back_format!(list, reader, 0, format!("  Protected Frame: {}", protected));
        field_back_format!(list, reader, 0, format!("  Order: {}", order));
        
        // Parse duration/ID field
        let duration_id = reader.read_u16()?;
        field_back_format!(list, reader, 2, format!("Duration/ID: {}", duration_id));
        
        // Parse addresses (up to 4 depending on frame type)
        // Address 1 is always present
        let addr1 = [
            reader.read_u8()?,
            reader.read_u8()?,
            reader.read_u8()?,
            reader.read_u8()?,
            reader.read_u8()?,
            reader.read_u8()?,
        ];
        field_back_format!(list, reader, 6, format!("Address 1: {}", format_mac_address(addr1)));
        
        // Address 2 is present in most frame types
        if frame_type != IEEE80211_TYPE_CONTROL || frame_subtype == 11 || frame_subtype == 12 { // RTS, CTS
            let addr2 = [
                reader.read_u8()?,
                reader.read_u8()?,
                reader.read_u8()?,
                reader.read_u8()?,
                reader.read_u8()?,
                reader.read_u8()?,
            ];
            field_back_format!(list, reader, 6, format!("Address 2: {}", format_mac_address(addr2)));
        }
        
        // Address 3 is present in management and data frames
        if frame_type == IEEE80211_TYPE_MANAGEMENT || frame_type == IEEE80211_TYPE_DATA {
            let addr3 = [
                reader.read_u8()?,
                reader.read_u8()?,
                reader.read_u8()?,
                reader.read_u8()?,
                reader.read_u8()?,
                reader.read_u8()?,
            ];
            field_back_format!(list, reader, 6, format!("Address 3: {}", format_mac_address(addr3)));
        }
        
        // Sequence control is present in management and data frames
        if frame_type == IEEE80211_TYPE_MANAGEMENT || frame_type == IEEE80211_TYPE_DATA {
            let seq_ctrl = reader.read_u16()?;
            let fragment_num = seq_ctrl & 0x000F;
            let sequence_num = (seq_ctrl >> 4) & 0x0FFF;
            field_back_format!(list, reader, 2, format!("Sequence Control: 0x{:04x}", seq_ctrl));
            field_back_format!(list, reader, 0, format!("  Fragment Number: {}", fragment_num));
            field_back_format!(list, reader, 0, format!("  Sequence Number: {}", sequence_num));
        }
        
        // Address 4 is only present in certain data frames (To DS and From DS both set)
        if frame_type == IEEE80211_TYPE_DATA && to_ds && from_ds {
            let addr4 = [
                reader.read_u8()?,
                reader.read_u8()?,
                reader.read_u8()?,
                reader.read_u8()?,
                reader.read_u8()?,
                reader.read_u8()?,
            ];
            field_back_format!(list, reader, 6, format!("Address 4: {}", format_mac_address(addr4)));
        }
        
        // QoS Control field is present in QoS data frames
        if frame_type == IEEE80211_TYPE_DATA && frame_subtype >= 8 {
            let qos_ctrl = reader.read_u16()?;
            field_back_format!(list, reader, 2, format!("QoS Control: 0x{:04x}", qos_ctrl));
        }
        
        // HT Control field is present if the Order bit is set
        if order {
            let ht_ctrl = reader.read_u32()?;
            field_back_format!(list, reader, 4, format!("HT Control: 0x{:08x}", ht_ctrl));
        }
        
        // The rest is frame body and FCS, which we'll just show as a single field
        if reader.remaining() > 0 {
            let body_length = reader.remaining();
            field_back_format!(list, reader, body_length, format!("Frame Body: {} bytes", body_length));
        }
        
        // Set the summary
        let type_name = get_frame_type_name(frame_type);
        let subtype_name = get_frame_subtype_name(frame_type, frame_subtype);
        field.summary = format!("IEEE 802.11 {} ({})", type_name, subtype_name);
        
        field.children = Some(list);
        
        Ok(Protocol::None)
    }
}