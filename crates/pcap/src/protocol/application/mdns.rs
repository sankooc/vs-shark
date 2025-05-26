use crate::{
    common::{
        concept::Field,
        core::Context,
        enum_def::{Protocol},
        io::Reader,
        Frame,
    },
    protocol::application::dns::{ Visitor as DNSVisitor},
};
use anyhow::Result;

// mDNS is essentially DNS on a different port with some specific features
// It uses the same packet format as DNS, so we can reuse most of the DNS parsing code

pub struct Visitor;

impl Visitor {
    pub fn info(ctx: &Context, frame: &Frame) -> Option<String> {
        // Use the DNS visitor to get the basic info
        if let Some(dns_info) = DNSVisitor::info(ctx, frame) {
            // Replace "Domain Name System" with "Multicast Domain Name System"
            return Some(dns_info.replace("Domain Name System", "Multicast Domain Name System"));
        }
        None
    }

    pub fn parse(ctx: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        // Use the DNS parser for mDNS
        DNSVisitor::parse(ctx, frame, reader)
    }

    pub fn detail(field: &mut Field, ctx: &Context, frame: &Frame, reader: &mut Reader) -> Result<Protocol> {
        // Use the DNS detail parser
        let result = DNSVisitor::detail(field, ctx, frame, reader)?;
        
        // Modify the summary to indicate this is mDNS
        if field.summary.starts_with("Domain Name System") {
            field.summary = field.summary.replace("Domain Name System", "Multicast Domain Name System");
        }
        
        // Add mDNS specific notes
        if let Some(children) = &mut field.children {
            // Add a note about mDNS at the beginning
            let mdns_note = Field::label(
                "Note: mDNS uses UDP port 5353 and multicast address 224.0.0.251 (IPv4) or FF02::FB (IPv6)".to_string(),
                field.start,
                field.start
            );
            children.insert(0, mdns_note);
        }
        
        Ok(result)
    }
}