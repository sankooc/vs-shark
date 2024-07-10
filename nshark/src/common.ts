import { AbstractReaderCreator, IP4Address, IP6Address, IPAddress, Uint8ArrayReader } from './io';
import { ARP } from './networkLayer';
import { TCP } from './transportLayer';
import { DNS, NBNS, RR, RR_A, RR_CNAME, ResourceRecord } from './application';
import { linktypeMap } from './constant';
import { TLSRecord,TLSHandshake,TLSServerHello, TLSClientHello, TLSHandshakeMessage, TLSHandshakeExtra } from './tls';

export enum Protocol {
  ETHER,
  MAC,
  IPV4,
  IPV6,
  ARP,
  TCP,
  UDP,
  ICMP,
  IGMP,
  DNS,
  NBNS,
  DHCP,
  TLS,
  SSL,
  HTTP,
  HTTPS,
  WEBSOCKET,
  PPPOESS,
  SLL,
}
export enum FileType {
  PCAP,
  PCAPNG,
}

export class TCPOption {
  len: number = 0;
  data: any;
  field!: PacketField;
  constructor(public kind: number) { }
}

export interface IField {
  getStartIndex(): number;
  getSize(): number;
  summary(): string;
  getChildFields(): IField[];
  getSource(): Uint8Array;
}

export class SimpleField implements IField {
  constructor(private readonly text: string) { }
  summary(): string {
    return this.text;
  }
  getStartIndex(): number {
    return 0;
  }
  getSize(): number {
    return 0;
  }
  getChildFields(): IField[] {
    return [];
  }
  getSource(): Uint8Array {
    return new Uint8Array();
  }
}

export class PacketField implements IField {
  // source?: Uint8Array;
  constructor(public source: Uint8Array, public name: string, public start: number, public size: number, public render: (f: string) => string = null) { }
  summary(): string {
    if (this.render) {
      return this.render(this.name);
    }
    return null;
  }
  getStartIndex(): number {
    return this.start;
  }
  getSize(): number {
    return this.size;
  }
  getChildFields(): IField[] {
    return null;
  }
  getSource(): Uint8Array {
    return this.source;
  }
}


export abstract class PosReader {
  fields: IField[] = [];
  abstract getReader(): Uint8ArrayReader;
  abstract createSummary(f: string): string;
  readHex(name: string, len: number, flag: string, render: (f: string) => string = null): string {
    this.fields.push(new PacketField(this.getReader().arr, name, this.getReader().cursor, len, render || this.createSummary.bind(this)))
    return this.getReader().readHex(len, flag);
  };
  read32Hex(name: string, render: (f: string) => string = null): string {
    this.fields.push(new PacketField(this.getReader().arr, name, this.getReader().cursor, 4, render || this.createSummary.bind(this)))
    return this.getReader().read32Hex();
  }
  read8(name: string, render: (f: string) => string = null): number {
    this.fields.push(new PacketField(this.getReader().arr, name, this.getReader().cursor, 1, render || this.createSummary.bind(this)))
    return this.getReader().read8();
  };
  read16(name: string, littleEndian: boolean = true, render: (f: string) => string = null): number {
    this.fields.push(new PacketField(this.getReader().arr, name, this.getReader().cursor, 2, render || this.createSummary.bind(this)))
    return this.getReader().read16(littleEndian);
  };
  read32(name: string, littleEndian: boolean = true, render: (f: string) => string = null): number {
    this.fields.push(new PacketField(this.getReader().arr, name, this.getReader().cursor, 4, render || this.createSummary.bind(this)))
    return this.getReader().read32(littleEndian);
  };
  readIp(name: string, render: (f: string) => string = null): IP4Address {
    this.fields.push(new PacketField(this.getReader().arr, name, this.getReader().cursor, 4, render || this.createSummary.bind(this)))
    return this.getReader().readIp()
  }
  readIp6(name: string, render: (f: string) => string = null): IP6Address {
    this.fields.push(new PacketField(this.getReader().arr, name, this.getReader().cursor, 16, render || this.createSummary.bind(this)))
    return this.getReader().readIpv6()
  }
  readDec(name: string, len: number, flag: string, render: (f: string) => string = null): string {
    this.fields.push(new PacketField(this.getReader().arr, name, this.getReader().cursor, len, render || this.createSummary.bind(this)))
    return this.getReader().readDec(len, flag);
  }
  slice(name: string, len: number): Uint8Array {
    this.fields.push(new PacketField(this.getReader().arr, name, this.getReader().cursor, len, this.createSummary.bind(this)))
    return this.getReader().slice(len);
  }
  read<T>(name: string, actor: (reader: Uint8ArrayReader) => T): T {
    const reader = this.getReader();
    const p = new PacketField(reader.arr, name, reader.cursor, 0, this.createSummary.bind(this));
    this.fields.push(p)
    const result: T = actor(reader);
    p.size = reader.cursor - p.start;
    return result;
  }
  readEnter(name: string): [string, PacketField] {
    const p = new PacketField(this.getReader().arr, name, this.getReader().cursor, 0, this.createSummary.bind(this));
    this.fields.push(p)
    const result = this.getReader().readEnter();
    p.size = this.getReader().cursor - p.start;
    return [result, p];
  }
}

