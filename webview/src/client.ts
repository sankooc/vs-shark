import { PCAPClient, ComMessage, ComLog, Panel, Frame, CTreeItem, TCPCol, Grap, Category, GrapNode, GrapLink, MainProps, OverviewSource, HexV } from "./common";
import { DataPacket, IPPacket, IPv6, Context, TCP, readBuffers, IPPack, ARP, HttpPT, UDP, TCPConnect, EtherPacket, TCPStack, ARPReply, DNS, TLS, ICMP, IGMP } from 'nshark';
import { protocolList } from 'nshark/built/src/constant';
import { TLSClientHello, TLSServerHello, TLSHandshake, TLSHandshakeMessage } from 'nshark/built/src/tls';
import { RR, RR_A, RR_CNAME, RR_SOA, RR_PRT, DHCP } from 'nshark/built/src/application';
import { DNSRecord } from 'nshark/built/src/common';
import { Protocol, IPv4 } from "nshark"
import { PPPoESS } from "nshark/built/src/dataLinkLayer";

const _map: string[] = protocolList;

export class Statc {
  size: number = 0;
  count: number = 0;
  start!: number;
  end!: number;
  stc: Map<string, number> = new Map();
  public addLable(label: string, packet: IPPacket): void {
    const count = this.stc.get(label) || 0;
    const size = packet.getPacketSize() || 0;
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
  const [hour, minutes, seconds, ms ] = [
    date.getHours(),
    date.getMinutes(),
    date.getSeconds(),
    date.getMilliseconds()
  ];
  return `${minutes}:${seconds} ${ms}`;
}
export abstract class Client extends PCAPClient {
  root!: Context;
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
    _stack(this.root, packet, items);
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
    if(ipPro){
      rs.source = ipPro.getSourceIp();
      rs.dest = ipPro.getTargetIp();
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
  init(): void {
    if (this.data) {
      console.log('data init');
      this.root = readBuffers(this.data);
      const frames = this.root.getFrames();
      const items: Frame[] = frames.map(this.convertTo.bind(this));
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
        const packet = item.getProtocol(Protocol.ETHER) as EtherPacket;
        const { nano, origin } = packet;
        const it = getArray(nano);
        it.size += origin;
        it.count += 1;
        const pname = _map[item.protocol]?.toLowerCase() || '';
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
      const dnsRecords = this.root.getDNSRecord();
      this.emitMessage(Panel.MAIN, new ComMessage<MainProps>('data', { status: 'done', items, tcps: cts, arpGraph: graph, overview, dnsRecords }));
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
  const code = '0x' + protocolType.toString(16).padStart(4, '0');
  item.label = `Address Resolution Protocol (${p.getOperation()})`;
  for (const field of p.fields) {
    const { name, size, start } = field;
    switch (name) {
      case 'htype':
        item.addIndex(`Hardware type: ${p.getHardwareType()} (${hardwareType})`, start, size);
        break;
      case 'ptype':
        item.addIndex(`Protocol type: ${p.getProtocolType()} (${code})`, start, size);
        break;
      case 'hlen':
        item.addIndex(`Hardware size: ${hardwareSize} bytes`, start, size);
        break;
      case 'plen':
        item.addIndex(`Protocol size: ${protocolSize} bytes`, start, size);
        break;
      case 'oper':
        item.addIndex(`Operation code ${p.getOperation()} (${oper})`, start, size);
        break;
      case 'senderMac':
        item.addIndex(`Sender Mac address: ${senderMac}`, start, size);
        break;
      case 'senderIp':
        item.addIndex(`Sender IP address: ${senderIp}`, start, size);
        break;
      case 'targetMac':
        item.addIndex(`Target Mac address: ${targetMac}`, start, size);
        break;
      case 'targetIp':
        item.addIndex(`Target IP address: ${targetIp}`, start, size);
        break;
    }

  }
}
// for (const field of p.fields) {
//   const { name, size, start } = field;
//   switch(name){

//   }
// }
const _resolveUdp = (item: CTreeItem, p: UDP): void => {
  item.label = 'User Datagram Protocaol';
  for (const field of p.fields) {
    const { name, size, start } = field;
    switch (name) {
      case 'sourcePort':
        item.addIndex(`Source Port: ${p.sourcePort}`, start, size);
        break;
      case 'targetPort':
        item.addIndex(`Destination Port: ${p.targetPort}`, start, size);
        break;
    }
  }

  item.append(`Payload Length: ${p.getPayloadSize()} bytes`);
}

const _resolveDNS = (item: CTreeItem, p: DNS): void => {

  for (const field of p.fields) {
    const { name, size, start } = field;
    switch (name) {
      case 'transactionId':
        item.addIndex(`Transaction Id: ${p.transactionId}`, start, size);
        break;
      case 'question':
        item.addIndex(`Questions: ${p.question}`, start, size);
        break;
      case 'answer':
        item.addIndex(`Answer RRs: ${p.answer}`, start, size);
        break;
      case 'authority':
        item.addIndex(`Authority RRs: ${p.authority}`, start, size);
        break;

      case 'addtional':
        item.addIndex(`Addtional RRs: ${p.addtional}`, start, size);
        break;
    }
  }

  const { queries, answers, authorities, addtionals } = p;
  if (queries.length) {
    const _queries = item.append('Queries');
    for (const q of queries) {
      const qs = _queries.append(`${q.name}: type ${q.type}, class ${q.cls}`);
      qs.append(`Name: ${q.name}`);
      qs.append(`Type: (${q.type})`);
      qs.append(`Class (${q.cls})`);
    }
  }
  const render = (r: RR) => {

  };

  const addRR = (title: string, items: RR[]): void => {
    const rritem = item.append(title);
    for (const rr of items) {
      const itemop = rritem.append(rr.summary());
      itemop.append(`Name: ${rr.record.onwer.toString()}`);
      itemop.append(`Type: ${rr.getType()} (${rr.record.type})`);
      itemop.append(`Class: ${rr.getClass()} (${rr.record.clz})`);
      itemop.append(`Time To Live: ${rr.record.ttl}`);
      itemop.append(`Data Length: ${rr.record.len}`);
      if (rr instanceof RR_A) {
        itemop.append(`Address: ${(rr as RR_A).ip}`);
      } else if (rr instanceof RR_CNAME) {
        itemop.append(`CNAME: ${(rr as RR_CNAME).host.toString()}`);
      } else if (rr instanceof RR_PRT) {
        itemop.append(`Domain: ${(rr as RR_PRT).domain.toString()}`);
      }
    }
  }
  if (answers.length) {
    addRR('Answers', answers);
  }

  if (authorities.length) {
    addRR('Authorities', authorities);
  }
  // transactionId
};

const _resolveTcp = (item: CTreeItem, p: TCP): void => {
  item.label = `Transmission Control Protocol, Src Port: ${p.sourcePort}, Dst Prot: ${p.targetPort}, Len: ${p.getProtocolSize()}`;
  for (const field of p.fields) {
    const { name, size, start } = field;
    switch (name) {
      case 'sourcePort':
        item.addIndex(`Source Port: ${p.sourcePort}`, start, size);
        break;
      case 'targetPort':
        item.addIndex(`Destination Port: ${p.targetPort}`, start, size);
        break;
      case 'sequence':
        item.addIndex(`Sequence: ${p.sequence}`, start, size);
        break;
      case 'acknowledge':
        item.addIndex(`Acknowledge: ${p.acknowledge}`, start, size);
        break;
    }
  }
  const psize = p.getPayloadSize();
  const start = p.getPacketSize() - psize;
  item.addIndex(`Payload Length: ${p.getPayloadSize()} bytes`, start, psize);

};

const _resolveDHCP = (item: CTreeItem, p: DHCP): void => {

}

const _resolveIGMP = (item: CTreeItem, p: IGMP): void => {
  for (const field of p.fields) {
    const { name, size, start } = field;
    switch (name) {
      case 'type':
        item.addIndex(`Type: ${p.getType()} (${p.type})`, start, size);
        break;
      case 'resp':
        item.addIndex(`Max Resp Time: ${p.resp} sec`, start, size);
        break;
      case 'address':
        item.addIndex(`Multicast Address: ${p.address}`, start, size);
        break;
    }
  }

}


const _stack = (root: Context, packet: IPPacket, items: CTreeItem[]): number => {
  const index = new Integer();
  if (packet.parent) {
    index.add(_stack(root, packet.parent, items));
  }
  const item = new CTreeItem(packet.toString());
  if (packet.end > packet.start) {
    item.index = [packet.start, packet.end - packet.start];
  }
  switch (packet.protocol) {
    case Protocol.PPPOESS: {
      const p = packet as PPPoESS;
      for (const field of p.fields) {
        const { name, size, start } = field;
        switch (name) {
          case 'head':
            item.addIndex(`Version: 1`, start, size);
            item.addIndex(`Type: 1`, start, size);
            break;
          case 'code':
            item.addIndex(`Code: ${p.getCode()} (${p.code})`, start, size);
            break;
          case 'session':
            item.addIndex(`Session Id: 0x${p.sessionId.toString(16)}`, start, size);
            break;
          case 'payload':
            item.addIndex(`Payload Length: ${p.payload}`, start, size);
            break;
          case 'protocol':
            item.addIndex(`Protocol: ${p.getPOEProtocol()} (0x${p.protocol.toString(16)})`, start, size);
            break;
        }
      }
    }
      break;
    case Protocol.TLS: {
      const p: TLS = packet as TLS;
      p.records.forEach((e) => {
        const sum = e.summary();
        const it = item.append(sum);
        it.append(`Content Type: ${e.getContentType()} (${e.type})`);
        it.append(`Version: ${e.getVersion()} (${e.version})`);
        it.append(`Length: ${e.data.length}`);
        if (e instanceof TLSHandshake) {
          const t = e as TLSHandshake;
          t.messages.forEach((msg) => {
            const message = it.append(msg.summary());
            if (msg instanceof TLSClientHello) {
              const ms = msg as TLSClientHello;
              message.append('Handshake Type: Client Hello (1)');
              message.append(`Length: ${ms.content.length}`);
              message.append(`Version: ${ms.getVersion()}`);
              message.append(`Random: ${ms.random}`);
              message.append(`Session Id: ${ms.sessionId}`);
              const suites = ms.cipherSuites;
              const cs = message.append(`Cipher Suites (${suites.length} suites)`);
              {
                for (const s of suites) {
                  cs.append(`Cipher Suite: ${TLSHandshakeMessage.getAlgoType(s)} (${s})`);
                }
              }
              const es = message.append(`Extensions (${ms.extensions.length})`);
              for (const ex of ms.extensions) {
                const _exten = es.append(ex.summary());
                _exten.append(`Type: ${ex.getType()}`);
                _exten.append(`Length: ${ex.getDataSize()}`);
              }
            } else if (msg instanceof TLSServerHello) {
              const ms = msg as TLSServerHello;
              message.append('Handshake Type: Server Hello (2)');
              message.append(`Length: ${ms.content.length}`);
              message.append(`Version: ${ms.getVersion()}`);
              message.append(`Random: ${ms.random}`);
              message.append(`Session Id: ${ms.sessionId}`);
              message.append(`Cipher Suite ${TLSHandshakeMessage.getAlgoType(ms.cipherSuite)} (${ms.cipherSuite})`);

              const es = message.append(`Extensions (${ms.extensions.length})`);
              for (const ex of ms.extensions) {
                const _exten = es.append(ex.summary());
                _exten.append(`Type: ${ex.getType()}`);
                _exten.append(`Length: ${ex.getDataSize()}`);
              }
            }
          })
        }
      });
    }
      break;
    case Protocol.DNS: {
      _resolveDNS(item, packet as DNS);
    }
      break;
    case Protocol.ICMP: {
      const p: ICMP = packet as ICMP;
      console.log(p);
      for (const field of p.fields) {
        const { name, size, start } = field;
        switch (name) {
          case 'type':
            item.addIndex(`Type: ${p.getType()} (${p.type})`, start, size);
            break;
          case 'code':
            item.addIndex(`Code: ${p.code}`, start, size);
            break;
        }
      }
    }
      break;
    case Protocol.IGMP:
      _resolveIGMP(item, packet as IGMP);
      break;
    case Protocol.HTTP: {
      const p = packet as HttpPT;
      item.label = p.summary();
      item.append(p.toString());
      p.headers.forEach((val: string) => {
        if (val) item.append(val);
      });
      item.append(`File Data ${p.getPayloadSize()} bytes`);
    }
      break;
    case Protocol.TCP: {
      _resolveTcp(item, packet as TCP);
    }
      break;
    case Protocol.IPV6: {
      const p = packet as IPv6;
      item.append(`IP version: 6`);
      for (const field of p.fields) {
        const { name, size, start } = field;
        switch (name) {
          case 'source':
            item.addIndex(`Source IP Address: (${p.source})`, start, size);
            break;
          case 'target':
            item.addIndex(`Destination IP Address: (${p.target})`, start, size);
            break;
          case 'ipprotocol':
            item.addIndex(`Protocol: ${p.nextHeader}`, start, size);
            break;
        }
      }
    }
      break;
    case Protocol.IPV4: {
      const p = packet as IPv4;
      item.append(`IP version: 4`);
      for (const field of p.fields) {
        const { name, size, start } = field;
        switch (name) {
          case 'source':
            item.addIndex(`Source IP Address: (${p.source})`, start, size);
            break;
          case 'target':
            item.addIndex(`Destination IP Address: (${p.target})`, start, size);
            break;
          case 'totalLen':
            item.addIndex(`Total length: 0x${p.totalLen.toString(16).padStart(4, '0')} (${p.totalLen})`, start, size);
            break;
          case 'identification':
            item.addIndex(`Identification: 0x${p.identification.toString(16).padStart(4, '0')} (${p.identification})`, start, size);
            break;
          case 'ttl':
            item.addIndex(`Time to Live: ${p.ttl}`, start, size);
            break;
          case 'ipprotocol':
            item.addIndex(`Protocol: ${p.getProtocalType()} (${p.ipprotocol})`, start, size);
            break;
        }
      }
    }
      break;
    case Protocol.ARP: {
      _resolveARP(item, packet as ARP);
    }
      break;
    case Protocol.UDP: {
      _resolveUdp(item, packet as UDP);
      break;
    }
      break;
    case Protocol.MAC: {
      const p = packet as DataPacket;
      for (const field of p.fields) {
        const { name, size, start } = field;
        switch (name) {
          case 'source':
            item.addIndex(`Source MAC Address: (${p.source})`, start, size);
            break;
          case 'target':
            item.addIndex(`Destination MAC Address: (${p.target})`, start, size);
            break;
          case 'type':
            const code = `0x${p.type.toUpperCase()}`
            item.addIndex(`Type : ${p.getProtocolType()} (${code})`, start, size);
            break;
        }
      }
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
      const inf = item.append(`Interface type: ${info.getLinkType()}(${info.linkType})`);
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


export { DNSRecord }