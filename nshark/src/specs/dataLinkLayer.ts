import { PVisitor, PacketElement, IPPacket, Protocol } from '../common';
import { IPv4Visitor, IPv6Visitor, ARPVisitor } from './networkLayer';
import { SLL_TYPE, linktypeMap, etypeMap, PPP_CODE_MAP, PPP_DLL_NUMBER_MAP } from '../common/constant';

export class DataPacket extends IPPacket {
  target: string;
  source: string;
  type: string;
  toString(): string {
    return `Ethernet II, src: () ${this.source} Dst: () ${this.target}`;
  }
  public getProtocolType(): string {
    const code = `0x${this.type.toUpperCase()}`
    return etypeMap[code];
  }
  createSummary(field: string): string {
    switch (field) {
      case 'source':
        return `Source MAC Address: (${this.source})`;
      case 'target':
        return `Destination MAC Address: (${this.target})`;
      case 'type':
        const code = `0x${this.type.toUpperCase()}`
        return `Type : ${this.getProtocolType()} (${code})`;
    }
    return null;
  }
}
export class SLLPacket extends IPPacket {
  type!: number;
  ltype!: number;
  source: string;
  ptype: string;
  toString(): string {
    return `Linux Cooked capture`;
  }
  public getPacketType(): string {
    return SLL_TYPE[this.type];
  }
  public getLinkLaylerAddressType(): string {
    return linktypeMap[this.ltype];
  }
  public getProtocolType(): string {
    const code = `0x${this.ptype.toUpperCase()}`
    return etypeMap[code];
  }
  createSummary(field: string): string {
    switch (field) {
      case 'type':
        return `Type: ${this.getPacketType()}`;
      case 'ltype':
        return `Linklayer Adress: ${this.getLinkLaylerAddressType()}`;
      case 'source':
        return `Source MAC Address: (${this.source})`;
      case 'ptype':
        return `Protocol Type: ${this.getProtocolType()} (${this.ptype})`;
    }
    return null;
  }
}
export class PPPoESS extends IPPacket {
  version: number;
  type: number;
  code: number;
  sessionId: number;
  payload: number;
  _protocol: number;
  public getCode(): string {
    return PPP_CODE_MAP[this.code];
  }
  public getPOEProtocol(): string {
    const type = this._protocol.toString(16).padStart(4, '0');
    return PPP_DLL_NUMBER_MAP[type]
  }
  toString(): string {
    return this.summary();
  }
  summary(): string {
    return 'Point-over-Ethernet';
  }
  createSummary(field: string): string {
    const p = this;
    switch (field) {
      case 'head':
        return 'Version: 1, Type: 1';
      case 'code':
        return `Code: ${p.getCode()} (${p.code})`;
      case 'session':
        return `Session Id: 0x${p.sessionId.toString(16)}`;
      case 'payload':
        return `Payload Length: ${p.payload}`;
      case 'protocol':
        return `Protocol: ${p.getPOEProtocol()} (0x${p.protocol.toString(16)})`;
    }
    return null;
  }
}
export class PPPoE_Session_Stage_Visitor implements PVisitor {
  mapper: Map<string, PVisitor> = new Map();
  constructor() {
    this.mapper.set('0021', new IPv4Visitor());
    this.mapper.set('0057', new IPv6Visitor());
  }
  visit(ele: PacketElement): IPPacket {
    //https://www.iana.org/assignments/ppp-numbers/ppp-numbers.xml
    //https://www.h3c.com/en/Support/Resource_Center/EN/Home/Switches/00-Public/Trending/Technology_White_Papers/PPPoE_Technology_White_Paper-6W100/
    const parent = ele.getPacket();
    const { reader } = parent;
    const data = new PPPoESS(parent, reader, Protocol.PPPOESS);

    const h = data.read8('head');
    data.version = h >> 4;
    data.type = h & 0x0f;
    data.code = data.read8('code');
    data.sessionId = data.read16('session', false);
    data.payload = data.read16('payload', false);
    data._protocol = data.read16('protocol', false);
    if (data.code === 0) { //SESSION 
      const type = data._protocol.toString(16).padStart(4, '0');
      return data.accept(this.mapper.get(type));
    }
    return data;
  }

}

export class DataLaylerVisitor implements PVisitor {
  mapper: Map<string, PVisitor> = new Map();
  constructor() {
    this.mapper.set('0800', new IPv4Visitor());
    this.mapper.set('86dd', new IPv6Visitor());
    this.mapper.set('0806', new ARPVisitor());
    this.mapper.set('8864', new PPPoE_Session_Stage_Visitor());
  }
  visit(ele: PacketElement): IPPacket {
    const parent = ele.getPacket();
    const { reader } = parent;
    const data = new DataPacket(parent, reader, Protocol.MAC);
    data.target = data.readHex('target', 6, ':');
    data.source = data.readHex('source', 6, ':');
    data.type = data.readHex('type', 2, '');
    //8864 893a 890d c0a8
    return data.accept(this.mapper.get(data.type));
  }
}

export class SLLVisitor implements PVisitor {
  mapper: Map<string, PVisitor> = new Map();
  constructor() {
    this.mapper.set('0800', new IPv4Visitor());
    this.mapper.set('86dd', new IPv6Visitor());
    this.mapper.set('0806', new ARPVisitor());
    this.mapper.set('8864', new PPPoE_Session_Stage_Visitor());
  }
  visit(ele: PacketElement): IPPacket {
    const parent = ele.getPacket();
    const { reader } = parent;
    const data = new SLLPacket(parent, reader, Protocol.SLL);
    data.type = data.read16('type', false);
    data.ltype = data.read16('ltype', false);
    const len = data.read16('len', false);
    data.source = data.readHex('source', len, ':');
    reader.skip(2);
    data.ptype = data.readHex('ptype', 2, '');
    return data.accept(this.mapper.get(data.ptype));
  }

}