import { PVisitor, PacketElement, IPPacket, Protocol, TCPStack, TCPConnect, TCPOption, PortProvider } from './common';

import { DNSVisitor, NBNSVisitor, DHCPVisitor, HTTPVisitor } from './application';
import { TLSVisitor } from './tls';
import { Uint8ArrayReader } from './io';
import { IPv4, IPv6, IPPack } from './networkLayer';
import { ICMPV6_TYPE_MAP, IGMP_TYPE_MAP, ICMP_TYPE_MAP } from './constant';

export class UDP extends PortProvider {
    extra: any;
    sourcePort: number;
    targetPort: number;
    toString(): string {
        return `[UDP] ${this.sourcePort} -> ${this.targetPort}`;
    }
    getSourcePort(): number {
        return this.sourcePort;
    }
    getTargetPort(): number {
        return this.targetPort;
    }
}
export class TCP extends UDP {
    sequence: number;
    acknowledge: number;
    ack: boolean;
    psh: boolean;
    rst: boolean;
    syn: boolean;
    fin: boolean;
    unseen: boolean;
    isDump: boolean = false;
    options!: TCPOption[];
    connection!: TCPConnect;
    missPre!: boolean;
    hasContent: boolean;
    // tlsRecords: TLSRecord[] = [];
    getIp(): string {
        const ip = (this.parent as IPPack).source;
        return ip;
    }
    mess(): any[] {
        let sourceIp;
        let targetIp;
        let rs = this.getProtocal(Protocol.IPV4);
        if (rs && rs instanceof IPv4) {
            sourceIp = (rs as IPv4).source;
            targetIp = (rs as IPv4).target;
        }
        rs = this.getProtocal(Protocol.IPV6);
        if (rs instanceof IPv6) {
            sourceIp = (rs as IPv6).source;
            targetIp = (rs as IPv6).target;
        }
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
}

export class ICMP extends IPPacket {
    type: number;
    code: number;
    toString(): string {
        return 'ICMP:' + this.getType();
    }
    public getType(): string {
        const ch = ICMP_TYPE_MAP[this.type];
        if(ch){
            if(typeof ch === 'string'){
                return ch;
            }
            return ch[this.code] || 'Reserved';
        }
        return 'Reserved';
    }
}
export class IGMP extends IPPacket {
    type: number;
    resp: number;
    address: string;
    public getType(){
        return IGMP_TYPE_MAP[this.type];
    }
}

export class ICMPV6 extends ICMP {

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
        const checksum = data.read16('checksum', false);
        const urgent = data.read16('urgent', false);
        //https://en.wikipedia.org/wiki/Transmission_Control_Protocol
        const optionSize = len - 5;
        if (optionSize > 0) {
            const optionLen = optionSize * 4;
            data.options = data.readTcpOption(optionLen);
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
        const ip = (parent as IPPack).source;
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
        if (stack.temp) {
            const _content = new Uint8Array([...stack.temp, ...reader.extra2()]);
            const _reader = new Uint8ArrayReader(_content);
            const [isTLS, len] = tryCheckTLS(_reader);
            if (isTLS) {
                tcpConnection.isTLS = true;
                return data.accept(this.tlsVisitor)
            }
        } else {
            const [isTLS, len] = tryCheckTLS(reader);
            if (isTLS) {
                tcpConnection.isTLS = true;
                return data.accept(this.tlsVisitor)
            }
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
        const len = reader.read16(false);
        const checksum = reader.read16();

        let visitor = null;
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