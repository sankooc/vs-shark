use std::fmt::Formatter;

use anyhow::{bail, Result};
use log::{info,error};
use pcap_derive::Packet2;

use crate::{
    common::Reader,
    constants::{tls_cipher_suites_mapper, tls_extension_mapper, tls_hs_message_type_mapper, tls_min_type_mapper},
    files::{Frame, Initer, PacketContext, PacketOpt, Ref2},
};

#[derive(Default, Clone, Packet2)]
struct CupherSuites {
    size: usize,
    suites: Vec<u16>,
}
impl std::fmt::Display for CupherSuites {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_fmt(format_args!("Cipher Suites ({} suites)", self.size))
    }
}
impl CupherSuites {
    fn _type(data: u16) -> String {
        format!("Cipher Suite: {} ({:#06x})", tls_cipher_suites_mapper(data), data)
    }
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _: Option<PacketOpt>) -> Result<()> {
        let len = packet.build_format(reader, Reader::_read16_be, "Cipher Suites Length: {}")?;
        let _len = len / 2;
        for _ in 0.._len {
            let code = packet.build_fn(reader, Reader::_read16_be, CupherSuites::_type)?;
            p.suites.push(code);
        }
        Ok(())
    }
}

#[derive(Default, Clone, Packet2)]
struct CompressMethod {
    size: usize,
    methods: Vec<u8>,
}
impl std::fmt::Display for CompressMethod {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_str("Compression Methods")
    }
}
impl CompressMethod {
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _: Option<PacketOpt>) -> Result<()> {
        let len = packet.build_format(reader, Reader::_read8, "Compression Methods Length: {}")?;
        for _ in 0..len {
            let code = packet.build_format(reader, Reader::_read8, "Compression Method:({})")?;
            p.methods.push(code);
        }
        Ok(())
    }
}

#[derive(Default, Clone, Packet2)]
struct ExtenstionPack {
    size: usize,
    items: Vec<Ref2<Extenstion>>,
}
impl std::fmt::Display for ExtenstionPack {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_str("Extenstions")
    }
}
impl ExtenstionPack {
    fn _type(data: u16) -> String {
        format!("Cipher Suite: {} ({:#06x})", tls_cipher_suites_mapper(data), data)
    }
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _: Option<PacketOpt>) -> Result<()> {
        let len = packet.build_format(reader, Reader::_read16_be, "Extensions Length: {}")?;
        let finish = reader.cursor() + len as usize;
        loop {
            if reader.cursor() >= finish {
                break;
            }
            let item = packet.build_packet(reader, Extenstion::create, None, None)?;
            p.items.push(item);
        }
        Ok(())
    }
}
#[derive(Default, Clone, Packet2)]
struct Extenstion {
    _type: u16,
    len: u16,
}
impl std::fmt::Display for Extenstion {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_fmt(format_args!("Extension: {} (len={})", self._type(), self.len))
    }
}
impl Extenstion {
    fn _type(&self) -> String {
        tls_extension_mapper(self._type)
    }
    fn _type_desc(&self) -> String {
        format!("Type: {} ({})", self._type(), self._type)
    }
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _: Option<PacketOpt>) -> Result<()> {
        p._type = packet.build_lazy(reader, Reader::_read16_be, Extenstion::_type_desc)?;
        p.len = packet.build_format(reader, Reader::_read16_be, "Length: {}")?;
        if p.len > 0 {
            let finish = reader.cursor() + p.len as usize;
            match p._type {
                0 => {
                    packet.build_packet(reader, super::extension::ServerName::create, None, None)?;
                }
                _ => {}
            }

            if finish > reader.cursor() {
                reader.slice(finish - reader.cursor());
            }
        }
        Ok(())
    }
}

fn hexlize(data: &[u8]) -> String {
    data.iter().map(|f| format!("{:02x}", f)).collect::<String>()
}


#[derive(Default, Packet2)]
struct Certificate {

}
impl std::fmt::Display for Certificate {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_str("Certificate")
    }
}
impl Certificate {
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _: Option<PacketOpt>) -> Result<()> {
        // let finish = opt.unwrap();
        let len = read24(reader)?;
        reader.slice(len as usize);
        Ok(())
    }
}



#[derive(Clone)]
struct HandshakeClientHello {
    random: Vec<u8>,
    session: Vec<u8>,
    ciper_suites: Ref2<CupherSuites>,
    compress_method: Ref2<CompressMethod>,
    extensions: Ref2<ExtenstionPack>,
}
impl HandshakeClientHello {
    fn create(reader: &Reader, packet: &PacketContext<HandshakeProtocol>) -> Result<Self> {
        reader.read8()?;
        let min = reader.read8()?;
        let version_desc = format!("Version: {} (0x03{})", tls_min_type_mapper(min), min);
        packet._build(reader, reader.cursor() - 2, 2, version_desc);
        let random = reader.slice(0x20);
        packet._build(reader, reader.cursor() - 0x20, 0x20, format!("Random: {}", hexlize(random)));
        let session_len = packet.build_format(reader, Reader::_read8, "Session ID Length: {}")?;
        let session = reader.slice(session_len as usize);
        packet._build(reader, reader.cursor() - session_len as usize, session_len as usize, format!("Session: {}", hexlize(session)));
        let ciper_suites: Ref2<CupherSuites> = packet.build_packet(reader, CupherSuites::create, None, None)?;
        let compress_method = packet.build_packet(reader, CompressMethod::create, None, None)?;
        let extensions = packet.build_packet(reader, ExtenstionPack::create, None, None)?;
        Ok(HandshakeClientHello {
            random: random.to_vec(),
            session: session.to_vec(),
            ciper_suites,
            compress_method,
            extensions,
        })
    }
}

