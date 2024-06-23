import { PCAPClient, ComMessage, ComLog, Panel, Frame, CTreeItem } from "./common";
import { DataPacket, IPPacket, IPv6, RootVisitor, TCP, readBuffers } from 'protocols';
import { Protocol, IPv4 } from "protocols"

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
];

export abstract class Client extends PCAPClient {
  root!: RootVisitor;
  abstract emitMessage(panel: Panel, msg: ComMessage<any>): void;
  abstract printLog(log: ComLog): void;
  protected getPacket(no: number): Uint8Array {
    if(!this.root) return new Uint8Array();
    const packet: IPPacket = this.root.packets[no - 1];
    if(!packet) return new Uint8Array();
    return packet.getProtocal(Protocol.ETHER).packet;
  }
  protected buildFrameTree(no: number): CTreeItem[] {
    if(!this.root) return [];
    const packet: IPPacket = this.root.packets[no - 1];
    if(!packet) return [];
    const items: CTreeItem[] = [];
    _stack(this.root, packet, items);
    return items;
  }
  abstract selectFrame(no: number): void;
  
  constructor(){
    super();
  }
  protected convertTo(packet: IPPacket): Frame{
    const rs = new Frame();
    rs.time = packet.getProtocal(Protocol.ETHER).ts;
    rs.no = packet.getIndex();
    const ip = (packet.getProtocal(Protocol.IPV4) || packet.getProtocal(Protocol.IPV6)) as IPv4;
    rs.protocol = (_map[packet.protocol] || '').toLowerCase();
    if(ip){
      rs.source = ip.source;
      rs.dest = ip.target;
    }
    rs.len = packet.getProtocal(Protocol.ETHER).packet.length;
    rs.info = packet.toString();
    return rs;

  }
  init(): void {
    if (this.data) {
      this.emitMessage(Panel.MAIN, new ComMessage('init', {
        status: 'init',
        time: Date.now()
      }));
      this.root = readBuffers(this.data, 100, (items: IPPacket[]) => {
      });
      const items: Frame[] = this.root.packets.map(this.convertTo);
      this.emitMessage(Panel.MAIN, new ComMessage('framelist', items));
    }
  }
}

class Integer {
  val: number = 0;
  add(v: number) {
    this.val += v;
  }
}

const formatDate = (date : Date): string => {
  const [year, month, day, hour, minutes, seconds, ms ] = [
    date.getFullYear(),
    date.getMonth(),
    date.getDate(),
    date.getHours(),
    date.getMinutes(),
    date.getSeconds(),
    date.getMilliseconds()
  ];
  return `${year}/${month}/${day} ${hour}:${minutes}:${seconds}.${ms}`;
}
const _stack = (root: RootVisitor, packet: IPPacket, items: CTreeItem[]): number => {
  const index = new Integer();
  if(packet.parent){
    index.add(_stack(root, packet.parent, items));
  }
  const item = new CTreeItem(packet.toString());
  switch(packet.protocol){
    case Protocol.TCP:{
      const p = packet as TCP;
      item.append(`sequence: ${p.sequence}`);
      item.append(`acknowledge: ${p.acknowledge}`);
      item.append(`source port: ${p.sourcePort}`);
      item.append(`target port: ${p.targetPort}`);
    }
    break;
    case Protocol.IPV6:{     
      const p = packet as IPv6;
      item.append(`IP version: 6`);
      item.append(`source ip: ${p.source}`);
      item.append(`target ip: ${p.target}`);
    }
    break;
    case Protocol.IPV4:{      
      const p = packet as IPv4;
      item.append(`IP version: ${p.version}`);
      item.append(`Total length: ${p.totalLen}`);
      item.append(`Identification: ${p.identification.toString(16)}`);
      item.append(`source ip: ${p.source}`);
      item.append(`target ip: ${p.target}`);
      item.append(`protocol: ${p.ipprotocol}`);
      item.append(`TTL: ${p.ttl}`);
    }
    break;
    case Protocol.MAC: {
      const p = packet as DataPacket;
      item.append(`source MAC: (${p.source})`);
      item.append(`target MAC: (${p.target})`);
      item.append(`type: (${p.type})`);
    }
    break;
    case Protocol.ETHER: {
      item.label = `FRAME ${packet.index}: ${packet.packet.length} bytes on interface ${root.interface?.name || ''} `
      const date = new Date(packet.ts);
      item.append(`Section Number: ${packet.index}`);
      const inf = item.append(`Interface type: ${root.interface.type}`);
      if(root.interface.name){
        inf.append(`Interface name: ${root.interface.name}`);
      }
      if(root.interface.description){
        inf.append(`Interface desc: ${root.interface.description}`);
      }
      item.append(`Arrival Time ${formatDate(date)}`);
      item.append(`Frame length: ${packet.packet.length}`);
    }
    break;
  }
  items.push(item);
  return index.val;
};
