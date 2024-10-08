use std::fmt::Formatter;
use std::rc::Rc;

use crate::common::io::AReader;
use anyhow::{bail, Result};
use pcap_derive::Packet2;
use pcap_derive::BerPacket;

use crate::common::Ref2;
use crate::{
    common::io::Reader,
    constants::{tls_cipher_suites_mapper, tls_extension_mapper, tls_hs_message_type_mapper, tls_min_type_mapper},
    common::base::{Frame, PacketBuilder, PacketContext, PacketOpt},
};

use super::ber::SEQUENCE;
use super::ber::TLVOBJ;
use super::extension::ExtensionType;

#[derive(Default, Packet2)]
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

#[derive(Default, Packet2)]
pub struct CompressMethod {
    pub size: usize,
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

#[derive(Default, Packet2)]
pub struct ExtenstionPack {
    pub size: usize,
    items: Vec<Extenstion>,
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
            p.items.push(item.take());
        }
        Ok(())
    }
}
#[derive(Default, Packet2)]
struct Extenstion {
    _type: u16,
    len: u16,
    info: Option<ExtensionType>,
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
                    let ext = packet.build_packet(reader, super::extension::ServerName::create, None, None)?;
                    let _info = ext.take();
                    p.info = Some(ExtensionType::ServerName(_info.names));
                }
                0x0010 => {
                    let ext = packet.build_packet(reader, super::extension::Negotiation::create, None, None)?;
                    p.info = Some(ExtensionType::Negotiation(ext.take().protocols));
                }
                0x002b => {
                    let ext = packet.build_packet(reader, super::extension::Version::create, Some(p.len as usize), None)?;
                    let reff = ext.take();
                    p.info = Some(ExtensionType::Version(reff.versions));

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

// #[derive(Default, Packet2)]
// pub struct Certificates {}
// impl std::fmt::Display for Certificates {
//     fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
//         fmt.write_str("Certificate")
//     }
// }
// impl Certificates {
//     //https://www.rfc-editor.org/rfc/rfc2459#appendix-A
//     //https://www.rfc-editor.org/rfc/rfc3280#appendix-A
//     //https://www.rfc-editor.org/rfc/rfc5280#appendix-A
//     //https://www.ietf.org/rfc/rfc2246.txt
//     //https://www.cryptologie.net/article/262/what-are-x509-certificates-rfc-asn1-der/
//     //https://letsencrypt.org/zh-cn/docs/a-warm-welcome-to-asn1-and-der/
//     fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _: Option<PacketOpt>) -> Result<()> {
//         let len = read24(reader)?;
//         packet.build_skip(reader, len as usize);
//         Ok(())
//     }
// }

#[derive(Default, BerPacket)]
pub struct SExtension {
    id: TLVOBJ,
}
impl SExtension {
    fn _summary(&self) -> &'static str {
        "Extension"
    }
}
impl SEQUENCE for SExtension {
    fn _sequence(&mut self, packet: &PacketContext<Self>, index: usize, reader: &Reader, _type: u8, len: usize) -> Result<()> {
        match index {
            0 => {
                let val = super::ber::parse(_type, reader.slice(len))?;
                packet.build_backward(reader, len, format!("Extension Id: {}", val.desc()));
                self.id = val;
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Default, BerPacket)]
pub struct Extensions {
    items: Vec<SExtension>,
}
impl Extensions {
    fn _summary(&self) -> &'static str {
        "Extensions"
    }
}
impl SEQUENCE for Extensions {
    fn _sequence(&mut self, packet: &PacketContext<Self>, _: usize, reader: &Reader, _type: u8, len: usize) -> Result<()> {
        let item = packet.build_packet(reader, SExtension::create, Some(len), None)?;
        self.items.push(item.take());
        Ok(())
    }
}

#[derive(Default, BerPacket)]
pub struct SubjectPublicKey {

}
impl SubjectPublicKey {
    fn _summary(&self) -> &'static str {
        "SubjectPublicKey"
    }
}
impl SEQUENCE for SubjectPublicKey {
    fn _sequence(&mut self, packet: &PacketContext<Self>, index: usize, reader: &Reader, _type: u8, len: usize) -> Result<()> {
        let val = super::ber::parse(_type, reader.slice(len))?;
        match index {
            0 => {
                packet.build_backward(reader, len, format!("modulus: {}", val));
            }
            1 => {
                packet.build_backward(reader, len, format!("publicExponent: {}", val));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Default, BerPacket)]
pub struct SubjectPublicKeyInfo {
    signature: Option<Signature>

}
impl SubjectPublicKeyInfo {
    fn _summary(&self) -> &'static str {
        "SubjectPublicKeyInfo"
    }
}
impl SEQUENCE for SubjectPublicKeyInfo {
    fn _sequence(&mut self, packet: &PacketContext<Self>, index: usize, reader: &Reader, _type: u8, len: usize) -> Result<()> {
        match index {
            0 => {
                let item = packet.build_packet(reader, Signature::create, Some(len), None)?.take();
                self.signature = Some(item);
            }
            1 => {

            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Default, BerPacket)]
pub struct Validity {
    before: TLVOBJ,
    after: TLVOBJ,
}
impl Validity {
    fn _summary(&self) -> &'static str {
        "Validity"
    }
}
impl SEQUENCE for Validity {
    fn _sequence(&mut self, packet: &PacketContext<Self>, index: usize, reader: &Reader, _type: u8, len: usize) -> Result<()> {
        let val = super::ber::parse(_type, reader.slice(len))?;
        match index {
            0 => {
                packet.build_backward(reader, len, format!("notBefore: utcTime ({})", val));
                self.before = val;
            }
            1 => {
                packet.build_backward(reader, len, format!("notAfter: utcTime ({})", val));
                self.after = val;
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Default, BerPacket)]
pub struct RdnSequence {
    object_id: TLVOBJ,
    val: TLVOBJ,
}
impl RdnSequence {

    fn _summary(&self) -> &'static str {
        "RdnSequence"
    }
}
impl SEQUENCE for RdnSequence {
    fn _sequence(&mut self, packet: &PacketContext<Self>, index: usize, reader: &Reader, _type: u8, len: usize) -> Result<()> {
        match index {
            0 => {
                let val = super::ber::parse(_type, reader.slice(len))?;
                packet.build_backward(reader, len, format!("Object Id: {}", val.desc()));
                self.object_id = val;
            }
            _ => {
                let val = super::ber::parse(_type, reader.slice(len))?;
                packet.build_backward(reader, len, format!("Printable String: {}", val));
                self.val = val;
            },
        }
        Ok(())
    }
}
#[derive(Default, BerPacket)]
pub struct RdnSequenceList {
    items: Vec<RdnSequence>
}
impl RdnSequenceList {
    fn _summary(&self) -> &'static str {
        "rdnSequence"
    }
}
impl SEQUENCE for RdnSequenceList {
    fn _sequence(&mut self, packet: &PacketContext<Self>, _: usize, reader: &Reader, _type: u8, len: usize) -> Result<()> {
        let item = packet.build_packet(reader, RdnSequence::create, Some(len), None)?.take();
        self.items.push(item);
        Ok(())
    }
}

#[derive(Default, BerPacket)]
pub struct Rdn {
    list: Option<RdnSequenceList>
}
impl Rdn {
    fn _summary(&self) -> &'static str {
        "rdnSequence"
    }
}
impl SEQUENCE for Rdn {
    fn _sequence(&mut self, packet: &PacketContext<Self>, index: usize, reader: &Reader, _type: u8, len: usize) -> Result<()> {
        match index {
            _ => {
                self.list = Some(packet.build_packet(reader, RdnSequenceList::create, Some(len), None)?.take());
            }
        }
        Ok(())
    }
}

#[derive(Default, BerPacket)]
pub struct Signature {
    algorithm: TLVOBJ,
}

impl Signature {
    fn _summary(&self) -> &'static str {
        "Signature"
    }
}