#[derive(Clone)]
struct HandshakeServerHello {
    random: Vec<u8>,
    session: Vec<u8>,
    ciper_suite: u16,
    method: u8,
    extensions: Ref2<ExtenstionPack>,
}
impl HandshakeServerHello {
    fn create(reader: &Reader, packet: &PacketContext<HandshakeProtocol>) -> Result<Self> {
        reader.read8()?;
        let min = reader.read8()?;
        let version_desc = format!("Version: {} (0x03{})", tls_min_type_mapper(min), min);
        packet._build(reader, reader.cursor() - 2, 2, version_desc);
        let random = reader.slice(0x20);
        packet._build(reader, reader.cursor() - 0x20, 0x20, format!("Random: {}", hexlize(random)));
        let session_len = packet.build_format(reader, Reader::_read8, "Session ID Length: {}")?;
        let session = reader.slice(session_len as usize);
        packet._build(reader, reader.cursor() - session_len as usize, session_len as usize, format!("Session: {}", hexlize(session)));
        let ciper_suite = packet.build_fn(reader, Reader::_read16_be, CupherSuites::_type)?;
        let method = packet.build_format(reader, Reader::_read8, "Compression Method: ({})")?;
        let extensions = packet.build_packet(reader, ExtenstionPack::create, None, None)?;
        Ok(HandshakeServerHello {
            random: random.to_vec(),
            session: session.to_vec(),
            ciper_suite,
            method,
            extensions,
        })
    }
}

fn read24(reader: &Reader) -> Result<u32> {
    let h = reader.read8()? as u32;
    let l = reader.read16(true)? as u32;
    Ok((h << 16) + l)
}

#[derive(Default, Packet2)]
struct HandshakeCertificate {
    items: Vec<Ref2<Certificate>>,
}
impl std::fmt::Display for HandshakeCertificate {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_str("Certificates")
    }
}
impl HandshakeCertificate {
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, opt: Option<PacketOpt>) -> Result<()> {
        let finish = opt.unwrap();
        let len = read24(reader)?;
        if finish != reader.cursor() + len as usize {
            error!("Certificate parse error");
            bail!("Certificate parse error")
        }
        packet._build(reader, reader.cursor() - 3, 3, format!("Certificates Length: {}", len));
        loop {
            if finish >= reader.cursor() {
                break;
            }
            let item = packet.build_packet(reader, Certificate::create, None, None)?;
            p.items.push(item);
        }
        Ok(())
    }
}

#[derive(Default, Clone)]
pub enum HandshakeType {
    #[default]
    UNKNOWN,
    Encrypted,
    HELLOREQUEST,
    ClientHello(HandshakeClientHello),
    ServerHello(HandshakeServerHello),
    NewSessionTicket,
    EncryptedExtensions,
    Certificate,
    ServerKeyExchange,
    CertificateRequest,
    ServerHelloDone,
    CertificateVerify,
    ClientKeyExchange,
    Finished,
}
#[derive(Default, Clone, Packet2)]
pub struct HandshakeProtocol {
    _type: u8,
    len: u32,
    pub msg: HandshakeType,
}
impl std::fmt::Display for HandshakeProtocol {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_str("Transport Layer Security")
    }
}
impl HandshakeProtocol {
    fn _type(&self) -> String {
        tls_hs_message_type_mapper(self._type)
    }
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, opt: Option<PacketOpt>) -> Result<()> {
        let finish = opt.unwrap();
        let head = reader.read32(true)?;
        let head_type = (head >> 24 & 0xff) as u8;
        let head_len = head & 0xffffff;
        p._type = head_type;
        p.len = head_len;
        let _mes = tls_hs_message_type_mapper(head_type);
        let current = reader.cursor();
        if _mes == "unknown" || head_len == 0 {
            p.msg = HandshakeType::Encrypted;
        } else {
            if finish >= current + (head_len as usize) {
            } else {
                p.msg = HandshakeType::Encrypted;
                return Ok(());
            }
            if head_len as usize > reader.left()? {
                p.msg = HandshakeType::Encrypted;
                return Ok(());
            }
            // let h_type_desc = format!("Handshake Type: {} ({})", tls_hs_message_type_mapper(head_type), head_type);
            // let h_len_desc = format!("Length: {}", head_len);
            let _finish = reader.cursor() + head_len as usize;

            match head_type {
                1 => {
                    p.msg = HandshakeType::ClientHello(HandshakeClientHello::create(reader, packet)?);
                }
                2 => {
                    p.msg = HandshakeType::ServerHello(HandshakeServerHello::create(reader, packet)?);
                }
                11 => {
                    HandshakeCertificate::create(reader, Some(_finish))?;
                }
                _ => {
                    // if len - head_len as u16 > 4 {
                    //     info!("type:{}", _mes);
                    //     info!(
                    //         "handshake {:#10x} {},  {} {}",
                    //         head, head_type, len, head_len
                    //     );
                    // }
                }
            }

            if _finish > reader.cursor() {
                reader.slice(_finish - reader.cursor());
            }
        }
        Ok(())
    }
}
