import { PCAPClient, ComMessage, ComLog, Panel, Frame, CTreeItem } from "./common";
import { DataPacket, IPPacket, RootVisitor, TCP, readBuffers } from 'protocols';
import { Protocol, IPv4 } from "protocols"

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
    _stack(packet, items);
    return items;
  }
  abstract selectFrame(no: number): void;
  
  constructor(){
    super();
  }
  init(): void {
    if (this.data) {
      this.emitMessage(Panel.MAIN, new ComMessage('init', {
        status: 'init',
        time: Date.now()
      }));
      this.root = readBuffers(this.data, 100, (items: IPPacket[]) => {
      });
      const items: Frame[] = this.root.packets.map(convert);
      this.emitMessage(Panel.MAIN, new ComMessage('framelist', items));
    }
  }
}

const _createItem = (label: string, start: Integer, len: number) => {
  const item = new CTreeItem(label);
  item.index = [start.val, start.val + len];
  start.add(len);
  return item;
}
class Integer {
  val: number = 0;
  add(v: number) {
    this.val += v;
  }
}
const _stack = (packet: IPPacket, items: CTreeItem[]): number => {
  const index = new Integer();
  if(packet.parent){
    index.add(_stack(packet.parent, items));
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
    case Protocol.IPV6:
    case Protocol.IPV4:{      
      const p = packet as IPv4;
      item.append(`source ip: ${p.source}`);
      item.append(`target ip: ${p.target}`);
    }
    break;
    case Protocol.MAC: {
      const p = packet as DataPacket;
      item.append(`source MAC: (${p.source})`);
      item.append(`target MAC: (${p.target})`);
      item.append(`type: (${p.type})`);
      // item.children.push(_createItem(`source MAC: (${p.source})`, index, 6))
      // item.children.push(_createItem(`target MAC: (${p.target})`, index, 6))
      // item.children.push(_createItem(`type: (${p.type})`, index, 2))
    }
    case Protocol.ETHER: {
      // const p = packet
    }
    break;
  }
  items.push(item);
  return index.val;
};


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

export const convert = (packet: IPPacket): Frame => {
  const rs = new Frame();
  rs.time = packet.getProtocal(Protocol.ETHER).ts;
  rs.no = packet.getIndex();
  const ip = (packet.getProtocal(Protocol.IPV4) || packet.getProtocal(Protocol.IPV6)) as IPv4;
  rs.protocol = _map[packet.protocol];
  if(ip){
    rs.source = ip.source;
    rs.dest = ip.target;
  }
  rs.len = packet.getProtocal(Protocol.ETHER).packet.length;
  rs.info = packet.toString();
  return rs;
}