impl SEQUENCE for Signature {
    fn _sequence(&mut self, packet: &PacketContext<Self>, index: usize, reader: &Reader, _type: u8, len: usize) -> Result<()> {
        match index {
            0 => {
                let val = super::ber::parse(_type, reader.slice(len))?;
                packet.build_backward(reader, len, format!("Algorithm Id: {}", val.desc()));
                self.algorithm = val;
            }
            1 => {
                // 0x0500
            }
            _ => {
            }
        }
        Ok(())
    }
}

#[derive(Default, BerPacket)]
pub struct TBSCertificate {
    vesion: &'static str,
    serial_number: String,
    signature: Option<Signature>,
    issuer: Option<Rdn>,
    validity: Option<Validity>,
    subject: Option<Rdn>,
    key_info: Option<SubjectPublicKeyInfo>,
    extensions: Option<Extensions>,
}
impl TBSCertificate {
    fn _summary(&self) -> &'static str {
        "TBSCertificate"
    }
    fn version(v: u8) -> &'static str {
        match v {
            1 => "v2",
            2 => "v3",
            _ => "v1",
        }
    }
}

impl SEQUENCE for TBSCertificate {
    fn _sequence(&mut self, packet: &PacketContext<Self>, index: usize, reader: &Reader, _type: u8, len: usize) -> Result<()> {
        match index {
            0 => {
                reader._move(2);
                let v = reader.read8()?;
                let version = TBSCertificate::version(v);
                packet.build_backward(reader, 1, format!("version: {} ({})", version, v));
                self.vesion = version;
            }
            1 => {
                let val = super::ber::parse(_type, reader.slice(len))?;
                self.serial_number = val.to_string();
                packet.build_backward(reader, len, format!("serialNumber: {}", val))
            }
            2 => {
                let val = packet.build_packet(reader, Signature::create, Some(len), None)?.take();
                self.signature = Some(val);
            }
            3 => {
                self.issuer = Some(packet.build_packet(reader, Rdn::create, Some(len), Some("Issuer: rdnSequence".into()))?.take());
            }
            4 => {
                let val = packet.build_packet(reader, Validity::create, Some(len), None)?;
                self.validity = Some(val.take());
            }
            5 => {
                let val = packet.build_packet(reader, Rdn::create, Some(len), Some("Subject: rdnSequence".into()))?;
                self.subject = Some(val.take());
            }
            6 => {
                let val = packet.build_packet(reader, SubjectPublicKeyInfo::create, Some(len), None)?;
                self.key_info = Some(val.take());
            }
            7 => {
                let (_type, _len) = super::ber::TLV::decode(reader, len)?;
                let val = packet.build_packet(reader, Extensions::create, Some(_len), None)?;
                self.extensions = Some(val.take());
            }
            _ => {}
        }
        Ok(())
    }
}


