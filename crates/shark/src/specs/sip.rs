use std::fmt::Formatter;

use super::ProtocolData;
use crate::cache::{add_sip_url, get_sip_url};
use crate::common::io::{AReader, Reader};
use crate::{
    common::base::{Frame, PacketContext, PacketOpt},
    common::FIELDSTATUS,
};
use anyhow::{bail, Result};
use pcap_derive::{Packet, Packet2, Visitor3};

// enum LineType {
//     Request,
// }

#[derive(Default, Packet)]
struct URI {
    prefix: &'static str,
    line:  &'static str,
}
impl std::fmt::Display for URI {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_fmt(format_args!("{}: {}", self.prefix, self.line))
    }
}
impl URI {
    fn create(reader: &Reader, prefix: &'static str) -> Result<PacketContext<Self>> {
        let packet: PacketContext<Self> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        p.prefix = prefix;
        
        let line = reader.read_enter()?.leak();
        
        let len = line.len();
        p.line = line;
        let uri = parse_token_with_cache(&line[..len-8]);
        if let Some(user) = &uri.user {
            packet.build_txt(format!("{} User Part: {}", prefix, user));
        }
        packet.build_txt(format!("{} Host Part: {}", prefix, uri.host));
        if let Some(port) = &uri.port {
            packet.build_txt(format!("{} Host Port: {}", prefix, port));
        }
        drop(p);
        Ok(packet)
    }
}

#[derive(Default, Packet2)]
struct SIPRequest {
    method: String,
    line: &'static str,
}

impl std::fmt::Display for SIPRequest {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_fmt(format_args!("Request-Line: {} {}", self.method, self.line))
    }
}
impl SIPRequest {
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _: Option<PacketOpt>) -> Result<()> {
        let method_read = |reader: &Reader| reader.read_space(10).ok_or(anyhow::Error::msg("parse_error"));
        p.method = packet.build_format(reader, method_read, None, "Method: {}")?;
        reader._move(1);
        let request_uri = |reader: &Reader, _: Option<PacketOpt>| URI::create(reader, "Request-URI");
        let uri = packet.build_packet(reader, request_uri, None, None)?;
        p.line = uri.borrow().line;
        Ok(())
    }
}

#[derive(Default, Packet2)]
struct SIPResponse {
    code: String,
    reason: String,
}

impl std::fmt::Display for SIPResponse {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_fmt(format_args!("Status-Line: SIP/2.0 {} {}", self.code, self.reason))
    }
}
impl SIPResponse {
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _: Option<PacketOpt>) -> Result<()> {
        reader._move(8);
        let method_read = |reader: &Reader| reader.read_space(10).ok_or(anyhow::Error::msg("parse_error"));
        p.code = packet.build_format(reader, method_read, None, "Status Code: {}")?;
        reader._move(1);
        p.reason = reader.read_enter()?;
        Ok(())
    }
}

// #[derive(Default, Packet)]
// struct Address {
//     prefix: String,
//     line: String
// }
// impl std::fmt::Display for Address {
//     fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
//         fmt.write_fmt(format_args!("{}: {}", self.prefix, self.line))
//     }
// }
// impl Address {
//     fn create(_reader: &Reader, _type: &'static str, line: String) -> Result<PacketContext<Self>> {
//         let packet: PacketContext<Self> = Frame::create_packet();
//         let mut p = packet.get().borrow_mut();
//         p.line = line.clone();
//         if let Some((_type, rest)) = line.split_once(":") {
//             p.prefix = _type.into();
//             if let Some((en, attr)) = rest.trim().split_once(";") {
//                 let len = en.len();
//                 let _url = &en[1..len-1];
                
//             } else {
//                 let len = rest.len();
//                 let _url = &rest[1..len-1];
                
//             }
//         }
//         drop(p);
//         Ok(packet)
//     }
// }
#[derive(Default, Packet2)]
pub struct MessageHeader {
    // from: Option<Address>,
    // to: Option<Address>,
    // headers: Vec<String>,
}

impl std::fmt::Display for MessageHeader {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_str("Message Header")
    }
}

impl MessageHeader {
    fn _create(reader: &Reader, packet: &PacketContext<Self>, _p: &mut std::cell::RefMut<Self>, _: Option<PacketOpt>) -> Result<()> {
        loop {
            if reader.enter_flag(0) {
                break;
            }
            let line = reader.read_enter()?;
            packet.build_backward(reader, line.len(), line);
            // if let Some((key, _)) = line.split_once(":") {
                // if key == "From" {
                //     let _read = |reader: &Reader, _: Option<PacketOpt>| Address::create(reader, "From", line);
                //     packet.build_packet(reader, _read, None, None)?;
                // } else if key == "To" {
                //     // _p.to = Some(URI::create(reader, "To")?);
                // } else {
                //     packet.build_backward(reader, line.len(), line);
                // }
                // packet.build_backward(reader, line.len(), line);
            // }
        }
        Ok(())
    }
}


