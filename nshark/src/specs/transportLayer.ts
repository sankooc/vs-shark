import { PVisitor, PacketElement, IPPacket, Protocol, TCPStack, TCPConnect, PortProvider, Packet, IField } from '../common';

import { DNSVisitor, NBNSVisitor, DHCPVisitor, HTTPVisitor } from './application';
import { TLSVisitor } from './tls';
import { Uint8ArrayReader, read16, read8 } from '../common/io';
import { IPv4, IPv6, IPPack } from './networkLayer';
import { TCP_OPTION_KIND_MAP, ICMPV6_TYPE_MAP, IGMP_TYPE_MAP, ICMP_TYPE_MAP } from '../common/constant';

export class UDP extends PortProvider {
  sourcePort?: number;
  targetPort?: number;
  len?: number;
  crc?: number;
  toString(): string {
    return `[UDP] ${this.sourcePort} -> ${this.targetPort}`;
  }
  getSourcePort(): number {
    return this.sourcePort;
  }
  getTargetPort(): number {
    return this.targetPort;
  }
  summary(): string {
    return 'User Datagram Protocol';
  }
  createSummary(field: string): string {
    switch (field) {
      case 'sourcePort':
        return `Source Port: ${this.sourcePort}`;
      case 'targetPort':
        return `Destination Port: ${this.targetPort}`;
      case 'len':
        return `Payload length: 0x${this.len.toString(16).padStart(4, '0')} (${this.len})`;
      case 'crc':
        return `Checksum: 0x${this.crc.toString(16).padStart(4, '0')}`;
    }
    return null;
  }
}

export class TCPOptionPanel extends Packet {
  len: number = 0;
  summary(): string {
    return `Options (${this.len} bytes)`;
  }
  getSize(): number {
    return this.len;
  }
}
export class TCPOption extends Packet {
  len?: number;
  kind?: number;
  val?: Uint8Array;
  getOptionKind(): string {
    return TCP_OPTION_KIND_MAP[this.kind];
  }
  summary(): string {
    return `TCP Option - ${this.getOptionKind()}`;
  }
  getSize(): number {
    return this.len;
  }
  createSummary(field: string): string {
    switch (field) {
      case 'kind':
        return `Kind: ${this.getOptionKind()} (${this.kind})`;
      case 'len':
        return `Length: ${this.len || 0}`
      case 'val':
        switch (this.kind) {
          case 2: {
            if (this.val && this.val.length > 1) {
              return `MSS Value: ${read16(this.val, 0, false)}`;
            }
          }
          case 3: {
            if (this.val && this.val.length > 0) {
              return `Shift count: ${read8(this.val, 0)}`;
            }
          }
        }
        return null;
    }
    return null;
  }
}
export class TCP extends UDP {
  extra?: any;
  sequence?: number;
  acknowledge?: number;
  ack?: boolean;
  psh?: boolean;
  rst?: boolean;
  syn?: boolean;
  fin?: boolean;
  unseen?: boolean;
  isDump?: boolean = false;
  options?: TCPOptionPanel;
  connection!: TCPConnect;
  missPre!: boolean;
  hasContent?: boolean;
  // tlsRecords: TLSRecord[] = [];
  getIp(): string {
    const ip = (this.parent as IPPack).getSourceIp().getAddress();
    return ip;
  }
  mess(): any[] {
    const ipPro = this.getIpProvider();
    const sourceIp: string = ipPro.getSourceIp().getAddress();
    const targetIp: string = ipPro.getTargetIp().getAddress();
    const arch = `${sourceIp}:${this.sourcePort}` > `${targetIp}:${this.targetPort}`;
    if (arch) {
      return [arch, sourceIp, this.sourcePort, targetIp, this.targetPort]
    }
    return [arch, targetIp, this.targetPort, sourceIp, this.sourcePort];
  }
  toString(): string {
    if (this.isDump) {
      return `[TCP Retransmission] ${this.sourcePort} -> ${this.targetPort}, ${this.getFlag()}`
    }
    return `[TCP] ${this.sourcePort} -> ${this.targetPort}, ${this.getFlag()}`
  }
  detail(): string {
    return `Transmission Control Protocol, Src Port: ${this.sourcePort}, Dst Prot: ${this.targetPort}, Len: ${this.getProtocolSize()}`;
  }
  public getFlag(): string {
    const its = [];
    this.ack && its.push('ACK');
    this.psh && its.push('PUSH');
    this.rst && its.push('RESET');
    this.syn && its.push('SYN');
    this.fin && its.push('FINISH');
    return '[' + its.join(',') + ']';
  }