#[derive(Default, Packet2)]
pub struct Certificate {
    tbs_certificate: Option<TBSCertificate>,
    signature: Option<Signature>,
    value: TLVOBJ,

    // sign
}
impl std::fmt::Display for Certificate {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_str(self._summary())
    }
}
impl Certificate {
    fn _summary(&self) -> &'static str {
        "Certificate"
    }
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _len: Option<PacketOpt>) -> Result<()> {
        p.decode(packet, reader, _len.unwrap())?;
        Ok(())
    }
}

impl SEQUENCE for Certificate {
    fn _sequence(&mut self, packet: &PacketContext<Self>, index: usize, reader: &Reader, _type: u8, len: usize) -> Result<()> {
        match index {
            0 => {
                let val = packet.build_packet(reader, TBSCertificate::create, Some(len), None)?;
                self.tbs_certificate = Some(val.take());
            }
            1 => {
                let val = packet.build_packet(reader, Signature::create, Some(len), None)?;
                self.signature = Some(val.take());
            }
            2 => {
                self.value = super::ber::parse(_type, reader.slice(len))?;
                packet.build_backward(reader, len, format!("encrypted: {}", self.value));
            }
            _ => {}
        }
        Ok(())
    }
}
#[allow(dead_code)]
pub struct HandshakeClientHello {
    random: Vec<u8>,
    session: Vec<u8>,
    ciper_suites: Ref2<CupherSuites>,
    compress_method: Ref2<CompressMethod>,
    extensions: Ref2<ExtenstionPack>,
}
impl HandshakeClientHello {
    pub fn server_name(&self) -> Option<Vec<String>> {
        let exps = self.extensions.as_ref().borrow();
        for ext in exps.items.iter() {
            if let Some(_info) = &ext.info {
                if let ExtensionType::ServerName(v) = _info {
                    return Some(v.clone())
                }
            }
        }
        None
    }
    pub fn versions(&self) -> Option<Vec<String>> {
        let exps = self.extensions.as_ref().borrow();
        for ext in exps.items.iter() {
            if let Some(_info) = &ext.info {
                if let ExtensionType::Version(v) = _info {
                    return Some(v.clone());
                }
            }
        }
        None
    }
    pub fn negotiation(&self) -> Option<Vec<String>> {
        let exps = self.extensions.as_ref().borrow();
        for ext in exps.items.iter() {
            if let Some(_info) = &ext.info {
                if let ExtensionType::Negotiation(v) = _info {
                    return Some(v.clone());
                }
            }
        }
        None
    }
    pub fn ciphers(&self) -> Vec<String> {
        let reff = self.ciper_suites.as_ref().borrow();
        let list = reff.suites.iter().map(|f|tls_cipher_suites_mapper(*f)).collect::<_>();
        drop(reff);
        list
    }
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
#[allow(dead_code)]
pub struct HandshakeServerHello {
    random: Vec<u8>,
    session: Vec<u8>,
    pub ciper_suite: u16,
    method: u8,
    extensions: Ref2<ExtenstionPack>,
}
impl HandshakeServerHello {
    pub fn ciper_suite(&self)-> String {
        tls_cipher_suites_mapper(self.ciper_suite)
    }
    pub fn versions(&self) -> Option<String> {
        let exps = self.extensions.as_ref().borrow();
        for ext in exps.items.iter() {
            if let Some(_info) = &ext.info {
                if let ExtensionType::Version(v) = _info {
                    // return v.get(0).clone();
                    if let Some(version) = v.get(0) {
                        return Some(version.into())
                    }
                }
            }
        }
        None
    }
    pub fn negotiation(&self) -> Option<Vec<String>> {
        let exps = self.extensions.as_ref().borrow();
        for ext in exps.items.iter() {
            if let Some(_info) = &ext.info {
                if let ExtensionType::Negotiation(v) = _info {
                    return Some(v.clone());
                }
            }
        }
        None
    }
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
pub struct HandshakeCertificate {
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
            bail!("Certificate parse error")
        }
        packet._build(reader, reader.cursor() - 3, 3, format!("Certificates Length: {}", len));
        loop {
            if finish <= reader.cursor() {
                break;
            }
            let len = read24(reader)? as usize;
            let item = packet.build_packet(reader, Certificate::create, Some(len), None)?;
            p.items.push(item);
        }
        Ok(())
    }
}
// #[derive(Default, Packet2)]
// struct HandshakeServerKeyExchange{
//     curve_type: u8,
//     named_curv: u16
// }
// impl std::fmt::Display for HandshakeServerKeyExchange {
//     fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
//         fmt.write_str("ServerKeyExchange (12)")
//     }
// }

