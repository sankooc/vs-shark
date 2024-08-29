import { PCAPClient, ComMessage, ComLog, Panel, Frame, CTreeItem, TCPCol, Grap, Category, GrapNode, GrapLink, MainProps, OverviewSource, HexV } from "./common";
import { DataPacket, IPPacket, IPv6, Context, TCP, readBuffers, IPPack, ARP, HttpPT, UDP, TCPConnect, EtherPacket, TCPStack, ARPReply, DNS, TLS, ICMP, IGMP } from 'nshark';
import { protocolList } from 'nshark/built/src/common/constant';
import { TLSClientHello, TLSServerHello, TLSHandshake, TLSHandshakeMessage } from 'nshark/built/src/specs/tls';
import { RR, RR_A, RR_CNAME, RR_SOA, RR_PRT, DHCP } from 'nshark/built/src/specs/application';
import { Protocol, IPv4 } from "nshark"
import { PPPoESS } from "nshark/built/src/specs/dataLinkLayer";
import { WContext, FrameInfo } from 'rshark';
import { DNSRecord } from 'rshark';

const _map: string[] = protocolList;

export class Statc {
  size: number = 0;
  count: number = 0;
  start!: number;
  end!: number;
  stc: Map<string, number> = new Map();
  public addLable(label: string, packet: FrameInfo): void {
    const count = this.stc.get(label) || 0;
    const size = packet.len || 0;
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
  return (p.getProtocol(Protocol.ETHER) as EtherPacket).nano;
}
const parseTime = (time: number): string => {
  const date = new Date(time);
  const [hour, minutes, seconds, ms] = [
    date.getHours(),
    date.getMinutes(),
    date.getSeconds(),
    date.getMilliseconds()
  ];
  return `${minutes}:${seconds} ${ms}`;
}
export abstract class Client extends PCAPClient {
  root!: Context;
  public getContext(): Context {
    return this.root;
  }
  abstract emitMessage(panel: Panel, msg: ComMessage<any>): void;
  abstract printLog(log: ComLog): void;
  protected getPacket(no: number): Uint8Array {
    if (!this.root) return new Uint8Array();
    const packet: IPPacket = this.root.getFrame(no);
    if (!packet) return new Uint8Array();
    return packet.getPacketData();
  }
  protected buildFrameTree(no: number): CTreeItem[] {
    if (!this.root) return [];
    const packet: IPPacket = this.root.getFrame(no);
    if (!packet) return [];
    const items: CTreeItem[] = [];
    // _stack(this.root, packet, items);
    return items;
  }
  abstract selectFrame(no: number): void;

  abstract renderHexView(data: HexV): void;

  constructor() {
    super();
  }
  protected convertTo(packet: IPPacket): Frame {
    const rs = new Frame();
    const eth = packet.getProtocol(Protocol.ETHER) as EtherPacket;
    rs.time = eth.ts;
    rs.time_str = ((eth.nano - this.root.getMetadata().getStart()) + '').replace(/(\d)(?=(\d\d\d)+(?!\d))/g, "$1,");
    rs.no = packet.getIndex();
    const ip = (packet.getProtocol(Protocol.IPV4) || packet.getProtocol(Protocol.IPV6)) as IPPack;
    rs.protocol = (_map[packet.protocol] || '').toLowerCase();
    rs.style = rs.protocol;
    if ('tcp' === rs.style) {
      const tcp = packet.getProtocol(Protocol.TCP) as TCP;
      if (tcp.isDump) {
        rs.style = 'dump';
      }
    }
    const ipPro = packet.getIpProvider();
    if (ipPro) {
      rs.source = ipPro.getSourceIp().getAddress();
      rs.dest = ipPro.getTargetIp().getAddress();
    }
    // const arp = packet.getProtocol(Protocol.ARP) as ARP;
    // if (arp) {
    //   rs.source = arp.senderIp;
    //   rs.dest = arp.targetIp;
    // } else if (ip) {
    //   rs.source = ip.source;
    //   rs.dest = ip.target;
    // }
    rs.len = packet.getPacketSize();
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
  init(): MainProps {
    // if (this.data) {
    //   this.root = readBuffers(this.data);
    //   const frames = this.root.getFrames();
    //   const items: Frame[] = frames.map(this.convertTo.bind(this));
    //   const connections: TCPConnect[] = this.root.getTCPConnections();
    //   const cts = connections.map(this.convertToConnect);
    //   const arpreplies = this.root.getARPReplies();
    //   const graph = this.convertARPReplies(arpreplies);

    //   const scale = 24;
    //   const start = getNanoDate(frames[0]);
    //   const end = getNanoDate(frames[frames.length - 1]);
    //   const duration = end - start;
    //   const per = Math.floor(duration / scale);
    //   const result: Statc[] = [];
    //   let cur = start;
    //   let limit = cur + per;
    //   let rs = Statc.create(start, per);
    //   const ps = new Set<string>();
    //   const getArray = (num: number): Statc => {
    //     if (num < limit) {
    //       return rs;
    //     }
    //     result.push(rs);
    //     rs = Statc.create(limit, per);
    //     limit = limit + per;
    //     return getArray(num);
    //   }
    //   for (const item of frames) {
    //     const packet = item.getProtocol(Protocol.ETHER) as EtherPacket;
    //     const { nano, origin } = packet;
    //     const it = getArray(nano);
    //     it.size += origin;
    //     it.count += 1;
    //     const pname = _map[item.protocol]?.toLowerCase() || '';
    //     it.addLable(pname, item);
    //     ps.add(pname);
    //   }

    //   const categories = ['total'];
    //   const map: any = {
    //     total: []
    //   };
    //   ps.forEach((c) => {
    //     categories.push(c);
    //     map[c] = [];
    //   });
    //   const labels = [];
    //   const countlist = [];
    //   for (const rs of result) {
    //     const { size, count, stc, start } = rs;
    //     labels.push(start);
    //     countlist.push(count);
    //     map.total.push(size)
    //     ps.forEach((c) => {
    //       map[c].push(stc.get(c) || 0);
    //     });
    //   }
    //   const overview = new OverviewSource();
    //   overview.legends = categories;
    //   overview.labels = labels;
    //   overview.counts = countlist;
    //   overview.valMap = map;
    //   const dnsRecords = this.root.getDNSRecord();
    //   return { client: this, items, tcps: cts, arpGraph: graph, overview, dnsRecords:[] };
    // }
    return null;
  }
}