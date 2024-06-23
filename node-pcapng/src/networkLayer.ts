import { PVisitor, BasicElement, IPPacket,Protocol } from './common';
import { ipProtocolMap } from './constant';
import {TCPVisitor, UDPVisitor, ICMPVisitor, IGMPVisitor} from './transportLayer';

export class IPPack extends IPPacket {
    extra: any;
    source: string;
    target: string;

}
export class IPv4 extends IPPack {
    version: number;
    totalLen: number;
    identification: number;
    ttl: number;
    ipprotocol: string;
    toString(): string {
        return `Internet Protocol Version : ${this.version} length: ${this.totalLen}`;
    }
}

export class IPv6 extends IPPack {
    nextHeader: number;
    hop: number;
    toString(): string {
        return `Internet Protocol Version : 6 src: ${this.source} dst: ${this.target}`;
    }
}

export class ARP extends IPPacket{
    oper: number;//1 request 2 reply
    senderMac: string;
    senderIp: string;
    targetMac: string;
    targetIp: string;
    hardwareType:number;
    protocolType: number;
    hardwareSize: number;
    protocolSize: number;
    toString(): string {
        if(this.oper === 1){
            if(this.senderIp === this.targetIp){
                return `ARP Announcement for ${this.senderIp}`;
            }
            if(this.senderIp === '0.0.0.0'){
                return `who has ${this.targetIp}? (ARP probe)`;
            }
            return `who has ${this.targetIp}? tell ${this.senderIp}`;
        }
        return 'ARP';
    }
}
// https://en.wikipedia.org/wiki/Internet_Protocol_version_4
export class IPv4Visitor implements PVisitor {
    mapper: Map<number, PVisitor> = new Map();
    constructor(){
        this.mapper.set(6, new TCPVisitor())
        this.mapper.set(17, new UDPVisitor())
        this.mapper.set(1, new ICMPVisitor())
        this.mapper.set(2, new IGMPVisitor())
        //https://en.wikipedia.org/wiki/List_of_IP_protocol_numbers
    }
    visit(ele: BasicElement): IPPacket {
        const { name, readerCreator, content } = ele;
        const prefix = name + '/ip';
        const reader = readerCreator.createReader(content, prefix, false);
        const cv = reader.read8();
        const version = cv >>> 4;
        const headLenth = cv & 0x0f;
        const tos = reader.read8();
        const totalLen = reader.read16();
        const identification = reader.read16();
        const flag = reader.read16() >>> 13
        const ttl = reader.read8();
        const protocol = reader.read8()
        const headCRC = reader.read16()
        // const source = reader.read32Hex()
        // const target = reader.read32Hex()
        const source = reader.readIp();
        const target = reader.readIp();
        if (headLenth > 5) {
            reader.skip((headLenth - 5) * 4)
        }
        const packet = reader.extra()
        
        const data = new IPv4(ele.packet, packet, Protocol.IPV4);
        data.source = source;
        data.target = target;
        data.version = version;
        data.totalLen = totalLen;
        data.identification = identification;
        data.ttl = ttl;
        data.ipprotocol = ipProtocolMap[protocol];
        data.extra = {cv,headLenth,tos,flag,headCRC }

        const nextVisitor = this.mapper.get(protocol);
        if(nextVisitor) {
            return data.createSubElement(prefix, ele).accept(nextVisitor)
        }
        return data;
    }
}

//https://en.wikipedia.org/wiki/Address_Resolution_Protocol
export class ARPVisitor implements PVisitor {
    visit(ele: BasicElement): IPPacket {
        const { name, readerCreator, content } = ele;
        const prefix = name + '/arp';
        const reader = readerCreator.createReader(content, prefix, false);
        //This field specifies the network link protocol type. Example: Ethernet is 1
        const htype = reader.read16(false);
        // This field specifies the internetwork protocol for which the ARP request is intended
        const ptype = reader.read16(false);
        //Length (in octets) of a hardware address. Ethernet address length is 6.
        const hlen = reader.read8();
        //Length (in octets) of internetwork addresses. The internetwork protocol is specified in PTYPE. Example: IPv4 address length is 4.
        const plen = reader.read8();
        //Specifies the operation that the sender is performing: 1 for request, 2 for reply.
        const oper = reader.read16(false);
        const senderMac = reader.readHex(6, ':');
        const senderIp = reader.readIp()
        const targetMac = reader.readHex(6, ':');
        const targetIp = reader.readIp()
        const data = new ARP(ele.packet, null, Protocol.ARP);
        data.hardwareType = htype;
        data.hardwareSize = hlen;
        data.protocolType = ptype;
        data.protocolSize = plen;
        data.oper = oper;
        data.senderMac = senderMac;
        data.senderIp = senderIp;
        data.targetMac = targetMac;
        data.targetIp = targetIp;
        return data;
    }
}

export class IPv6Visitor implements PVisitor {
    visit(ele: BasicElement): IPPacket {
        const { name, readerCreator, content } = ele;
        const prefix = name + '/ipv6';
        const reader = readerCreator.createReader(content, prefix, false);
        reader.read32();
        const plen = reader.read16(false);
        const header = reader.read8();
        const hopLimit = reader.read8();
        //https://en.wikipedia.org/wiki/IPv6_address
        const sourceip = reader.readHex(16, ':')
        const targetip = reader.readHex(16, ':')

        ele.log('ipv6', plen, header, hopLimit)

        Object.assign(this, {plen, header, hopLimit,sourceip ,targetip})

        const arr = reader.extra();
        const data = new IPv6(ele.packet, arr, Protocol.IPV6);
        data.source = sourceip;
        data.target = targetip;
        data.nextHeader = header;
        data.hop = hopLimit;
        return data;
    }
}

// https://en.wikipedia.org/wiki/EtherType
// export default {
//     createVisitor(parent: AbstractVisitor, type: string): AbstractVisitor {
//         switch (type) {
//             case '0800':
//                 return new IPv4Visitor(parent, 'ipv4', type);
//             case '86dd':
//                 return new IPv6Visitor(parent, 'ipv6', type);
//             case '0806':
//                 return new ARPVisitor(parent, 'arp', type);
//             case '8035':
//         }
//         return new BasicVisitor(parent,'unknown', type);
//     }
// }