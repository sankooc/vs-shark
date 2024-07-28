import { linktypeMap, SLL_TYPE, ipProtocolMap, etypeMap } from  './index.js';
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
const conts = [
  buildConstants('link_type', linktypeMap, k => parseInt(k, 10), 'u16'),
  buildConstants('ip_protocol_type', ipProtocolMap, k => parseInt(k, 10), 'u16'),
  buildConstants('ssl_type', SLL_TYPE, k => parseInt(k, 10), 'u16'),
  buildConstants('etype_map', etypeMap, k => parseInt(k, 16), 'u16'),
];




const _content = str + "lazy_static! {\r\n" + conts.join('')+ "}";

fs.writeFileSync('../rshark/src/constants.rs', _content);
