import { linktypeMap, DHCP_OPTION_TYPE_MAP, IGMP_TYPE_MAP, DHCP_TYPE_MAP, SLL_TYPE, ipProtocolMap, etypeMap, DNS_CLASS_MAP, ARP_HARDWARE_TYPE_MAP, TCP_OPTION_KIND_MAP, ARP_OPER_TYPE_MAP, ICMPV6_TYPE_MAP } from  './cons.js';
import { TLS_CONTENT_TYPE_MAP,TLS_MIN_VERSION_MAP,TLS_HS_MESSAGE_TYPE,TLS_CIPHER_SUITES_MAP,TLS_EXTENSION_MAP, NBNS_TYPE_MAP } from  './cons.js';
import fs from 'node:fs';

import { rebuild } from './wireshark.js';

import { oid_map } from './wireshark/oid/dump.js';

const str = `use lazy_static::lazy_static;
use std::collections::HashMap;
`;


const DNS_TYPE_MAP = rebuild('dns_type', parseInt);

const buildConstants = (name, obj, fn, typed) => {
  const wk = Object.keys(obj).map((k) => {return `\t\tm.insert(${fn(k)}, "${obj[k]}");`}).join('\r\n');
  return `\tpub static ref ${name}: HashMap<${typed}, &'static str> = {\r\n\t\tlet mut m = HashMap::new();
${wk}\r\n\t\tm
\t};\r\n`;
};

const buildMapper = (name, typed, _def) => {
  const def = _def || "unknown";
  return `pub fn ${name}_mapper(code:${typed}) -> String {
    (*${name}_map.get(&code).unwrap_or(&"${def}")).into()
  }`;
}

const items = [
  ['link_type', linktypeMap, k => parseInt(k, 10), 'u16'],
  ['ip_protocol_type', ipProtocolMap, k => parseInt(k, 10), 'u16'],
  ['ssl_type', SLL_TYPE, k => parseInt(k, 10), 'u16'],
  ['etype', etypeMap,  k => parseInt(k, 16), 'u16'],
  ['tcp_option_kind', TCP_OPTION_KIND_MAP, k => parseInt(k, 10), 'u16'],
  ['dns_class', DNS_CLASS_MAP,  k => parseInt(k, 10), 'u16'],
  ['dns_type', DNS_TYPE_MAP,  k => parseInt(k, 10), 'u16'],
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
  ['oid_map', oid_map, k => `"${k}"`, "&'static str"],
];
//DHCP_OPTION_TYPE
const conts = items.map((item) => buildConstants(item[0]+'_map', item[1], item[2], item[3]));

let _content = str + "lazy_static! {\r\n" + conts.join('')+ "}";

_content += (items.map((item) => buildMapper(item[0], item[3], item[4])).join('\r\n'))

fs.writeFileSync('../core/src/constants.rs', _content);

console.log('complete');