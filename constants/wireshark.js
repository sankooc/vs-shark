import fs from 'node:fs';

export const rebuild = (key, fn) => {
  const _key = fs.readFileSync(`./wireshark/${key}/key`).toString();
  const _val = fs.readFileSync(`./wireshark/${key}/value`).toString();
  const rs = {};
  let aa = _key.split('\n');
  for (const a of aa){
    const ars = a.trim().match(/^#define\s(\S+)\s+((0x)?[0-9,a-f,A-F]+)(\s.+)?$/);
    if(!ars){
      console.log(a)
    }
    if(ars.length < 3){
      continue;
    }
    const [_, k, v] = ars;
    rs[k] = fn(v);
  }
  aa = _val.split('\n');
  const rt = {};
  for (const a of aa){
    const ars = a.trim().match(/\{\s*(\S+),\s+"([^"]+)".+$/);
    if(!ars){
      console.log(a.trim())
    }
    if(ars.length < 3){
      continue;
    }
    const [_, k, v] = ars;
    if (rs[k] !== undefined) {
      rt[rs[k]] = v;
    }
  }
  return rt;
};



// fetch('dns_type', parseInt);