// impl HandshakeServerKeyExchange {
//     fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, opt: Option<PacketOpt>) -> Result<()> {
//         Ok(())
//     }
//     fn curve_type(&self) -> &'static str {
//         match self.curve_type {
//             1 => "explicit_prime",
//             2 => "explicit_char2",
//             3 => "named_curve",
//             _ => "NULL",
//         }
//     }
// }

#[derive(Default,Clone)]
pub enum HandshakeType {
    #[default]
    UNKNOWN,
    Encrypted,
    HELLOREQUEST,
    ClientHello(Rc<HandshakeClientHello>),
    ServerHello(Rc<HandshakeServerHello>),
    NewSessionTicket,
    EncryptedExtensions,
    Certificate(Ref2<HandshakeCertificate>),
    ServerKeyExchange,
    CertificateRequest,
    ServerHelloDone,
    CertificateVerify,
    ClientKeyExchange,
    Finished,
}
#[derive(Default, Packet2)]
pub struct HandshakeProtocol {
    _type: u8,
    len: u32,
    pub msg: HandshakeType,
}
impl std::fmt::Display for HandshakeProtocol {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_str(self.msg())
    }
}
impl HandshakeProtocol {
    fn _type(&self) -> String {
        tls_hs_message_type_mapper(self._type)
    }
    fn msg(&self) -> &'static str {
        match &self.msg {
            HandshakeType::Certificate(_) => "Certificate",
            HandshakeType::ClientHello(_) => "Client Hello",
            HandshakeType::ServerHello(_) => "Server Hello",
            _ => "Encrypted",
        }
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
            let h_type_desc = format!("Handshake Type: {} ({})", tls_hs_message_type_mapper(head_type), head_type);
            let h_len_desc = format!("Length: {}", head_len);
            packet._build(reader, current - 4, 1, h_type_desc);
            packet._build(reader, current - 3, 3, h_len_desc);
            let _finish = reader.cursor() + head_len as usize;

            match head_type {
                1 => {
                    p.msg = HandshakeType::ClientHello(Rc::new(HandshakeClientHello::create(reader, packet)?));
                }
                2 => {
                    p.msg = HandshakeType::ServerHello(Rc::new(HandshakeServerHello::create(reader, packet)?));
                }
                11 => {
                    let pk = packet.build_packet(reader, HandshakeCertificate::create, Some(_finish), None)?;
                    p.msg = HandshakeType::Certificate(pk.clone());
                }
                _ => {}
            }

            if _finish > reader.cursor() {
                reader.slice(_finish - reader.cursor());
            }
        }
        Ok(())
    }
}
