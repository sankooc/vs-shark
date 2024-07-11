import { AbstractReaderCreator, IP4Address, IP6Address, IPAddress, Uint8ArrayReader } from './io';
import { ARP } from '../specs/networkLayer';
import { TCP } from '../specs/transportLayer';
import { DNS, NBNS, RR_A, RR_CNAME, ResourceRecord } from '../specs/application';
import { linktypeMap } from './constant';

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
  constructor(public source: Uint8Array, public name: string, public start: number, public size: number, public render?: (f: string) => string) { }
  summary(): string {
    if (this.render) {
      return this.render(this.name);
    }
    return '';
  }
  getStartIndex(): number {
    return this.start;
  }
  getSize(): number {
    return this.size;
  }
  getChildFields(): IField[] {
    return [];
  }
  getSource(): Uint8Array {
    return this.source;
  }
}


export abstract class PosReader {
  fields: IField[] = [];
  abstract getReader(): Uint8ArrayReader;
  abstract createSummary(f: string): string;
  readHex(name: string, len: number, flag: string, render?: (f: string) => string): string {
    this.fields.push(new PacketField(this.getReader().arr, name, this.getReader().cursor, len, render || this.createSummary.bind(this)))
    return this.getReader().readHex(len, flag);
  };
  read32Hex(name: string, render?: (f: string) => string): string {
    this.fields.push(new PacketField(this.getReader().arr, name, this.getReader().cursor, 4, render || this.createSummary.bind(this)))
    return this.getReader().read32Hex();
  }
  read8(name: string, render?: (f: string) => string): number {
    this.fields.push(new PacketField(this.getReader().arr, name, this.getReader().cursor, 1, render || this.createSummary.bind(this)))
    return this.getReader().read8();
  };
  read16(name: string, littleEndian: boolean = true, render?: (f: string) => string): number {
    this.fields.push(new PacketField(this.getReader().arr, name, this.getReader().cursor, 2, render || this.createSummary.bind(this)))
    return this.getReader().read16(littleEndian);
  };
  read32(name: string, littleEndian: boolean = true, render?: (f: string) => string): number {
    this.fields.push(new PacketField(this.getReader().arr, name, this.getReader().cursor, 4, render || this.createSummary.bind(this)))
    return this.getReader().read32(littleEndian);
  };
  readIp(name: string, render?: (f: string) => string): IP4Address {
    this.fields.push(new PacketField(this.getReader().arr, name, this.getReader().cursor, 4, render || this.createSummary.bind(this)))
    return this.getReader().readIp()
  }
  readIp6(name: string, render?: (f: string) => string): IP6Address {
    this.fields.push(new PacketField(this.getReader().arr, name, this.getReader().cursor, 16, render || this.createSummary.bind(this)))
    return this.getReader().readIpv6()
  }
  readDec(name: string, len: number, flag: string, render?: (f: string) => string): string {
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
    return '';
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
  index!: number;
  protocol: Protocol;
  parent: IPPacket | null;
  fields: IField[] = [];
  constructor(parent: IPPacket | null, reader: Uint8ArrayReader, protocol: Protocol) {
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
  public getIpProvider(): IPProvider | null {
    if (this instanceof IPProvider) {
      return this as IPProvider;
    }
    if (this.parent) {
      return this.parent.getIpProvider();
    }
    return null;
  }
  public getPortProvider(): PortProvider | null {
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
  getProtocol(name: Protocol): IPPacket | null {
    if (this.protocol === name) {
      return this;
    }
    if (this.parent) {
      return this.parent.getProtocol(name);
    }
    return null;
  }
  getContext(): Context | null {
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
  interface?: number;
  ts?: number;
  nano?: number;
  captured?: number;
  origin?: number;
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
  // resolveTLS(record: TLSRecord): void {
  //   // record.extra instanceof TLSHandshake

  //   // if (record.extra instanceof TLSHandshake) {
  //     // for (const msg of (record.extra as TLSHandshake).messages) {
  //       // const ext: TLSHandshakeExtra = msg.extra;
  //       // if(!ext){
  //       //   continue;
  //       // }
  //       // if (ext instanceof TLSClientHello) {
  //       //   // this.clientHello = msg.extra;
  //       //   break;
  //       // }
  //       // if (ext instanceof TLSServerHello) {
  //       //   // this.serverHello = msg.extra;
  //       //   break;
  //       // }
  //     // }
  //   // }
  // }
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

export interface PacketElement extends PElement {
  getContext(): Context | null;
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