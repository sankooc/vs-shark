import { PCAPClient, ComMessage, ComLog, Panel, Frame, CTreeItem, TCPCol, Grap, Category, GrapNode, GrapLink, MainProps, OverviewSource } from "./common";
import { DataPacket, IPPacket, IPv6, AbstractRootVisitor, TCP, readBuffers, IPPack, ARP, linktypeMap, HttpPT, UDP, TCPConnect, EtherPacket, TCPStack, ARPReply } from 'protocols';
import { ARP_OPER_TYPE_MAP, ARP_HARDWARE_TYPE_MAP, etypeMap } from 'protocols/built/src/constant';
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
  'TLS',
  'SSL',
  'HTTP',
  'HTTPS',
  'WEBSOCKET',
];
export class Statc {
  size: number = 0;
  count: number = 0;
  start!: number;
  end!: number;
  stc: Map<string, number> = new Map();
  public addLable(label: string, packet: IPPacket): void {
    const count = this.stc.get(label) || 0;
    const size = (packet.getProtocal(Protocol.ETHER) as EtherPacket).packet?.length || 0;
    this.stc.set(label, count + size);
  }
  public static create(ts: number, per: number) {
    const item = new Statc();
    item.start = ts;
    item.end = ts + per;
    return item;
  }
}
const getNanoDate = (p: IPPacket) => {
  return (p.getProtocal(Protocol.ETHER) as EtherPacket).nano;
}

export abstract class Client extends PCAPClient {
  root!: AbstractRootVisitor;
  abstract emitMessage(panel: Panel, msg: ComMessage<any>): void;
  abstract printLog(log: ComLog): void;
  protected getPacket(no: number): Uint8Array {
    if (!this.root) return new Uint8Array();
    const packet: IPPacket = this.root.packets[no - 1];
    if (!packet) return new Uint8Array();
    return packet.getProtocal(Protocol.ETHER).packet;
  }
  protected buildFrameTree(no: number): CTreeItem[] {
    if (!this.root) return [];
    const packet: IPPacket = this.root.packets[no - 1];
    if (!packet) return [];
    const items: CTreeItem[] = [];
    _stack(this.root, packet, items);
    return items;
  }
  abstract selectFrame(no: number): void;