export class Packet extends PosReader implements IField {
  reader: Uint8ArrayReader;
  readonly start: number;
  end: number = 0;

  getReader(): Uint8ArrayReader {
    return this.reader;
  };
  constructor(reader: Uint8ArrayReader) {
    super();
    this.reader = reader;
    this.start = reader.cursor;
  }
  getPacketData(): Uint8Array {
    return this.reader.arr;
  }
  getPacketSize(): number {
    return this.reader.arr.length;
  }
  getProtocolSize(): number {
    return this.reader.arr.length - this.start;
  }
  getPayLoad(): Uint8Array {
    return this.reader.extra2();
  }
  getPayloadSize(): number {
    return this.reader.left();
  }
  _end(): void {
    this.end = this.reader.cursor;
  }
  toString(): string {
    return 'packet';
  }
  info(): string {
    return this.toString();
  }
  summary(): string {
    return this.toString();
  }
  createSummary(field: string): string {
    return null;
  }

  getChildFields(): IField[] {
    return this.fields;
  }
  getSource(): Uint8Array {
    return this.reader.arr;
  }
  getStartIndex(): number {
    return this.start;
  }
  getSize(): number {
    return this.end - this.start;
  }
}

export class FolderField extends Packet {
  label: string;
  constructor(reader: Uint8ArrayReader, label: string) {
    super(reader);
    this.label = label;
  }
  summary(): string {
    return this.label;
  }
}


// export interface MACProvider {
//     getSourceMac(): string;
//     getTargetMac(): string;
// }


export class IPPacket extends Packet implements PacketElement {
  index: number;
  protocol: Protocol;
  parent!: IPPacket;
  fields: IField[] = [];
  constructor(parent: IPPacket, reader: Uint8ArrayReader, protocol: Protocol) {
    super(reader);
    this.protocol = protocol;
    this.parent = parent;
    if (this.parent) {
      this.parent._end();
    }
  }
  // public getMacProvider(): MACProvider{
  //     if(this instanceof MACProvider){
  //         return this as MACProvider;
  //     }
  //     if (this.parent) {
  //         return this.parent.getMacProvider();
  //     }
  //     return null;
  // }
  public getIpProvider(): IPProvider {
    if (this instanceof IPProvider) {
      return this as IPProvider;
    }
    if (this.parent) {
      return this.parent.getIpProvider();
    }
    return null;
  }
  public getPortProvider(): PortProvider {
    if (this instanceof PortProvider) {
      return this as PortProvider;
    }
    if (this.parent) {
      return this.parent.getPortProvider();
    }
    return null;
  }
  setIndex(inx: number): void {
    if (this.parent) {
      this.parent.setIndex(inx);
      return;
    }
    this.index = inx;
  }
  getIndex(): number {
    if (this.parent) {
      return this.parent.getIndex();
    }
    return this.index;
  }
  getProtocol(name: Protocol): IPPacket {
    if (this.protocol === name) {
      return this;
    }
    if (this.parent) {
      return this.parent.getProtocol(name);
    }
    return null;
  }
  getContext(): Context {
    const ep = (this.getProtocol(Protocol.ETHER) as EtherPacket);
    if (ep) {
      return ep.context;
    }
    return null;
  }
  getPacket(): IPPacket {
    return this;
  }
  accept(visitor?: PVisitor): IPPacket {
    if (!visitor) {
      return this;
    }
    return visitor.visit(this);
  }
}
export abstract class IPProvider extends IPPacket {
  abstract getSourceIp(): IPAddress;
  abstract getTargetIp(): IPAddress;
}
export abstract class PortProvider extends IPPacket {
  abstract getSourcePort(): number;
  abstract getTargetPort(): number;
}

