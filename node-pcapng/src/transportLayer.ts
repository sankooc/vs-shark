import { PVisitor, BasicElement, IPPacket, Protocol, TCPStack, TCPConnect } from './common';

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
        return `Transmission Control Protocol, Src Port: ${this.sourcePort}, Dst Prot: ${this.targetPort}, Len: ${this.packet.length}`;
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
    visit(ele: BasicElement): IPPacket {
        const { name, readerCreator, content } = ele;
        const prefix = name + '/tcp'
        const reader = readerCreator.createReader(content, prefix, false);
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
        ele.log('port', sourcePort, targetPort);
        ele.log('sequence', sequence, acknowledge);
        ele.log('extra', len, window, checksum);
        if (len > 5) {
            const optionLen = (len - 5) * 4;
            const optionBytes = reader.slice(optionLen);
        }
        const payload = reader.extra2();

        const data = new TCP(ele.packet, payload, Protocol.TCP);
        data.sourcePort = sourcePort;
        data.targetPort = targetPort;
        data.sequence = sequence;
        data.acknowledge = acknowledge;
        data.ack = ack;
        data.psh = psh;
        data.rst = rst;
        data.syn = syn;
        data.fin = fin;
        data.payload = payload;
        data.extra = { window, cwr, ece, urg, urgent };
        ele.context.resolveTCP(data);
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

        if(nextVisitor) {
            return data.createSubElement(prefix, ele).accept(nextVisitor);
        }
        return data;
    }
}

export class UDPVisitor implements PVisitor {
    dNSVisitor: DNSVisitor = new DNSVisitor();
    nBNSVisitor: NBNSVisitor = new NBNSVisitor();
    dHCPVisitor: DHCPVisitor = new DHCPVisitor();
    visit(ele: BasicElement): IPPacket {
        const { name, readerCreator, content } = ele;
        const prefix = name + '/udp';
        const reader = readerCreator.createReader(content, prefix, false);
        const sourcePort = reader.read16(false)
        const targetPort = reader.read16(false)
        const len = reader.read16(false);
        const checksum = reader.read16();
        ele.log('udp', sourcePort, targetPort, len, checksum);
        const payload = reader.slice(len - 8);
        const data = new UDP(ele.packet, payload, Protocol.UDP);
        data.sourcePort = sourcePort;
        data.targetPort = targetPort;
        data.payload = payload;
        let visitor = null;
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
        if (visitor) {
            return data.createSubElement(prefix, ele).accept(visitor);
        }
        return data;
    }
}

export class ICMPVisitor implements PVisitor {
    visit(ele: BasicElement): IPPacket {
        const { name, readerCreator, content } = ele;
        const prefix = name + '/ICMP';
        const reader = readerCreator.createReader(content, prefix, false);
        const type = reader.read8()
        const code = reader.read8()
        const checksum = reader.read16(false)
        Object.assign(this, { type, code, checksum });
        const data = new ICMP(ele.packet, null, Protocol.ICMP);
        data.type = type;
        data.code = code;
        return data;
    }
}
export class IGMPVisitor implements PVisitor {
    visit(ele: BasicElement): IPPacket {
        const { name, readerCreator, content } = ele;
        const prefix = name + '/igmp';
        const reader = readerCreator.createReader(content, prefix, false);
        const type = reader.read8()
        const resp = reader.read8()
        const checksum = reader.read16(false)
        const address = reader.readDec(4, '.')
        const data = new IGMP(ele.packet, null, Protocol.IGMP);
        data.type = type;
        data.resp = resp;
        data.address = address;
        return data;
    }
}

export class ICMPV6Visitor implements PVisitor {
    visit(ele: BasicElement): IPPacket {
        const { name, readerCreator, content } = ele;
        const prefix = name + '/ICMP6';
        const reader = readerCreator.createReader(content, prefix, false);
        const type = reader.read8();
        const code = reader.read8();
        const data = new ICMPV6(ele.packet, null, Protocol.ICMP);
        data.type = type;
        data.code = code;
        // reader.read16(); //crc
        // reader.skip(4); //flag
        // const target = reader.readHex(16, ':')
        return data;
    }

}