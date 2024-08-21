use std::fmt::Display;

use pcap_derive::{ Packet};

#[derive(Default, Packet)]
struct Ethernet {
    protocol: Protocol,
    len: u16,
    ptype: u16,
}

impl Ethernet {
    fn _info(&self) -> String {
        return self.to_string()
    }
    fn _summary(&self) -> String {
        return self.to_string()
    }
}

impl Display for Ethernet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("ousize")?;
        Ok(())
    }
}
#[derive(Default, Debug, Copy, Clone)]
pub enum Protocol {
    ETHERNET,
    PPPoESS,
    // SSL,
    IPV4,
    // IPV6,
    // ARP,
    // TCP,
    UDP,
    // ICMP,
    // ICMPV6,
    // IGMP,
    DNS,
    // DHCP,
    // TLS,
    // HTTP,
    #[default]
    UNKNOWN,
}
pub trait ContainProtocol {
    fn get_protocol(&self) -> Protocol;
    // fn info(&self) -> String;
}
pub trait Initer {
    fn new(protocol: Protocol) -> Self;
    fn summary(&self) -> String;
}


// #[show_streams(bar)]
// fn invoke2() {}

// #[derive(Default, Packet)]
// struct Pancakes {
//     protocol: Protocol,
//     age: u16,
// }

// impl Pancakes {
//     fn _summary(&self) -> String {
//         return "".into()
//     }
// }

// impl Initer for Pancakes {
//     fn new(protocol: Protocol) -> Self {
//         Self {
//             protocol,
//             ..Default::default()
//         }
//     }

//     fn summary(&self) -> String {
//         self._summary()
//     }
// }
// impl ContainProtocol for Pancakes {
//     fn get_protocol(&self) -> Protocol {
//       self.protocol.clone()
//     }
// }

// impl Initer<Pancakes> for Pancakes {
//     fn new(protocol: Protocol) -> Pancakes {
//         Pancakes {
//             protocol,
//             ..Default::default()
//         }
//     }
// }

#[test]
fn tests() {
    // let token = DeriveInput::parse("");
    Ethernet::new(Protocol::ETHERNET);
}