export class EtherPacket extends IPPacket {
  interface: number;
  ts: number;
  nano: number;
  captured: number;
  origin: number;
  context: Context;
  constructor(reader: Uint8ArrayReader, context: Context, index: number) {
    super(null, reader, Protocol.ETHER);
    this.context = context;
    this.index = index;
  }
  summary(): string {
    const info = this.context.getFileInfo();
    return `FRAME ${this.index}: ${this.origin} bytes on wire (${this.origin * 8} bits), ${this.captured} bytes on captured (${this.captured * 8} bits), on interface ${info.interfaceName || ''} `
  }
  getChildFields(): IField[] {
    const list: string[] = [];

    const info = this.context.getFileInfo();
    const { client, os, hardware } = info;
    list.push(`Divice: ${hardware}  OS: ${os} Client: ${client}`);
    const date = new Date(this.ts);
    list.push(`Frame Number: ${this.index}`);
    list.push(`Interface name: ${info.interfaceName}, Interface type: ${info.getLinkType()}(${info.linkType})`);
    // if (info.interfaceName) {
    //   list.push(`Interface name: ${info.interfaceName}`);
    // }
    // if (info.interfaceDesc) {
    //   list.push(`Interface desc: ${info.interfaceDesc}`);
    // }
    // item.append(`Arrival Time ${formatDate(date)}`);
    list.push(`Frame length: ${this.origin} bytes (${this.origin * 8} bits)`);
    list.push(`Capture length: ${this.captured} bytes (${this.captured * 8} bits)`);
    return list.map((txt) => new SimpleField(txt));
  }
}

export interface PElement {
  accept(visitor: Visitor): IPPacket;
}
export class TCPStack {
  ip: string;
  port: number;
  start: number;
  sequence: number = 0;
  next: number = 0;
  ack: number = 0;
  finished: boolean = false;
  temp!: Uint8Array;
  _segment: number[];
  constructor(ip: string, port: number) {
    this.ip = ip;
    this.port = port;
  }
  checkDump(sequence: number, next: number) {
    return this.sequence == sequence && this.next == next;
  }
  rebuildReader(reader: Uint8ArrayReader): Uint8ArrayReader {
    if (this.temp) {
      const _content = new Uint8Array([...this.temp, ...reader.extra2()]);
      return new Uint8ArrayReader(_content);
    }
    return reader;
  }
  clearSegment(): void {
    this._segment = [];
    this.temp = null;
  };
  addSegment(no: number, data: Uint8Array): void {
    if (!data || data.length < 1) {
      return;
    }
    this._segment.push(no);
    if (this.temp) {
      this.temp = new Uint8Array([...this.temp, ...data]);
    } else {
      this.temp = data;
    }
  };
}

export class TCPConnect {
  ep1: TCPStack;
  ep2: TCPStack;
  total: number = 0;
  tcpSize: number = 0;
  tcpUse: number = 0;
  count: number = 0;
  countUse: number = 0;
  from: string;
  status: number;
  isTLS: boolean = false;
  // clientHello?: TLSClientHello;
  // serverHello?: TLSServerHello;
  constructor(ip1: string, port1: number, ip2: string, port2: number) {
    this.ep1 = new TCPStack(ip1, port1);
    this.ep2 = new TCPStack(ip2, port2);
  }
  getConnectName(): string {
    return `${this.ep1.ip}:${this.ep1.port}-${this.ep2.ip}:${this.ep2.port}`;
  }
  getStackFromStr(ip: string, port: number): TCPStack {
    if (this.ep1.ip === ip && this.ep1.port === port) {
      return this.ep1;
    }
    return this.ep2;
  }
  getStack(arch: boolean): TCPStack {
    if (arch) return this.ep1;
    return this.ep2;
  }
  resolveTLS(record: TLSRecord): void {
    // record.extra instanceof TLSHandshake

    // if (record.extra instanceof TLSHandshake) {
      // for (const msg of (record.extra as TLSHandshake).messages) {
        // const ext: TLSHandshakeExtra = msg.extra;
        // if(!ext){
        //   continue;
        // }
        // if (ext instanceof TLSClientHello) {
        //   // this.clientHello = msg.extra;
        //   break;
        // }
        // if (ext instanceof TLSServerHello) {
        //   // this.serverHello = msg.extra;
        //   break;
        // }
      // }
    // }
  }
}

