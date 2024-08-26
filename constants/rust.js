import { linktypeMap, SLL_TYPE, ipProtocolMap, etypeMap, DNS_CLASS_MAP, DNS_TYPE_MAP, ARP_HARDWARE_TYPE_MAP, TCP_OPTION_KIND_MAP, ARP_OPER_TYPE_MAP } from  './index.js';
import fs from 'node:fs';

const str = `use lazy_static::lazy_static;
use std::collections::HashMap;
`;

const buildConstants = (name, obj, fn, typed) => {
  const wk = Object.keys(obj).map((k) => {return `\t\tm.insert(${fn(k)}, "${obj[k]}");`}).join('\r\n');
  return `\tpub static ref ${name}: HashMap<${typed}, &'static str> = {\r\n\t\tlet mut m = HashMap::new();
${wk}\r\n\t\tm
\t};\r\n`;
};

const buildMapper = (name, typed) => {
  return `pub fn ${name}_mapper(code:${typed}) -> String {
    (*${name}_map.get(&code).unwrap_or(&"unknown")).into()
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
];

const conts = items.map((item) => buildConstants(item[0]+'_map', item[1], item[2], item[3]));

let _content = str + "lazy_static! {\r\n" + conts.join('')+ "}";

_content += (items.map((item) => buildMapper(item[0], item[3])).join('\r\n'))

fs.writeFileSync('../rshark/src/constants.rs', _content);
