// Copyright (c) 2025 sankooc
// 
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

// Parser for IEEE 802.11 MAC frames (Link Type 105)
// References: IEEE Std 802.11-2020

use crate::{
    add_field_backstep, add_field_format,
    common::{
        concept::Field,
        core::Context,
        enum_def::{Protocol, ProtocolInfoField},
        io::Reader, // Assuming read_u16_le is available
        Frame,
    },
    protocol::enthernet_protocol_mapper,
};
use anyhow::Result;

const FRAME_TYPE_DATA: u8 = 0b10;
// const FRAME_TYPE_MANAGEMENT: u8 = 0b00;
// const FRAME_TYPE_CONTROL: u8 = 0b01;
// const FRAME_TYPE_EXTENSION: u8 = 0b11;

// Helper struct for Frame Control bits
// IEEE Std 802.11-2020, Section 9.2.4.1 Frame Control field
#[derive(Debug, Clone, Copy)]
pub struct FrameControlInfo {
    pub protocol_version: u8,
    frame_type: u8,
    frame_subtype: u8,
    to_ds: bool,
    from_ds: bool,
    pub more_fragments: bool,
    pub retry: bool,
    pub power_management: bool,
    pub more_data: bool,
    protected_frame: bool,
    order: bool, // Or HTC (Order bit for non-QoS Data, HTC for QoS Control frames)
}

impl From<u16> for FrameControlInfo {
    fn from(fc_value: u16) -> Self {
        Self {
            protocol_version: (fc_value & 0b0000_0000_0000_0011) as u8,
            frame_type: ((fc_value & 0b0000_0000_0000_1100) >> 2) as u8,
            frame_subtype: ((fc_value & 0b0000_0000_1111_0000) >> 4) as u8,
            to_ds: (fc_value & 0b0000_0001_0000_0000) != 0,
            from_ds: (fc_value & 0b0000_0010_0000_0000) != 0,
            more_fragments: (fc_value & 0b0000_0100_0000_0000) != 0,
            retry: (fc_value & 0b0000_1000_0000_0000) != 0,
            power_management: (fc_value & 0b0001_0000_0000_0000) != 0,
            more_data: (fc_value & 0b0010_0000_0000_0000) != 0,
            protected_frame: (fc_value & 0b0100_0000_0000_0000) != 0,
            order: (fc_value & 0b1000_0000_0000_0000) != 0,
        }
    }
}

impl FrameControlInfo {
    fn is_qos_frame(&self) -> bool {
        match self.frame_type {
            FRAME_TYPE_DATA => match self.frame_subtype {
                8 | 9 | 10 | 11 => {
                    return true;
                }
                _ => {}
            },
            _ => {}
        };
        false
    }
}

pub struct Visitor;

impl Visitor {
    /**
    *
    * +--------------------+  <-- Frame Control (2 bytes)
       | Frame Control      |
       +--------------------+
       | Duration / ID      |  (2 bytes)
       +--------------------+
       | Address 1          |  (6 bytes)
       +--------------------+
       | Address 2          |  (6 bytes)
       +--------------------+
       | Address 3          |  (6 bytes)
       +--------------------+
       | Sequence Control   |  (2 bytes)
       +--------------------+
       | Address 4          |  (6 bytes)
       +--------------------+
       | QoS Control        |  (2 bytes)
       +--------------------+
       | HT Control         |  (4 bytes)
       +--------------------+
       | Initialization Vector (IV) | (3 bytes)
       +--------------------+
       | Key ID             |  (1 byte)
       +--------------------+
       | Frame Body         |
       +--------------------+
       | Frame Check Sequence (FCS) | (4 bytes)
       +--------------------+

    *
    *
    */
    // Provides a summary string for the IEEE 802.11 MAC layer.
    pub fn info(_ctx: &Context, frame: &Frame) -> Option<String> {
        match &frame.protocol_field {
            ProtocolInfoField::Ieee80211(head) => {
                let fc_info = FrameControlInfo::from(*head);
                Some(frame_subtype_to_str(fc_info.frame_type, fc_info.frame_subtype))
            },
            _ => Some("IEEE 802.11 MAC Layer".to_string())
        }
    }