export class TLSInfo {

}

export class DNSRecord {
  name: string;
  type: string;
  clz: string;
  ttl: number;
  address: string;
  source: string;
  // constructor(public name: string,public type: string,public clz: string,public ttl: number,public address: string,public source: string){}
  constructor(source: string, rr: ResourceRecord) {
    this.source = source;
    this.name = rr.owner.toString();
    this.type = rr.getType();
    this.clz = rr.getClass();
    this.ttl = rr.ttl;
    switch (this.type) {
      case 'A': {
        this.address = (rr.extra as RR_A).ip;
      }
        break;
      case 'CNAME': {
        this.address = (rr.extra as RR_CNAME).host.toString();
      }
        break;

    }
  }
}
export class Resolver {
  tcpConnections: TCPConnect[] = [];
  tcpCache: Map<string, TCPConnect> = new Map();
  arpMap: Map<string, Set<string>> = new Map();
  dnsRecord: DNSRecord[] = [];
  flush(key: string): void {
    if (!key) {
      this.tcpCache.forEach((value) => {
        this.tcpConnections.push(value);
      })
      this.tcpCache.clear();
      return;
    }
    const connect = this.tcpCache.get(key);
    this.tcpCache.set(key, null);
    if (connect) {
      this.tcpConnections.push(connect)
    }
  }
}

export interface PacketElement extends PElement {
  getContext(): Context;
  getPacket(): IPPacket;
  accept(visitor: PVisitor): IPPacket;
}

export interface Visitor {
  visit(ele: PElement): Packet;
}
export interface PVisitor extends Visitor {
  visit(ele: PacketElement): IPPacket;
}
export abstract class AbstractVisitor implements Visitor {
  abstract visit(ele: PElement): IPPacket;
}


export class FileInfo {
  type: FileType;
  hardware!: string;
  os!: string;
  client!: string;
  majorVersion!: number;
  minorVersion!: number;
  linkType!: number;
  interfaceName!: string;
  interfaceDesc!: string;
  getLinkType(): string {
    return linktypeMap[this.linkType];
  }
}

export class CNode {
  ip: string;
  mac: string;
  constructor(ip: string, mac: string) {
    this.ip = ip;
    this.mac = mac;
  }
}
export class ARPReply {
  host: CNode;
  clients: CNode[] = [];
  constructor(host: CNode) {
    this.host = host;
  }
}

export class Metadata {
  peroid: [number, number] = [0, 0];
  public getStart() {
    return this.peroid[0];
  }
}

export interface Context {
  getFrames(): IPPacket[];
  getFrame(inx: number): IPPacket;//1 based
  getCurrentIndex(): number;
  getFileInfo(): FileInfo;
  resolve(p: ARP): void;
  getARPReplies(): ARPReply[];
  resolveTCP(p: TCP): TCPConnect;
  getTCPConnections(): TCPConnect[];
  resolveDNS(p: DNS | NBNS): void;
  getDNSRecord(): DNSRecord[];
  createEtherPacket(reader: Uint8ArrayReader): EtherPacket;
  getMetadata(): Metadata;
}


export abstract class AbstractRootVisitor implements Visitor, Context {
  resolver: Resolver = new Resolver();
  readonly packets: IPPacket[] = []
  index: number = 0;
  readonly metadata: Metadata = new Metadata();
  getFrames(): IPPacket[] {
    return this.packets;
  }
  getFrame(inx: number): IPPacket {
    return this.packets[inx - 1];
  }
  getCurrentIndex(): number {
    return this.index;
  }
  abstract getFileInfo(): FileInfo;
  protected getNextIndex(): number {
    this.index += 1;
    return this.index;
  }