  constructor() {
    super();
  }
  protected convertTo(packet: IPPacket): Frame {
    const rs = new Frame();
    rs.time = (packet.getProtocal(Protocol.ETHER) as EtherPacket).ts;
    rs.no = packet.getIndex();
    const ip = (packet.getProtocal(Protocol.IPV4) || packet.getProtocal(Protocol.IPV6)) as IPPack;
    rs.protocol = (_map[packet.protocol] || '').toLowerCase();
    rs.style = rs.protocol;
    if ('tcp' === rs.style) {
      const tcp = packet.getProtocal(Protocol.TCP) as TCP;
      if (tcp.isDump) {
        rs.style = 'dump';
      }
    }
    const arp = packet.getProtocal(Protocol.ARP) as ARP;
    if (arp) {
      rs.source = arp.senderIp;
      rs.dest = arp.targetIp;
    } else if (ip) {
      rs.source = ip.source;
      rs.dest = ip.target;
    }
    rs.len = packet.getProtocal(Protocol.ETHER).packet.length;
    rs.info = packet.toString();

    return rs;

  }
  protected convertToConnect(connect: TCPConnect, index: number): TCPCol {
    const _str = (stack: TCPStack): string => {
      return `${stack.ip}:${stack.port}`;
    };
    const { ep1, ep2, total, tcpSize, tcpUse, count, countUse } = connect;
    const col = new TCPCol();
    col.no = index + 1;
    col.ep1 = _str(ep1);
    col.ep2 = _str(ep2);
    col.total = total;
    col.tcp = tcpSize;
    col.tcpUse = tcpUse;
    col.count = count;
    col.countUse = countUse;
    return col;
  }
  protected convertARPReplies(replies: ARPReply[]): Grap {
    const graph = new Grap();
    graph.categories.push(new Category('sender'));
    graph.categories.push(new Category('target'));
    for (let i = 0; i < replies.length; i += 1) {
      const r = replies[i];
      const index = i;
      const { host, clients } = r;
      const { ip, mac } = host;
      const h = GrapNode.create(ip, 0);
      graph.nodes.push(h);
      for (const c of clients) {
        const ch = GrapNode.create(c.ip, 1);
        graph.nodes.push(ch);
        graph.links.push(new GrapLink(ch.id, h.id));
      }
    }
    return graph;
  }
  init(): void {
    if (this.data) {
      // this.emitMessage(Panel.MAIN, new ComMessage('init', {
      //   status: 'init',
      //   time: Date.now()
      // }));
      this.root = readBuffers(this.data);
      const frames = this.root.getFrames();
      const items: Frame[] = frames.map(this.convertTo);
      const connections: TCPConnect[] = this.root.getTCPConnections();
      const cts = connections.map(this.convertToConnect);
      const arpreplies = this.root.getARPReplies();
      const graph = this.convertARPReplies(arpreplies);


      const scale = 24;
      const start = getNanoDate(frames[0]);
      const end = getNanoDate(frames[frames.length - 1]);
      const duration = end - start;
      const per = Math.floor(duration / scale);
      const result: Statc[] = [];
      let cur = start;
      let limit = cur + per;
      let rs = Statc.create(start, per);
      const ps = new Set<string>();
      const getArray = (num: number): Statc => {
        if (num < limit) {
          return rs;
        }
        result.push(rs);
        rs = Statc.create(limit, per);
        limit = limit + per;
        return getArray(num);
      }
      for (const item of frames) {
        const packet = item.getProtocal(Protocol.ETHER) as EtherPacket;
        const { nano, origin } = packet;
        const it = getArray(nano);
        it.size += origin;
        it.count += 1;
        const pname = _map[item.protocol].toLowerCase();
        it.addLable(pname, item);
        ps.add(pname);
      }

      const categories = ['total'];
      const map: any = {
        total: []
      };
      ps.forEach((c) => {
        categories.push(c);
        map[c] = [];
      });
      const labels = [];
      const countlist = [];
      for (const rs of result) {
        const { size, count, stc, start } = rs;
        labels.push(start);
        countlist.push(count);
        map.total.push(size)
        ps.forEach((c) => {
          map[c].push(stc.get(c) || 0);
        });
      }
      const overview = new OverviewSource();
      overview.legends = categories;
      overview.labels = labels;
      overview.counts = countlist;
      overview.valMap = map;
      console.log('parse complete', frames.length, cts.length);
      this.emitMessage(Panel.MAIN, new ComMessage<MainProps>('data', { status: 'done', items, tcps: cts, arpGraph: graph, overview }));
      // this.emitMessage(Panel.MAIN, new ComMessage('tcplist', cts));
      // this.emitMessage(Panel.MAIN, new ComMessage('arpGraph', graph));
      // console.log(arpreplies);
      // console.log(graph)
      // console.log(JSON.stringify(graph));
    }
  }
}

class Integer {
  val: number = 0;
  add(v: number) {
    this.val += v;
  }
}