  summary(): string {
    return `Transmission Control Protocol, Src Port: ${this.sourcePort}, Dst Prot: ${this.targetPort}, Len: ${this.getProtocolSize()}`
  }
  
  createSummary(field: string): string {
    const v = super.createSummary(field);
    if(v) return v;
    switch (field) {
      case 'sequence':
        return `Sequence: ${this.sequence}`;
      case 'acknowledge':
        return `Acknowledge: ${this.acknowledge}`;
      case 'window':
        return `Window: ${this.extra['window']}`;
      case 'urgent':
        return `Urgent Pointer: ${this.extra['urgent']}`;
    }
    return null;
  }
}

export class ICMP extends IPPacket {
  type: number;
  code: number;
  toString(): string {
    return 'ICMP:' + this.getType();
  }
  public getType(): string {
    const ch = ICMP_TYPE_MAP[this.type];
    if (ch) {
      if (typeof ch === 'string') {
        return ch;
      }
      return ch[this.code] || 'Reserved';
    }
    return 'Reserved';
  }

  createSummary(field: string): string {
    switch (field) {
      case 'type':
        return `Type: ${this.getType()} (${this.type})`;
      case 'code':
        return `Code: ${this.code}`;
    }
    return null;
  }
}
export class IGMP extends IPPacket {
  type: number;
  resp: number;
  address: string;
  public getType() {
    return IGMP_TYPE_MAP[this.type];
  }
  createSummary(field: string): string {
    switch (field) {
      case 'type':
        return `Type: ${this.getType()} (${this.type})`;
      case 'resp':
        return `Max Resp Time: ${this.resp} sec`;
      case 'address':
        return `Multicast Address: ${this.address}`;
    }
    return null;
  }
}

export class ICMPV6 extends ICMP {
  public getType(): string {
    return ICMPV6_TYPE_MAP[this.type];
  }
}
const lann = (mask: number) => {
  const arr = [7, 6, 5, 4, 3, 2, 1, 0];
  return arr.map((off) => (!!((mask >>> off) & 0x01)))
}


export class TCPVisitor implements PVisitor {
  dNSVisitor: DNSVisitor = new DNSVisitor();
  nBNSVisitor: NBNSVisitor = new NBNSVisitor();
  dHCPVisitor: DHCPVisitor = new DHCPVisitor();
  httpVisitor: HTTPVisitor = new HTTPVisitor();
  tlsVisitor: TLSVisitor = new TLSVisitor();
  visit(ele: PacketElement): IPPacket {
    const parent = ele.getPacket();
    const { reader } = parent;
    const data = new TCP(parent, reader, Protocol.TCP);
    data.sourcePort = data.read16('sourcePort', false);
    data.targetPort = data.read16('targetPort', false);
    data.sequence = data.read32('sequence', false);
    data.acknowledge = data.read32('acknowledge', false);
    const h1 = data.read16('head', false);
    const len = (h1 >>> 12) & 0x0f;
    const [cwr, ece, urg, ack, psh, rst, syn, fin] = lann(h1);
    const window = data.read16('window', false);
    data.crc = data.read16('crc', false);
    const urgent = data.read16('urgent', false);
    //https://en.wikipedia.org/wiki/Transmission_Control_Protocol
    const optionSize = len - 5;
    if (optionSize > 0) {
      const optionLen = optionSize * 4;
      const oPanel = new TCPOptionPanel(reader);
      oPanel.len = optionLen;
      data.fields.push(oPanel);
      const _start = oPanel.getReader().cursor;
      while (oPanel.getReader().cursor < _start + oPanel.len) {
        const opt = new TCPOption(oPanel.getReader());
        opt.kind = opt.read8('kind');

        switch (opt.kind) {
          case 2:
          case 3:
          case 4:
          case 5:
          case 8:
          case 28:
          case 29:
          case 30:
            {
        
              opt.len = opt.read8('len');
              opt.val = opt.slice('val', opt.len - 2);
            }
            break;
          default: {

          }
        }
        opt._end();
        oPanel.fields.push(opt);
      }
      oPanel._end();

    }
    // if (len > 5) {
    //     const optionLen = (len - 5) * 4;
    //     const optionBytes = reader.slice(optionLen);
    // }
    data.ack = ack;
    data.psh = psh;
    data.rst = rst;
    data.syn = syn;
    data.fin = fin;
    data.extra = { window, cwr, ece, urg, urgent };

    const tcpConnection = ele.getContext().resolveTCP(data);
    data.connection = tcpConnection;
    if (!tcpConnection) {
      return data;
    }
    const ip = (parent as IPPack).getSourceIp().getAddress();
    const stack = tcpConnection.getStackFromStr(ip, data.sourcePort);


    const tryCheckTLS = (_reader: Uint8ArrayReader): [boolean, number] => {
      if (_reader.left() > 5) {
        const [type, major, minor, len] = _reader.tryReadTLS();
        if (type > 19 && type < 25 && major === 3 && minor < 5) {
          return [true, len];
        }
      }
      return [false, 0];
    }
    const _reader = stack.rebuildReader(reader);
    
    const [ isTLS ] = tryCheckTLS(_reader);
    if (isTLS) {
      tcpConnection.isTLS = true;
      return data.accept(this.tlsVisitor)
    }
    let nextVisitor;
    const method = reader.readSpace(10);
    if (method) {
      switch (method) {
        case 'GET':
        case 'POST':
        case 'PUT':
        case 'DELETE':
        case 'HEAD':
        case 'CONNECT':
        case 'OPTIONS':
        case 'TRACE':
        case 'PATCH':
        case 'NOTIFY':
        case 'HTTP/1.1': {
          nextVisitor = this.httpVisitor;
        }
          break;
      }
    }
    return data.accept(nextVisitor);
  }
}

