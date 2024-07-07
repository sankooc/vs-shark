import { PacketElement, IPPacket, PVisitor, Protocol } from './common';
import { Uint8ArrayReader } from './io';
import { TCP } from './transportLayer';
import { TLS_MIN_VERSION_MAP, TLS_CONTENT_TYPE_MAP, TLS_EXTENSION_MAP, TLS_CIPHER_SUITES_MAP } from './constant';


export class TLSRecord {
  constructor(public readonly type: number, public version: number, public data: Uint8Array) { }
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
  static createHanshake(r: TLSRecord, _reader: Uint8ArrayReader): TLSRecord {
    const hanshake = new TLSHandshake(r);
    do {
      const msg = TLSHandshakeMessage.read(_reader);
      hanshake.messages.push(msg)
      if (!_reader.eof()) {
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
  static create(type: number, version: number, data: Uint8Array): TLSRecord {
    const r = new TLSRecord(type, version, data);
    const _reader = new Uint8ArrayReader(data);
    switch (type) {
      case 20:
        return TLSRecord.createCCS(r, _reader);
      case 21:
        return TLSRecord.createAlert(r, _reader);
      case 22:
        return TLSRecord.createHanshake(r, _reader);
      case 23:
        break;
      case 24:
        return TLSRecord.createHeartbeat(r, _reader);
    }
    return r;
  }
  static read(_reader: Uint8ArrayReader): TLSRecord {
    const type = _reader.read8();
    const major = _reader.read8();
    const minor = _reader.read8();
    const len = _reader.read16(false);
    const _content = _reader.slice(len);
    return TLSRecord.create(type, minor, _content);
  }
};

export class ComposeRecord extends TLSRecord {
  constructor(record: TLSRecord) {
    super(record.type, record.version, record.data);
  }
}

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

export class TLSHandshakeMessage {
  constructor(public content: Uint8Array) { }
  public summary(): string {
    return 'Handshake Protocol: Encrypted Handshake Message';
  }
  static getAlgoType(s: string): string {
    return TLS_CIPHER_SUITES_MAP['0x'+ s]
  }

  static read(_reader: Uint8ArrayReader): TLSHandshakeMessage {
    const content = _reader.extra2();
    if(content.length < 4){
      return new TLSHandshakeMessage(content);
    }
    const [type, len] = _reader.tryTLSMessage();
    switch (type) {
      case 1: {
        _reader.skip(4)
        if (len == _reader.left()) {
          const ins = new TLSClientHello(content);
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
          return ins;
        }
      }
        break;
      case 2:{
        _reader.skip(4)
        if (len == _reader.left()) {
          const major = _reader.read8();
          const min = _reader.read8();
          const ins = new TLSServerHello(content);
          ins.type= type;
          ins.version = min;
          ins.random = _reader.readHex(32);
          ins.sessionId = readSession(_reader);
          ins.cipherSuite = _reader.readHex(2);
          ins.compressMethod = _reader.read8();
          ins.extensions = readTLSExtension(_reader);
          _reader.extra();
          return ins;
        }
      }
        break;
    }
    _reader.extra()
    return new TLSHandshakeMessage(content);
  }
}
export class TLSHandshake extends ComposeRecord {
  messages: TLSHandshakeMessage[] = [];
}
export class TLSClientHello extends TLSHandshakeMessage {
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
  public summary(): string {
      return 'Handshake Protocol: Client Hello';
  }
}
export class TLSServerHello extends TLSHandshakeMessage {
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
  public summary(): string {
      return 'Handshake Protocol: Server Hello';
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
          if (stack.temp) {
            stack.temp = new Uint8Array([...stack.temp, ..._reader.extra()]);
          } else {
            stack.temp = _reader.extra();
          }
          break;
        }
        try {
          const record = TLSRecord.read(_reader);
          if(record instanceof TLSHandshake){
            for(const msg of record.messages){
              if(msg instanceof TLSClientHello){
                tcpConnection.clientHello = msg;
                break;
              }
              if(msg instanceof TLSServerHello){
                tcpConnection.serverHello = msg;
                break;
              }
            }
          }
          if (record) data.records.push(record);
        } catch (e) {
          console.log(data.getIndex(), len);
          console.error(e);
        }
      }
    }
    if (stack.temp) {
      const _content = new Uint8Array([...stack.temp, ...reader.extra()]);
      stack.temp = null;
      const _reader = new Uint8ArrayReader(_content);
      tlsProcess(_reader);
    } else {
      tlsProcess(reader);
    }
    return data;
  }
}