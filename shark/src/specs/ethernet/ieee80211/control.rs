use std::fmt::Display;

use anyhow::Result;
use pcap_derive::Packet;

use crate::{
    common::base::{Frame, PacketBuilder, PacketContext},
    common::io::Reader,
};

use super::i802::IEE80211;

#[derive(Default, Packet)]
pub struct Control;

impl Display for Control {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("Control")
    }
}

impl Control {
    pub fn _create(_reader: &Reader, sup: &IEE80211) -> Result<PacketContext<Self>> {
        let packet: PacketContext<Self> = Frame::create_packet();
        match sup.sub_type {
            _ => {},
        };
        Ok(packet)
    }
}