#[derive(Default, Packet2)]
pub struct SIP {
    _type: &'static str,
    _info: &'static str,
}
impl SIP {
    pub fn check(reader: &impl AReader) -> bool {
        let cur = reader.cursor();
        let method = reader.read_space(10);
        let rs = match method {
            Some(_method) => {
                return match _method.as_str() {
                    "INVITE" | "ACK" | "BYE" | "CANCEL" | "REGISTER" | "OPTIONS" | "SUBSCRIBE" | "NOTIFY" | "PUBLISH" | "REFER" | "UPDATE" | "INFO" | "PRACK" | "MESSAGE" => {
                        reader._move(1);
                        if let Ok(uri) = reader.read_string(3) {
                            return uri == "sip";
                        }
                        return false;
                    }
                    "SIP/2.0" => true,
                    _ => false,
                }
            }
            _ => false,
        };
        reader._set(cur);
        return rs;
    }
}

impl std::fmt::Display for SIP {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_fmt(format_args!("Session Initiation Protocol"))
    }
}

impl crate::common::base::InfoPacket for SIP {
    fn info(&self) -> String {
        format!("{}: {}", self._type, self._info)
    }

    fn status(&self) -> FIELDSTATUS {
        FIELDSTATUS::INFO
    }
}

impl SIP {
    fn _create(reader: &Reader, packet: &PacketContext<Self>, _p: &mut std::cell::RefMut<Self>, _: Option<PacketOpt>) -> Result<()> {
        let method = reader._read_space(10);
        match method {
            Some(_method) => {
                match _method.as_str() {
                    "INVITE" | "ACK" | "BYE" | "CANCEL" | "REGISTER" | "OPTIONS" | "SUBSCRIBE" | "NOTIFY" | "PUBLISH" | "REFER" | "UPDATE" | "INFO" | "PRACK" | "MESSAGE" => {
                        _p._type = "Request";
                        let req = packet.build_packet(reader, SIPRequest::create, None, None)?;
                        _p._info = req.borrow().line;

                        packet.build_packet(reader, MessageHeader::create, None, None)?;
                        if reader.enter_flag(0) {
                          reader._move(2);
                          let left = reader.left();
                          if left > 0 {
                            packet._build(reader, reader.cursor(), reader.left(), None,format!("Message Body: {}", left)); 
                          }
                        }
                        
                    }
                    "SIP/2.0" => {
                        _p._type = "Status";
                        packet.build_packet(reader, SIPResponse::create, None, None)?;
                        packet.build_packet(reader, MessageHeader::create, None, None)?;

                    },
                    _ => bail!("protocol error"),
                }
            }
            _ => bail!("protocol error"),
        };
        Ok(())
    }
}

#[derive(Visitor3)]
pub struct SIPVisitor;

impl SIPVisitor {
    pub fn visit2(&self, reader: &Reader) -> Result<(ProtocolData, &'static str)> {
        let packet = SIP::create(reader, None)?;
        Ok((ProtocolData::SIP(packet), "none"))
    }
}

#[derive(Debug, Clone)]
pub struct SIPURI {
    user: Option<String>,
    host: String,
    port: Option<u16>,
    _params: Option<Vec<(String, String)>>,
    _headers: Option<Vec<(String, String)>>,
}

impl SIPURI {
    fn new(user: Option<String>, host: String, port: Option<u16>, _params: Option<Vec<(String, String)>>, _headers: Option<Vec<(String, String)>>) -> Self {
        Self { user, host, port, _params, _headers }
    }
}


pub fn parse_token_with_cache(line: &'static str) -> &'static SIPURI{
    if let Some(data) = get_sip_url(line) {
        return data;
    } else {
        let obj = parse_token(line);
        add_sip_url(line, obj);
        return get_sip_url(line).unwrap()
    }
}
pub fn parse_token(line: & str) -> SIPURI {
    let rest = &line[4..];
    let (path, headers) = match rest.split_once('?') {
        Some((p, h)) => (p, Some(h)),
        None => (rest, None),
    };

    let (user_host, params) = match path.split_once(';') {
        Some((u, p)) => (u, Some(p)),
        None => (path, None),
    };

    let (user, host_port) = match user_host.split_once('@') {
        Some((u, h)) => (Some(u.to_string()), h),
        None => (None, user_host),
    };

    let (host, port) = match host_port.split_once(':') {
        Some((h, p)) => (h.to_string(), p.parse::<u16>().map_err(|_| "Invalid port").ok()),
        None => (host_port.to_string(), None),
    };

    let params: Option<Vec<(String, String)>> = params.map(|p| p.split(';').filter_map(|kv| kv.split_once('=').map(|(k, v)| (k.to_string(), v.to_string()))).collect());

    let headers: Option<Vec<(String, String)>> = headers.map(|h| h.split('&').filter_map(|kv| kv.split_once('=').map(|(k, v)| (k.to_string(), v.to_string()))).collect());

    SIPURI::new(user, host, port, params, headers)
}

#[cfg(test)]
mod test {
    use super::parse_token;

    #[test]
    fn unit() {
        let token1 = "sip:test@10.0.2.15:5060";
        let token2 = "sip:sip.cybercity.dk";
        let token3 = "sip:user@example.com:5060;transport=udp?subject=project&priority=urgent";
        println!("{:?}", parse_token(token1));
        println!("{:?}", parse_token(token2));
        println!("{:?}", parse_token(token3));
        // parse_token(token);
    }
}
