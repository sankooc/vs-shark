import { PVisitor, PacketElement, IPPacket, Protocol, TCPStack, TCPConnect } from './common';

import { DNSVisitor, NBNSVisitor, DHCPVisitor, HTTPVisitor } from './application';
import { IPv4, IPv6 } from './networkLayer';

export class UDP extends IPPacket {
    extra: any;
    payload: Uint8Array
    sourcePort: number;
    targetPort: number;
    toString(): string {
        return `[UDP] ${this.sourcePort} -> ${this.targetPort}`;
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
        return `Transmission Control Protocol, Src Port: ${this.sourcePort}, Dst Prot: ${this.targetPort}, Len: ${this.getSize()}`;
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
        return 'ICMP';
    }
}
export class IGMP extends IPPacket {
    type: number;
    resp: number;
    address: string;
}

export class ICMPV6 extends ICMP {

}
const lann = (mask: number) => {
    const arr = [7, 6, 5, 4, 3, 2, 1, 0];
    return arr.map((off) => (!!((mask >>> off) & 0x01)))
}

//https://en.wikipedia.org/wiki/Transport_Layer_Security
const minorVersion = {
    0: 'ssl 3.0',
    1: 'tls 1.0',
    2: 'tls 1.1',
    3: 'tls 1.2',
    4: 'tls 1.3',
}
const types = {
    20: 'ChangeCipherSpec',
    21: 'Alert',
    22: 'Handshake',
    23: 'Application',
    24: 'Heartbeat',
}
export class TCPVisitor implements PVisitor {
    dNSVisitor: DNSVisitor = new DNSVisitor();
    nBNSVisitor: NBNSVisitor = new NBNSVisitor();
    dHCPVisitor: DHCPVisitor = new DHCPVisitor();
    httpVisitor: HTTPVisitor = new HTTPVisitor();
    visit(ele: PacketElement): IPPacket {
        
        const parent = ele.getPacket();
        const { reader } = parent;
        const data = new TCP(parent, reader, Protocol.TCP);


        const sourcePort = reader.read16(false);
        const targetPort = reader.read16(false);
        const sequence = reader.read32(false);
        const acknowledge = reader.read32(false);
        const h1 = reader.read16(false);
        const len = (h1 >>> 12) & 0x08;
        const [cwr, ece, urg, ack, psh, rst, syn, fin] = lann(h1);
        const window = reader.read16(false);
        const checksum = reader.read16(false);
        const urgent = reader.read16(false);

        if (len > 5) {
            const optionLen = (len - 5) * 4;
            const optionBytes = reader.slice(optionLen);
        }
        data.sourcePort = sourcePort;
        data.targetPort = targetPort;
        data.sequence = sequence;
        data.acknowledge = acknowledge;
        data.ack = ack;
        data.psh = psh;
        data.rst = rst;
        data.syn = syn;
        data.fin = fin;
        data.extra = { window, cwr, ece, urg, urgent };
        // ele.context.resolveTCP(data);
        
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
                case 'HTTP/1.1':{
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


        const sourcePort = reader.read16(false)
        const targetPort = reader.read16(false)
        const len = reader.read16(false);
        const checksum = reader.read16();

        data.sourcePort = sourcePort;
        data.targetPort = targetPort;
        let visitor = null;
        // 137 NBNS  138 NBND 139 NBSS
        switch (targetPort) {
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
        switch (sourcePort) {
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

        const type = reader.read8()
        const code = reader.read8()
        const checksum = reader.read16(false)
        // Object.assign(this, { type, code, checksum });
        data.type = type;
        data.code = code;
        return data;
    }
}

export class IGMPVisitor implements PVisitor {
    visit(ele: PacketElement): IPPacket {
        
        const parent = ele.getPacket();
        const { reader } = parent;
        const data = new IGMP(parent, reader, Protocol.IGMP);

        const type = reader.read8()
        const resp = reader.read8()
        const checksum = reader.read16(false)
        const address = reader.readDec(4, '.')

        data.type = type;
        data.resp = resp;
        data.address = address;
        return data;
    }
}

export class ICMPV6Visitor implements PVisitor {
    visit(ele: PacketElement): IPPacket {
        const parent = ele.getPacket();
        const { reader } = parent;
        const data = new ICMPV6(parent, reader, Protocol.ICMP);

        const type = reader.read8();
        const code = reader.read8();
        data.type = type;
        data.code = code;
        // reader.read16(); //crc
        // reader.skip(4); //flag
        // const target = reader.readHex(16, ':')
        return data;
    }

}