    pub fn parse(_ctx: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        let head = reader.read16(false)?;
        frame.protocol_field = ProtocolInfoField::Ieee80211(head);
        let fc_info = FrameControlInfo::from(head);
        reader.forward(22); // duration + addr + seq 2 + 18 + 2

        if fc_info.to_ds && fc_info.from_ds && reader.left() >= 6 {
            reader.forward(6);
        }
        if fc_info.is_qos_frame() {
            reader.forward(2);
        }
        if fc_info.order {
            reader.forward(4);
        }
        if fc_info.protected_frame {
            reader.forward(8); // TODO need check type
        }
        if fc_info.frame_type == FRAME_TYPE_DATA {
            if reader.left() >= 8 {
                let _data = reader.slice(3, true)?;
                match _data {
                    [0xaa, 0xaa, 0x03] => {
                        //LLC
                        reader.forward(3);
                        let ptype = reader.read16(true)?;
                        return Ok(enthernet_protocol_mapper(ptype));
                    }
                    _ => {}
                }
            }
        }
        Ok(Protocol::None)
    }

    pub fn detail(field: &mut Field, _ctx: &Context, _frame: &Frame, reader: &mut Reader) -> Result<Protocol> {
        // 1. Frame Control Field (2 bytes, Little Endian)
        let head = reader.read16(false)?;

        let fc_info = FrameControlInfo::from(head);
        field.summary = format!("IEEE 802.11: {}", frame_subtype_to_str(fc_info.frame_type, fc_info.frame_subtype));
        // 2. Duration/ID (2 bytes, Little Endian)
        add_field_format!(field, reader, reader.read16(false)?, "Duration: {} microseconds");

        // Store BSSID based on ToDS/FromDS and address fields
        // To DS | From DS | Addr1 | Addr2 | Addr3   | Meaning for BSSID
        // -----------------------------------------------------------------
        // 0     | 0       | RA=DA | TA=SA | BSSID   | Addr3 is BSSID (IBSS or Mgt/Ctrl to STA)
        // 0     | 1       | RA=DA | TA=BSSID| SA      | Addr2 is BSSID (AP to STA)
        // 1     | 0       | RA=BSSID| TA=SA | DA      | Addr1 is BSSID (STA to AP)
        // 1     | 1       | RA    | TA    | DA      | (WDS) BSSID not directly in Addr1/2/3
        // if !fc_info.to_ds && !fc_info.from_ds { // 0 0
        //     frame.bssid = Some(addr3);
        // } else if !fc_info.to_ds && fc_info.from_ds { // 0 1
        //     frame.bssid = Some(addr2);
        // } else if fc_info.to_ds && !fc_info.from_ds { // 1 0
        //     frame.bssid = Some(addr1);
        // }
        // For 1 1 (WDS), BSSID is more complex and not set here.

        match (fc_info.to_ds, fc_info.from_ds) {
            (false, false) => {
                let mut addr = reader.read_mac()?;
                add_field_backstep!(field, reader, 6, format!("Receiver address: {}", addr));
                add_field_backstep!(field, reader, 6, format!("Destination address: {}", addr));
                addr = reader.read_mac()?;
                add_field_backstep!(field, reader, 6, format!("Transmitter address: {}", addr));
                add_field_backstep!(field, reader, 6, format!("Source address: {}", addr));
                addr = reader.read_mac()?;
                add_field_backstep!(field, reader, 6, format!("BSS id: {}", addr));
            }
            (false, true) => {
                let mut addr = reader.read_mac()?;
                add_field_backstep!(field, reader, 6, format!("Receiver address: {}", addr));
                add_field_backstep!(field, reader, 6, format!("Destination address: {}", addr));
                addr = reader.read_mac()?;
                add_field_backstep!(field, reader, 6, format!("Transmitter address: {}", addr));
                add_field_backstep!(field, reader, 6, format!("BSS id: {}", addr));
                addr = reader.read_mac()?;
                add_field_backstep!(field, reader, 6, format!("Source address: {}", addr));
            }
            (true, false) => {
                let mut addr = reader.read_mac()?;
                add_field_backstep!(field, reader, 6, format!("Receiver address: {}", addr));
                add_field_backstep!(field, reader, 6, format!("BSS id: {}", addr));
                addr = reader.read_mac()?;
                add_field_backstep!(field, reader, 6, format!("Transmitter address: {}", addr));
                add_field_backstep!(field, reader, 6, format!("Source address: {}", addr));
                addr = reader.read_mac()?;
                add_field_backstep!(field, reader, 6, format!("Destination address: {}", addr));
            }
            (true, true) => {
                let mut addr = reader.read_mac()?;
                add_field_backstep!(field, reader, 6, format!("Receiver address: {}", addr));
                addr = reader.read_mac()?;
                add_field_backstep!(field, reader, 6, format!("Transmitter address: {}", addr));
                addr = reader.read_mac()?;
                add_field_backstep!(field, reader, 6, format!("Destination address: {}", addr));
            }
        }
        // 6. Sequence Control (2 bytes, Little Endian)
        // Present in all Management frames and Data frames. Some Control frames omit it.
        // IEEE Std 802.11-2020, Table 9-1 indicates which control frames have no Seq Ctrl.
        // For simplicity, assume present if not a specific control frame known to omit it.
        // A more robust check would involve frame_type and frame_subtype.
        // if has_sequence_control && reader.left() >= 2 {
        let sequence_control = reader.read16(false)?;
        // add_field_backstep!(frame, reader, 2, "Sequence Control", &format!("{:#06x}", sequence_control), ProtocolInfoField::SequenceControl.into());
        let fragment_number = (sequence_control & 0x000F) as u8;
        let sequence_number = (sequence_control & 0xFFF0) >> 4;
        add_field_backstep!(field, reader, 2, format!("Fragment Number: {}", fragment_number));
        add_field_backstep!(field, reader, 2, format!("Sequence Number: {}", sequence_number));

        // add_field_format!(frame, "  Fragment Number", &format!("{}", fragment_number), ProtocolInfoField::FragmentNumber.into());
        // add_field_format!(frame, "  Sequence Number", &format!("{}", sequence_number), ProtocolInfoField::SequenceNumber.into());
        // }

        // 7. Address 4 (6 bytes) - Optional
        // Present only when To DS and From DS are both 1
        if fc_info.to_ds && fc_info.from_ds && reader.left() >= 6 {
            add_field_format!(field, reader, reader.read_mac()?, "Address 4 (SA in WDS): {}");
        }

        // 8. QoS Control (2 bytes) - Optional
        // Present if Subtype indicates QoS Data frame or QoS Management frame (e.g. Action with QoS bit)
        // IEEE Std 802.11-2020, Section 9.2.4.5 QoS Control field
        // The most significant bit of the Subtype field is 1 for QoS variants of Data and Management frames.
        // For Data frames: Subtype bit 7 (0-indexed) is 1. (e.g., 0b1xxx for QoS Data subtypes)
        // For Management frames: Only Action frames (subtype 0b1101) can be QoS.
        if fc_info.is_qos_frame() {
            reader.forward(2);
        }

        // TODO: Parse HT Control (if present and indicated by Order/HTC bit in Frame Control for QoS frames)
        if fc_info.order {
            reader.forward(4);
        }

        if fc_info.protected_frame {
            //  Initialization Vector (IV)
            //  Key ID
            reader.forward(8); // TODO need check type
        }

        // Determine next protocol based on Frame Type (primarily for Data frames)
        if fc_info.frame_type == FRAME_TYPE_DATA {
            if reader.left() >= 8 {
                let _data = reader.slice(3, true)?;
                match _data {
                    [0xaa, 0xaa, 0x03] => {
                        //LLC
                        reader.forward(3);
                        let ptype = reader.read16(true)?;
                        return Ok(enthernet_protocol_mapper(ptype));
                    }
                    _ => {}
                }
            }
        }

        Ok(Protocol::None)
    }
}

