import { ICMP_TYPE_MAP } from  './index.js';



const swap = (key, _map) => {
  const list = [];
  for (const t in _map){
    const v = _map[t];
    if(typeof v === 'string'){
      const str = `${t} => "${v}".into()`;
      list.push(str);
    } else {
      const str = `${t} => {
        ${swap('code', v)}
      }`;
      list.push(str);
    }
  }
  return `match ${key} {\r\n${list.join(',\r\n')}\r\n}`
};

const _map = ICMP_TYPE_MAP;

const key = '_t';

console.log(swap(key, ICMP_TYPE_MAP));