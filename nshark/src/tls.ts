import { PacketElement, IPPacket, PVisitor, Protocol, Packet, PosReader } from './common';
import { Uint8ArrayReader } from './io';
import { TCP } from './transportLayer';
import { TLS_MIN_VERSION_MAP, TLS_CONTENT_TYPE_MAP, TLS_EXTENSION_MAP, TLS_CIPHER_SUITES_MAP,TLS_HS_MESSAGE_TYPE } from './constant';
// export class ObjectViewer<T> {
  
// }
class ComposeRecord {}
export class TLSRecord extends Packet {
  type?: number;
  version?: number;
  data?: Uint8Array;
  len?: number;
  extra?: Packet;
  constructor(reader: Uint8ArrayReader) {
    super(reader);
  }
  public getDataSize(): number {
    return this.data.length;
  }
  public getContentType(): string {
    return TLS_CONTENT_TYPE_MAP[this.type] || 'KNONW TYPE'
  }
  public getVersion(): string {
    return TLS_MIN_VERSION_MAP[this.version] || 'KnownVersion';
  }
  public summary(): string {
    return `${this.getVersion()} Record Layer: ${this.getContentType()}`;
  }
  createSummary(field: string): string {
      switch (field) {
          case 'type':
              return `Content type: ${this.getContentType()} (${this.type})`;
          case 'version':
              return `Version: ${this.getVersion()}`;
          case 'len':
              return `Length: ${this.len}`
      }
      return null;
  }
  static createHanshake(reader: Uint8ArrayReader): Packet {
    const hanshake = new TLSHandshake(reader);
    do {
      const msg = TLSHandshakeMessage.read(hanshake);
      hanshake.messages.push(msg);
      hanshake.fields.push(msg);
      if (!reader.eof()) {
        break;
      }
    } while (true)
    return hanshake;
  }
  static createAlert(r: TLSRecord, _reader: Uint8ArrayReader): TLSRecord {
    return r;
  }
  static createHeartbeat(r: TLSRecord, _reader: Uint8ArrayReader): TLSRecord {
    return r;
  }
  static createCCS(r: TLSRecord, _reader: Uint8ArrayReader): TLSRecord {
    return r;
  }
  // static create(type: number, version: number, data: Uint8Array): TLSRecord {
  //   const r = new TLSRecord(type, version, data);
  //   const _reader = new Uint8ArrayReader(data);
  //   switch (type) {
  //     case 20:
  //       return TLSRecord.createCCS(r, _reader);
  //     case 21:
  //       return TLSRecord.createAlert(r, _reader);
  //     case 22:
  //       return TLSRecord.createHanshake(r, _reader);
  //     case 23:
  //       break;
  //     case 24:
  //       return TLSRecord.createHeartbeat(r, _reader);
  //   }
  //   return r;
  // }
  static read(_reader: Uint8ArrayReader): TLSRecord {
    const record = new TLSRecord(_reader);
    record.type = record.read8('type');
    const major = _reader.read8();
    record.version = record.read8('version');
    record.len = record.read16('len', false);
    record.data = record.slice('data', record.len);
    switch (record.type) {
      // case 20:
        // return TLSRecord.createCCS(r, record.data);
      // case 21:
        // return TLSRecord.createAlert(r, _reader);
        // break;
      case 22:
        record.extra = TLSRecord.createHanshake(new Uint8ArrayReader(record.data));
        record.fields.push(record.extra)
        break;
      // case 23:
      //   break;
      // case 24:
        // return TLSRecord.createHeartbeat(r, _reader);
    }
    return record;
  }
};


export class TLSAlert extends ComposeRecord {
  level: number;
  desc: number;
}

export class TLSChangeCipherSpec extends ComposeRecord {
  ccs: number;
}

export class TLSMethods {
  constructor(public data: Uint8Array) { };
}

export class TLSExtension {
  constructor(public type: number, public data: Uint8Array) { };
  public getType(): string{
    return TLS_EXTENSION_MAP[this.type];
  }
  public getDataSize(){
    return this.data.length;
  }
  public summary(): string{
    return `Extension: ${this.getType()} (len=${this.getDataSize()})`
  }
  static create(_type: number, content: Uint8Array): TLSExtension {
    return new TLSExtension(_type, content);
  }
}

const readSession = (_reader: Uint8ArrayReader): string => {
  const sessionLen = _reader.read8();
  return _reader.readHex(sessionLen);
}

const readCipherSuite = (_reader: Uint8ArrayReader): string[] => {
  const len = _reader.read16(false);
  const suiteCount = Math.round(len / 2);
  const list: string[] = [];
  for (let i = 0; i < suiteCount; i += 1) {
    list.push(_reader.readHex(2));
  }
  return list;
}

const readTLSMethods = (_reader: Uint8ArrayReader): TLSMethods => {
  //https://www.ietf.org/rfc/rfc3749.txt
  const compressMethodLen = _reader.read8();
  return new TLSMethods(_reader.slice(compressMethodLen));
}

const readTLSExtension = (_reader: Uint8ArrayReader): TLSExtension[] => {
  const extensionLen = _reader.read16(false);
  const rs: TLSExtension[] = [];
  if (extensionLen < 1) return;
  const cur = _reader.cursor;
  do {
    const _type = _reader.read16(false);
    const _len = _reader.read16(false);
    rs.push(TLSExtension.create(_type, _reader.slice(_len)));
  } while (_reader.cursor < extensionLen + cur)
  return rs;
}