fn frame_subtype_to_str(frame_type: u8, subtype: u8) -> String {
    // See IEEE Std 802.11-2020, Table 9-1 "Valid type and subtype combinations"
    match (frame_type, subtype) {
        // Management (00)
        (0b00, 0b0000) => "Association Request".to_string(),
        (0b00, 0b0001) => "Association Response".to_string(),
        (0b00, 0b0010) => "Reassociation Request".to_string(),
        (0b00, 0b0011) => "Reassociation Response".to_string(),
        (0b00, 0b0100) => "Probe Request".to_string(),
        (0b00, 0b0101) => "Probe Response".to_string(),
        (0b00, 0b0110) => "Timing Advertisement".to_string(),
        (0b00, 0b0111) => "Reserved".to_string(),
        (0b00, 0b1000) => "Beacon".to_string(),
        (0b00, 0b1001) => "ATIM".to_string(),
        (0b00, 0b1010) => "Disassociation".to_string(),
        (0b00, 0b1011) => "Authentication".to_string(),
        (0b00, 0b1100) => "Deauthentication".to_string(),
        (0b00, 0b1101) => "Action".to_string(),
        (0b00, 0b1110) => "Action No Ack".to_string(),
        (0b00, 0b1111) => "Reserved".to_string(),
        // Control (01)
        (0b01, 0b0000..=0b0011) => "Reserved".to_string(),
        (0b01, 0b0100) => "Beamforming Report Poll".to_string(),
        (0b01, 0b0101) => "VHT NDP Announcement".to_string(),
        (0b01, 0b0110) => "Control Frame Extension".to_string(),
        (0b01, 0b0111) => "Control Wrapper".to_string(),
        (0b01, 0b1000) => "Block Ack Request".to_string(), // Has Sequence Control
        (0b01, 0b1001) => "Block Ack".to_string(),         // Has Sequence Control
        (0b01, 0b1010) => "PS-Poll".to_string(),
        (0b01, 0b1011) => "RTS".to_string(),
        (0b01, 0b1100) => "CTS".to_string(),
        (0b01, 0b1101) => "ACK".to_string(),
        (0b01, 0b1110) => "CF-End".to_string(),
        (0b01, 0b1111) => "CF-End + CF-Ack".to_string(),
        // Data (10)
        (0b10, 0b0000) => "Data".to_string(),
        (0b10, 0b0001) => "Data + CF-Ack".to_string(),
        (0b10, 0b0010) => "Data + CF-Poll".to_string(),
        (0b10, 0b0011) => "Data + CF-Ack + CF-Poll".to_string(),
        (0b10, 0b0100) => "Null (no data)".to_string(),
        (0b10, 0b0101) => "CF-Ack (no data)".to_string(),
        (0b10, 0b0110) => "CF-Poll (no data)".to_string(),
        (0b10, 0b0111) => "CF-Ack + CF-Poll (no data)".to_string(),
        (0b10, 0b1000) => "QoS Data".to_string(),
        (0b10, 0b1001) => "QoS Data + CF-Ack".to_string(),
        (0b10, 0b1010) => "QoS Data + CF-Poll".to_string(),
        (0b10, 0b1011) => "QoS Data + CF-Ack + CF-Poll".to_string(),
        (0b10, 0b1100) => "QoS Null (no data)".to_string(),
        (0b10, 0b1101) => "Reserved".to_string(), // Was "QoS CF-Ack (no data)" in some older docs, now reserved
        (0b10, 0b1110) => "QoS CF-Poll (no data)".to_string(),
        (0b10, 0b1111) => "QoS CF-Ack + CF-Poll (no data)".to_string(),
        // Extension (11)
        (0b11, _) => format!("Extension Subtype {}", subtype),
        _ => format!("Unknown Type/Subtype ({}, {})", frame_type, subtype),
    }
}
