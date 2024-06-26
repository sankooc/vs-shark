import { IPPacket, Protocol, readBuffers, IPv4, EtherPacket } from '../src/index';
import { readLoc } from './misc';



class Statc {
  size: number= 0;
  count: number =0;
  start!: number;
  end!: number;
  stc: Map<string, number> = new Map();
  public addLable(label: string, packet: IPPacket): void{
    const count = this.stc.get(label) || 0;
    const size = (packet.getProtocal(Protocol.ETHER) as EtherPacket).packet?.length || 0;
    this.stc.set(label, count + size);
  }
  public static create(ts: number, per: number){
    const item = new Statc();
    item.start = ts;
    item.end = ts + per;
    return item;
  }
}
const getNanoDate = (p: IPPacket) => {
  return (p.getProtocal(Protocol.ETHER) as EtherPacket).nano;
}
const _map: string[] = [
  'ETHER',
  'MAC',
  'IPV4',
  'IPV6',
  'ARP',
  'TCP',
  'UDP',
  'ICMP',
  'IGMP',
  'DNS',
  'NBNS',
  'DHCP',
  'TLS',
  'SSL',
  'HTTP',
  'HTTPS',
  'WEBSOCKET',
];

if (process.argv.length > 2) {
  const filepath = process.argv[2];
  const tmpfile = './temp'
  const e = new EventTarget();
  readLoc(filepath, (arr: Uint8Array) => {
    const root = readBuffers(arr);
    console.log('# finsih');
    console.log('total count', root.packets.length);
    // console.log(root.getFileInfo());
    // console.log(root.getARPReplies());
    console.log(root.getTCPConnections().length);

    // const items = root.packets;
    // const scale = 10;
    // const start = getNanoDate(items[0]);
    // const end = getNanoDate(items[items.length -1]);
    // const duration = end - start;
    // const per = Math.floor(duration / scale) ;
    // const result: Statc[] = [];
    // let cur = start;
    // let limit = cur + per;
    // let rs = Statc.create(start, per);
    // const ps = new Set<string>();
    // const getArray = (num: number): Statc => {
    //   if(num < limit){
    //     return rs;
    //   }
    //   result.push(rs);
    //   rs = Statc.create(limit, per);
    //   limit = limit + per;
    //   return getArray(num);
    // }
    // for(const item of items){
    //   const packet = item.getProtocal(Protocol.ETHER) as EtherPacket;
    //   const { nano , origin } = packet;
    //   const it = getArray(nano);
    //   it.size += origin;
    //   it.count += 1;
    //   const pname = _map[item.protocol].toLowerCase();
    //   it.addLable(pname, item);
    //   ps.add(pname);
    // }

    // const categories = ['total'];
    // const map: any = {
    //   total: []
    // };
    // ps.forEach((c) => {
    //   categories.push(c);
    //   map[c] = [];
    // });
    // const labels = [];
    // const countlist = [];
    // for(const rs of result){
    //   const { size, count, stc, start } = rs;
    //   labels.push(start);
    //   countlist.push(count);
    //   map.total.push(size)
    //   ps.forEach((c) => {
    //     map[c].push(stc.get(c) || 0);
    //   });
    // }
    // console.log(categories);
    // console.log(labels);
    // console.log(countlist);
    // console.log(map);
    // console.log(result);
    // console.log(JSON.stringify(result));
    // for(const item of items){
    //   const n = (item.getProtocal(Protocol.ETHER) as EtherPacket).nano;
    //   console.log(n);
    // }
  })
}