export class TLSHandshakeMessage extends Packet{
  type: number = 9999;
  len?: number;
  extra?: TLSHandshakeExtra;
  createSummary(field: string): string {
    switch(field){
      case 'type':
        return ``
      case 'len':
    }
    return null;
    // return this.extra && this.extra.createSummary(field);
  }
  public summary(): string {
    return `Handshake Protocol: ${this.getType()}`;
  }
  getType(): string{
    return TLS_HS_MESSAGE_TYPE[this.type] || 'Encrypted Handshake Message'
  }
  static getAlgoType(s: string): string {
    return TLS_CIPHER_SUITES_MAP['0x'+ s]
  }

  static read(pos: PosReader): TLSHandshakeMessage {
    const _reader = pos.getReader();
    const rs =  new TLSHandshakeMessage(pos.getReader());
    if(_reader.left() < 4){
      return rs;
    }
    const [type, len] = pos.getReader().tryTLSMessage();
    switch (type) {
      case 1: {
        rs.type = pos.read8('type');
        rs.len = len;
        pos.slice('len', 3);
        if (len == _reader.left()) {
          const ins = new TLSClientHello();
          const major = _reader.read8();
          const min = _reader.read8();
          ins.type= type;
          ins.version = min;
          ins.random = _reader.readHex(32);
          ins.sessionId = readSession(_reader);
          ins.cipherSuites = readCipherSuite(_reader);
          ins.compressMethod = readTLSMethods(_reader);
          ins.extensions = readTLSExtension(_reader);
          _reader.extra();
          rs.extra = ins;
          break;
        }
      }
        break;
      case 2:{
        rs.type = pos.read8('type');
        rs.len = len;
        pos.slice('len', 3);
        if (len == _reader.left()) {
          const major = _reader.read8();
          const min = _reader.read8();
          const ins = new TLSServerHello();
          ins.type= type;
          ins.version = min;
          ins.random = _reader.readHex(32);
          ins.sessionId = readSession(_reader);
          ins.cipherSuite = _reader.readHex(2);
          ins.compressMethod = _reader.read8();
          ins.extensions = readTLSExtension(_reader);
          _reader.extra();
          rs.extra = ins;
          break;
        }
      }
        break;
    }
    _reader.extra()
    return new TLSHandshakeMessage(_reader);
  }
}
export class TLSHandshake extends Packet {
  getSubType(): string{
    return 'Encrypted Handshake Message';
  }
  summary(): string {
    return 'Handshake Protocol: '
  }
  messages: TLSHandshakeMessage[] = [];
}

export class TLSHandshakeExtra {

}
export class TLSClientHello extends TLSHandshakeExtra {
  type: number;
  version: number;
  random: string;
  sessionId: string;
  cipherSuites: string[];
  compressMethod: TLSMethods;
  extensions: TLSExtension[];
  public getVersion(): string {
    return TLS_MIN_VERSION_MAP[this.version] || 'KnownVersion';
  }
}
export class TLSServerHello extends TLSHandshakeExtra {
  type: number;
  version: number;
  random: string;
  sessionId: string;
  cipherSuite: string;
  compressMethod: number;
  extensions: TLSExtension[];
  public getVersion(): string {
    return TLS_MIN_VERSION_MAP[this.version] || 'KnownVersion';
  }
}

export class TLSCCS extends ComposeRecord {
}
export class TLS extends IPPacket {
  records: TLSRecord[] = [];
  summary(): string {
    return 'Transport Layer Security';
  }
  toString(): string {
    return this.summary();
  }
}

const tryCheckTLS = (_reader: Uint8ArrayReader): [boolean, number] => {
  if (_reader.left() > 5) {
    const [type, major, minor, len] = _reader.tryReadTLS();
    if (type > 19 && type < 25 && major === 3 && minor < 5) {
      return [true, len];
    }
  }
  return [false, 0];
}
export class TLSVisitor implements PVisitor {
  visit(ele: PacketElement): IPPacket {
    const tcp = ele.getPacket() as TCP;
    const { reader } = tcp;
    const data = new TLS(tcp, reader, Protocol.TLS);
    const tcpConnection = tcp.connection;
    const ip = tcp.getIp();
    const stack = tcpConnection.getStackFromStr(ip, tcp.sourcePort);
    const tlsProcess = (_reader: Uint8ArrayReader): void => {
      while (true) {
        if (_reader.left() === 0) {
          break;
        }
        const [its, len] = tryCheckTLS(_reader);
        if (!its) {
          console.log('########error', tcp.getIndex());
          // process.exit(1);
          break;
        }
        if (len + 5 > _reader.left()) {
          stack.addSegment(data.getIndex(), _reader.extra());
          break;
        }
        try {
          const record = TLSRecord.read(_reader);
          tcpConnection.resolveTLS(record);
          
          record.extra instanceof TLSHandshake
          data.records.push(record);
          data.fields.push(record);
          stack.clearSegment();
        } catch (e) {
          console.log(data.getIndex(), len);
          console.error(e);
          break;
        }
      }
    }
    const _reader = stack.rebuildReader(reader);
    stack.clearSegment();
    tlsProcess(_reader);
    return data;
  }
}