  createEtherPacket(reader: Uint8ArrayReader): EtherPacket {
    return new EtherPacket(reader, this, this.getNextIndex());
  }
  protected addPacket(packet: IPPacket): void {
    const epack = packet.getProtocol(Protocol.ETHER) as EtherPacket;
    if (epack) {
      const nano = epack.nano;
      if (this.packets.length == 0) {
        this.metadata.peroid[0] = nano;
      }
      this.metadata.peroid[1] = nano;
    }
    this.packets.push(packet);
  };
  public getContext(): Context {
    return this;
  }
  getMetadata(): Metadata {
    return this.metadata;
  }
  getDNSRecord(): DNSRecord[] {
    return this.resolver.dnsRecord;
  }
  resolveDNS(p: DNS): void {
    if (p.isResponse()) {
      const ip = p.getIpProvider().getSourceIp().getAddress();
      const port = p.getPortProvider().getSourcePort();
      const source = `${ip}:${port}`;
      for (const answer of p.answers) {
        switch (answer.getType()) {
          case 'A':
          case 'CNAME':
            this.resolver.dnsRecord.push(new DNSRecord(source, answer));
            break;
        }
      }
    }
  }
  resolve(p: ARP): void {
    const { oper } = p;
    if (oper === 2) {
      const sourceKey = `${p.senderMac}@${p.senderIp}`;
      let list = this.resolver.arpMap.get(sourceKey);
      if (!list) {
        list = new Set();
        this.resolver.arpMap.set(sourceKey, list);
      }
      list.add(`${p.targetMac}@${p.targetIp}`);
    }
  }
  resolveTCP(p: TCP): TCPConnect {
    if (p.rst) return null;
    const resolver = this.resolver;
    const payloadSize = p.getPayloadSize()
    const noContent = !p.syn && p.ack && !p.psh && payloadSize < 9;
    p.hasContent = !noContent;
    const [arch, ip1, port1, ip2, port2] = p.mess();
    const key = `${ip1}${port1}-${ip2}${port2}`;
    let connect = resolver.tcpCache.get(key);
    if (!connect) {
      if (noContent) return null;
      connect = new TCPConnect(ip1, port1, ip2, port2);
      resolver.tcpCache.set(key, connect);
    }
    const sequence = p.sequence;
    const nextSequence = (p.syn || p.fin) ? p.sequence + 1 : p.sequence + payloadSize
    const stack = connect.getStack(arch);
    const dump = stack.checkDump(sequence, nextSequence);
    p.isDump = dump;
    connect.count += 1;
    connect.total += p.getPacketSize();
    connect.tcpSize += payloadSize;
    if (dump) {
      return;
    }
    if (stack.next > 0 && stack.next != sequence) {
      p.missPre = true;
      stack.clearSegment();
    }
    connect.tcpUse += payloadSize;
    connect.countUse += 1;
    stack.sequence = sequence;
    stack.next = nextSequence;
    const stackRec = connect.getStack(!arch);
    stackRec.ack = p.acknowledge;
    if (noContent) {
      return null;
    }
    return connect;
    // if (p.ack) {

    // }
    // if (p.ack && !p.psh) {
    //     if (p.packet.length > 10) {
    //         const len = p.getProtocol(Protocol.ETHER).packet.length;
    //     }
    // }
    // if (p.psh) {
    // }
  }
  getTCPConnections(): TCPConnect[] {
    return this.resolver.tcpConnections;
  }
  getARPReplies(): ARPReply[] {
    const arp = this.resolver.arpMap;
    const hostnames = arp.keys();
    const rs: ARPReply[] = [];
    arp.forEach((values, hostname) => {
      const [mac, ip] = hostname.split('@');
      const reply = new ARPReply(new CNode(ip, mac));
      values.forEach((val) => {
        const [mac, ip] = val.split('@');
        reply.clients.push(new CNode(ip, mac));
      });
      rs.push(reply);
    });
    return rs;
  }

  getHTTPConnects(): void { }
  abstract _visit(ele: InputElement): void;
  visit(ele: InputElement): Packet {
    const { readerCreator, content } = ele;
    const start = Date.now();
    this._visit(ele);
    const per = Date.now() - start;
    this.resolver.flush(null);
    return null;
  }
}

export class Option {
  code: number;
  value: any;
  len: number;
  constructor(code: number, value: any, len: number) {
    this.code = code;
    this.value = value;
    this.len = len;
  }
}

export class InputElement implements PElement {
  accept(visitor: Visitor): IPPacket {
    return visitor.visit(this) as IPPacket;
  }
  constructor(public content: Uint8Array, public readerCreator: AbstractReaderCreator) { }
} 