import { linktypeMap, DHCP_OPTION_TYPE_MAP, IGMP_TYPE_MAP, DHCP_TYPE_MAP, SLL_TYPE, ipProtocolMap, etypeMap, DNS_CLASS_MAP, ARP_HARDWARE_TYPE_MAP, TCP_OPTION_KIND_MAP, ARP_OPER_TYPE_MAP, ICMPV6_TYPE_MAP } from  './cons.js';
import { PPP_LCP_OPTION_MAP,TLS_hash_algorithm, TLS_signature_algorithm, EC_POINTS_MAP, TLS_CONTENT_TYPE_MAP,TLS_MIN_VERSION_MAP,TLS_HS_MESSAGE_TYPE,TLS_CIPHER_SUITES_MAP,TLS_EXTENSION_MAP, NBNS_TYPE_MAP } from  './cons.js';
import fs from 'node:fs';

import { rebuild } from './wireshark.js';

import { oid_map } from './wireshark/oid/dump.js';


const DNS_TYPE_MAP = rebuild('dns_type', parseInt);
const PPP_TYPE_MAP = rebuild('ppp_type', parseInt);
const PPP_LCP_TYPE_MAP = rebuild('ppp_lcp_type', parseInt);
// const IEEE802_MANAGE = rebuild('ieee802_m', parseInt);
const IEEE802_MANAGE = rebuild('ieee802_m', (v) => parseInt(v, 16));
const IEEE802_MANAGE_TAGS = rebuild('ieee802_tag', parseInt);
const IEEE802_MANAGE_CAT = rebuild('ieee802_cat', parseInt);
// const PPP_LCP_TYPE_MAP = rebuild('ppp_lcp_type', parseInt);


const items = [
  ['link_type', linktypeMap, k => parseInt(k, 10), 'u16'],
  ['ip_protocol_type', ipProtocolMap, k => parseInt(k, 10), 'u16'],
  ['ssl_type', SLL_TYPE, k => parseInt(k, 10), 'u16'],
  ['etype', etypeMap,  k => parseInt(k, 16), 'u16'],
  ['tcp_option_kind', TCP_OPTION_KIND_MAP, k => parseInt(k, 10), 'u16'],
  ['dns_class', DNS_CLASS_MAP,  k => parseInt(k, 10), 'u16'],
  ['dns_type', DNS_TYPE_MAP,  k => parseInt(k, 10), 'u16'],
  ['ieee802_subtype', IEEE802_MANAGE, k => parseInt(k, 10), 'u8'],
  ['ieee802_mnt_tags', IEEE802_MANAGE_TAGS, k => parseInt(k, 10), 'u8'],
  ['ieee802_mnt_cat', IEEE802_MANAGE_CAT, k => parseInt(k, 10), 'u8'],
  ['ppp_lcp_option_type', PPP_LCP_OPTION_MAP,  k => parseInt(k, 10), 'u8'],
  ['ppp_type', PPP_TYPE_MAP,  k => parseInt(k, 10), 'u16'],
  ['ppp_lcp_type', PPP_LCP_TYPE_MAP,  k => parseInt(k, 10), 'u8'],
  ['arp_hardware_type', ARP_HARDWARE_TYPE_MAP, k => parseInt(k, 10), 'u16'],
  ['arp_oper_type', ARP_OPER_TYPE_MAP, k => parseInt(k, 10), 'u16'],
  ['icmpv6_type', ICMPV6_TYPE_MAP, k => parseInt(k, 10), 'u16'],
  ['dhcp_option_type', DHCP_OPTION_TYPE_MAP, k => parseInt(k, 10), 'u8'],
  ['dhcp_type', DHCP_TYPE_MAP, k => parseInt(k, 10), 'u8'],
  ['igmp_type', IGMP_TYPE_MAP, k => parseInt(k, 10), 'u8'],
  ['tls_content_type', TLS_CONTENT_TYPE_MAP, k => parseInt(k, 10), 'u8'],
  ['tls_min_type', TLS_MIN_VERSION_MAP, k => parseInt(k, 10), 'u8'],
  ['tls_hs_message_type', TLS_HS_MESSAGE_TYPE, k => parseInt(k, 10), 'u8'],
  ['tls_cipher_suites', TLS_CIPHER_SUITES_MAP, k => parseInt(k, 16), 'u16', 'Reserved (GREASE)'],
  ['tls_extension', TLS_EXTENSION_MAP, k => parseInt(k, 10), 'u16'],
  ['nbns_type', NBNS_TYPE_MAP, k => parseInt(k, 10), 'u16'],
  ['oid_map', oid_map, k => `"${k}"`, "&str"],
  ['ec_points_type', EC_POINTS_MAP, k => parseInt(k, 10), 'u8', 'NULL'],
  ['hash_algorithm', TLS_hash_algorithm, k => parseInt(k, 10), 'u8', 'none'],
  ['signature_algorithm', TLS_signature_algorithm, k => parseInt(k, 10), 'u8', 'none'],
];

const create_case = (mapper, parser) => {
  return Object.keys(mapper).map((k) => {
    return `\t\t${parser(k)} => "${mapper[k]}",`
  }).join('\r\n')
}

const create_fn = (name, mapper, parser, type, def="Unknown") => {
    return `pub fn ${name}_mapper(code: ${type}) -> &'static str {
\tmatch code {
${create_case(mapper, parser)}
\t\t_ => "${def}",
\t}
}`
}

const code = items.map((item) => create_fn(item[0], item[1], item[2], item[3], item[4])).join('\r\n');

// console.log(code);

fs.writeFileSync('../crates/shark/src/constants.rs', code);

console.log('complete');