export class UDPVisitor implements PVisitor {
  dNSVisitor: DNSVisitor = new DNSVisitor();
  nBNSVisitor: NBNSVisitor = new NBNSVisitor();
  dHCPVisitor: DHCPVisitor = new DHCPVisitor();
  visit(ele: PacketElement): IPPacket {
    const parent = ele.getPacket();
    const { reader } = parent;
    const data = new UDP(parent, reader, Protocol.UDP);
    data.sourcePort = data.read16('sourcePort', false)
    data.targetPort = data.read16('targetPort', false)
    data.len = data.read16('len', false)
    data.crc = data.read16('crc', false)
    let visitor;
    // 137 NBNS  138 NBND 139 NBSS
    switch (data.targetPort) {
      case 53:
        visitor = this.dNSVisitor;
        break;
      case 137:
        visitor = this.nBNSVisitor;
        break;
      case 67:
      case 68:
        visitor = this.dHCPVisitor;
        break;
    }
    switch (data.sourcePort) {
      case 53:
        visitor = this.dNSVisitor;
        break;
      case 137:
        visitor = this.nBNSVisitor;
        break;
    }
    return data.accept(visitor);
  }
}

export class ICMPVisitor implements PVisitor {
  visit(ele: PacketElement): IPPacket {
    const parent = ele.getPacket();
    const { reader } = parent;
    const data = new ICMP(parent, reader, Protocol.ICMP);

    data.type = data.read8('type')
    data.code = data.read8('code')
    const checksum = reader.read16(false)
    // Object.assign(this, { type, code, checksum });
    return data;
  }
}

export class IGMPVisitor implements PVisitor {
  visit(ele: PacketElement): IPPacket {

    const parent = ele.getPacket();
    const { reader } = parent;
    const data = new IGMP(parent, reader, Protocol.IGMP);
    data.type = data.read8('type')
    data.resp = data.read8('resp')
    const checksum = reader.read16(false)
    data.address = data.readDec('address', 4, '.')
    return data;
  }
}

export class ICMPV6Visitor implements PVisitor {
  visit(ele: PacketElement): IPPacket {
    const parent = ele.getPacket();
    const { reader } = parent;
    const data = new ICMPV6(parent, reader, Protocol.ICMP);

    data.type = data.read8('type');
    data.code = data.read8('code');
    // reader.read16(); //crc
    // reader.skip(4); //flag
    // const target = reader.readHex(16, ':')
    return data;
  }

}