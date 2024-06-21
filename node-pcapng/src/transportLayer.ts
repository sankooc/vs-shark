import { PVisitor, BasicElement, IPPacket, Protocol,TCPStack,TCPConnect } from './common';

import { DNSVisitor, NBNSVisitor, DHCPVisitor } from './application';
import { IPv4, IPv6 } from './networkLayer';

export class UDP extends IPPacket {
    extra: any;
    payload: Uint8Array
    sourcePort: number;
    targetPort: number;
    toString(): string{
        return `[UDP] ${this.sourcePort} -> ${this.targetPort}`;
    }
}
export class TCP extends UDP {
    sequence: number;
    acknowledge: number;
    ack: boolean;
    psh: boolean;
    rsp: boolean;
    syn: boolean;
    fin: boolean;
    unseen: boolean;
    isDump: boolean = false;
    mess(): any[] {
        let sourceIp;
        let targetIp;
        let rs = this.getProtocal(Protocol.IPV4);
        if(rs && rs instanceof IPv4){
            sourceIp = (rs as IPv4).source;
            targetIp = (rs as IPv4).target;
        }
        rs = this.getProtocal(Protocol.IPV6);
        if(rs instanceof IPv6){
            sourceIp = (rs as IPv6).source;
            targetIp = (rs as IPv6).target;
        }
        // sourceIp = `${sourceIp}:${this.sourcePort}`
        // targetIp = `${targetIp}:${this.targetPort}`
        const arch = `${sourceIp}:${this.sourcePort}` > `${targetIp}:${this.targetPort}`;
        if(arch){
            return [arch, sourceIp, this.sourcePort, targetIp,this.targetPort]
        }
        return [arch, targetIp,this.targetPort,sourceIp, this.sourcePort];
    }
    toString(): string{
        return `[TCP] ${this.sourcePort} -> ${this.targetPort}`;
    }
    createSubElement(name: string, parent: BasicElement): BasicElement {

        const noContent = this.ack && !this.psh && this.packet.length < 10;
        const [arch, ip1, port1, ip2, port2] = this.mess();
        const key = `${ip1}${port1}-${ip2}${port2}`;
        let connect = parent.resolver.tcpCache.get(key);
        if(!connect) {
            if(noContent) return; // 
            connect = new TCPConnect(ip1, port1, ip2, port2);
            parent.resolver.tcpCache.set(key, connect);
        }
        const sequence = this.sequence;
        const nextSequence = this.sequence + this.packet.length;
        const stack = connect.getStack(arch);
        const dump = stack.checkDump(sequence, nextSequence);
        this.isDump = dump;
        connect.count += 1;
        connect.total += this.getProtocal(Protocol.ETHER).packet.length;
        connect.tcpSize += this.packet.length;
        if(dump) {
            // console.log('dump:', this.getIndex())
            return null;
        }
        connect.tcpUse += this.packet.length;
        connect.countUse += 1;
        // if(this.getIndex() == 72){
        //     console.log('--', this.packet.length)
        // }
        stack.sequence = sequence;
        stack.next = nextSequence;
        const stackRec = connect.getStack(!arch);
        stackRec.ack = this.acknowledge;
      

        // if(arch) {
        //     const stack = connect.getStack(arch);
        //     stack.sequence = sequence;
        //     stack.ack = nextSequence;
        //     if(connect.sequence1 == sequence && connect.ack1 == nextSequence){

        //     }
        //     connect.sequence1 = this.sequence;
        //     connect.ack1 = this.sequence + this.packet;
        // } else {
        //     connect.sequence2 = this.sequence;
        //     connect.ack2 = this.sequence + this.packet;
        // }
        if(this.ack){

        }
        if(this.ack && !this.psh){
            if(this.packet.length > 10){
                const len = this.getProtocal(Protocol.ETHER).packet.length;
            }
        }
        if(this.psh){
            // if(arch){
            //     connect
            // }
        }
        return super.createSubElement(name, parent);
    }
}

export class ICMP extends IPPacket {
    type: number;
    code: number;
}
export class IGMP extends IPPacket {
    type: number;
    resp: number;
    address: string;
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
        const [cwr, ece, urg, ack, psh, rsp, syn, fin] = lann(h1);
        const window = reader.read16(false);
        const checksum = reader.read16(false);
        const urgent = reader.read16(false);
        ele.log('port', sourcePort, targetPort);
        ele.log('sequence', sequence, acknowledge);
        ele.log('extra', len, window, checksum);
        ele.log('flag', cwr, ece, urg, ack, psh, rsp, syn, fin);
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
        data.rsp = rsp;
        data.syn = syn;
        data.fin = fin;
        data.payload = payload;
        data.extra = { window, cwr, ece, urg, urgent };

        data.createSubElement(prefix, ele)
        
        // let visitor = null;
        // if (payload[0] == 23) {
        //     if (payload[1] == 3 && minorVersion[payload[2]]) {
        //         console.log('tls', data.getIndex())
        //     }
        // }

        // switch (targetPort) {
        //     case 53:
        //         visitor = this.dNSVisitor;
        //         break;
        //     case 137:
        //         visitor = this.nBNSVisitor;
        //         break;
        //     case 67:
        //     case 68:
        //         visitor = this.dHCPVisitor;
        //         break;
        // }
        // if (visitor) {
        //     return data.createSubElement(prefix, ele).accept(visitor);
        // }
        return data;
    }
}

export class UDPVisitor implements PVisitor {
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
        return data;
    }
}

export class ICMPVisitor implements PVisitor {
    visit(ele: BasicElement): IPPacket {
        const { name, readerCreator, content } = ele;
        const prefix = name + '/icmp';
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
// export default {
//     createVisitor(parent: AbstractVisitor, protocol: number): AbstractVisitor {
//         //https://en.wikipedia.org/wiki/List_of_IP_protocol_numbers
//         switch (protocol) {
//             case 6://TCP
//                 return new TCPVisitor(parent, 'tcp', protocol);
//             case 17://UDP
//                 return new UDPVisitor(parent, 'udp', protocol);
//             case 1://ICMP
//                 return new ICMPVisitor(parent, 'icmp', protocol)
//             case 2: // IGMP
//                 return new IGMPVisitor(parent, 'igmp', protocol)
//             case 41: // ENCAP
//             case 89: // OSPF
//             case 132:// SCTP
//         }

//         return new BasicVisitor(parent, 'default', protocol);
//     }
// }