const formatDate = (date: Date): string => {
  const [year, month, day, hour, minutes, seconds, ms] = [
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
const _resolveARP = (item: CTreeItem, p: ARP): void => {
  const { oper, senderMac, senderIp, targetMac, targetIp, hardwareType, protocolType, hardwareSize, protocolSize } = p;

  const code = '0x' + protocolType.toString(16).padStart(4, '0')
  item.label = `Address Resolution Protocol (${ARP_OPER_TYPE_MAP[oper]})`;
  item.append(`Hardware type: ${ARP_HARDWARE_TYPE_MAP[hardwareType]} (${hardwareType})`);
  item.append(`Hardware size: ${hardwareSize} bytes`);
  item.append(`Protocol type: ${etypeMap[code]} (${code})`);
  item.append(`Protocol size: ${protocolSize} bytes`);
  item.append(`opcode ${ARP_OPER_TYPE_MAP[oper]} (${oper})`);
  item.append(`Sender Mac address: ${senderMac}`);
  item.append(`Sender IP address: ${senderIp}`);
  item.append(`Target Mac address: ${targetMac}`);
  item.append(`Target IP address: ${targetIp}`);
}

const _resolve = (item: CTreeItem, p: IPv4): void => {
}
const _stack = (root: AbstractRootVisitor, packet: IPPacket, items: CTreeItem[]): number => {
  const index = new Integer();
  if (packet.parent) {
    index.add(_stack(root, packet.parent, items));
  }
  const item = new CTreeItem(packet.toString());
  switch (packet.protocol) {
    case Protocol.HTTP: {
      const p = packet as HttpPT;
      item.label = p.summary();
      item.append(p.toString());
      p.headers.forEach((val: string) => {
        if (val) item.append(val);
      });
      item.append(`File Data ${p.payload?.length || 0} bytes`);
    }
      break;
    case Protocol.TCP: {
      const p = packet as TCP;
      item.label = p.detail();
      item.append(`Sequence: ${p.sequence}`);
      item.append(`Acknowledge: ${p.acknowledge}`);
      item.append(`Source Port: ${p.sourcePort}`);
      item.append(`Destination Port: ${p.targetPort}`);
      item.append(`Payload Length: ${p.packet.length} bytes`);
    }
      break;
    case Protocol.IPV6: {
      const p = packet as IPv6;
      item.append(`IP version: 6`);
      item.append(`source ip: ${p.source}`);
      item.append(`target ip: ${p.target}`);
    }
      break;
    case Protocol.IPV4: {
      const p = packet as IPv4;
      item.append(`IP version: ${p.version}`);
      item.append(`Total length: ${p.totalLen}`);
      item.append(`Identification: ${p.identification.toString(16)}`);
      item.append(`Source Address: ${p.source}`);
      item.append(`Destination Address: ${p.target}`);
      item.append(`Protocol: ${p.ipprotocol}`);
      item.append(`TTL: ${p.ttl}`);
    }
      break;
    case Protocol.ARP: {
      _resolveARP(item, packet as ARP);
    }
      break;
    case Protocol.UDP: {
      const p = packet as UDP;
      item.append(`Source Port: ${p.sourcePort}`);
      item.append(`Destination Port: ${p.targetPort}`);
      item.append(`Payload Length: ${p.packet?.length} bytes`);
    }
      break;
    case Protocol.MAC: {
      const p = packet as DataPacket;
      item.append(`source MAC: (${p.source})`);
      item.append(`target MAC: (${p.target})`);
      const code = `0x${p.type.toUpperCase()}`
      item.append(`type: ${etypeMap[code]} (${code})`);
    }
      break;
    case Protocol.ETHER: {
      const info = root.getFileInfo()
      const { client, os, hardware } = info;
      const p = packet as EtherPacket;
      const { origin, captured } = p;
      item.label = `FRAME ${packet.index}: ${origin} bytes on wire (${origin * 8} bits), ${captured} bytes on captured (${captured * 8} bits), on interface ${info.interfaceName || ''} `

      item.append(`Divice: ${hardware}  OS: ${os} Client: ${client}`);
      const date = new Date(p.ts);
      item.append(`Frame Number: ${packet.index}`);
      const inf = item.append(`Interface type: ${linktypeMap[info.linkType]}(${info.linkType})`);
      if (info.interfaceName) {
        inf.append(`Interface name: ${info.interfaceName}`);
      }
      if (info.interfaceDesc) {
        inf.append(`Interface desc: ${info.interfaceDesc}`);
      }
      item.append(`Arrival Time ${formatDate(date)}`);
      item.append(`Frame length: ${origin} bytes (${origin * 8} bits)`);
      item.append(`Capture length: ${captured} bytes (${captured * 8} bits)`);
    }
      break;
  }
  items.push(item);
  return index.val;
};
