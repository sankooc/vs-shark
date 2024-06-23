import { BasicElement, IPPacket, PVisitor, Protocol } from './common';
import { UDP } from './transportLayer';

export class NBNS extends IPPacket {
    transactionId: string;
    flag: number;
    question: number;
    answer: number;
    authority: number;
    addtional: number;
    payload: Uint8Array;
    toString(): string {
        return `NBNS transaction id: ${this.transactionId}`;
    }
}

export class Query {
    name: string;
    type: number;
    cls: number;
    constructor(name: string, type: number,cls: number){
        this.name = name;
        this.type = type;
        this.cls = cls;
    }
}
export class Answer {
    name: number;
    type: number;
    cls: number;
    ttl: number;
    len: number;
    host: string;
}
export class DNS extends NBNS {
    domain: string;
    type: number;
    isAnswer: boolean;
    queries: Query[] = [];
    toString(): string {
        return `DNS transaction id: ${this.transactionId}`;
    }
}

export class DHCP extends IPPacket {
    transactionId: number;
    clientAddress: string;
    yourAddress: string;
    nextServerAddress: string;
    relayAddress: string;
    macAddress: string;
    toString(): string {
        return `DHCP transaction id: ${this.transactionId}`;
    }
}

export class DNSVisitor implements PVisitor {
    visit(ele: BasicElement): IPPacket {
        const { name, readerCreator, content } = ele;
        const prefix = name + '/dns';
        const reader = readerCreator.createReader(content, prefix, false);
        const transactionId = reader.readHex(2)
        const flag = reader.read16(false);
        const question = reader.read16(false);
        const answer = reader.read16(false);
        const authority = reader.read16(false);
        const addtional = reader.read16(false);
        const data = new DNS(ele.packet, null, Protocol.DNS);
        data.transactionId = transactionId;
        data.flag = flag;
        data.question = question;
        data.answer = answer;
        data.authority = authority;
        data.addtional = addtional;
        const udp: UDP = ele.packet.getProtocal(Protocol.UDP) as UDP;
        data.isAnswer = udp.targetPort === 53;
        for(let i = 0; i < data.question; i += 1) {
            const domain = reader.readDNSQuery();
            const type = reader.read16(false);
            const cls = reader.read16(false);
            data.queries.push(new Query(domain, type, cls));
        }
        for(let i = 0; i < data.answer; i += 1){
            const ans = new Answer();
            ans.name = reader.read16(false);
            ans.type = reader.read16(false);
            ans.cls = reader.read16(false);
            ans.ttl = reader.read32(false);
            ans.len = reader.read16(false);
            if(ans.type === 5) {
                const [domain, id] = reader.readDNSAnswer(ans.len);
                ans.host = domain;
            } else {
                ans.host = reader.readIp();
            }
        }
        // if(data.answer > 0){
        //     process.exit(0);
        // }
        // if (this.destPort === 53) {
        //     const domain = reader.readDNSQuery();
        //     const type = reader.read16(false)
        //     const claz = reader.read16(false)
        //     // Object.assign(this, { domain, type, claz });
        //     ele.log('dns', type, claz, domain)
        // }

        return data;
    }
}

export class NBNSVisitor implements PVisitor {
    visit(ele: BasicElement): IPPacket {
        const { name, readerCreator, content } = ele;
        const prefix = name + '/nbns';
        const reader = readerCreator.createReader(content, prefix, false);
        const transactionId = reader.readHex(2)
        const flag = reader.read16(false);
        const question = reader.read16(false);
        const answer = reader.read16(false);
        const authority = reader.read16(false);
        const addtional = reader.read16(false);
        const data = new NBNS(ele.packet, null, Protocol.NBNS);
        data.transactionId = transactionId;
        data.flag = flag;
        data.question = question;
        data.answer = answer;
        data.authority = authority;
        data.addtional = addtional;
        data.payload = reader.extra();
        return data;
    }
}

export class DHCPVisitor implements PVisitor {
    // https://en.wikipedia.org/wiki/Dynamic_Host_Configuration_Protocol
    visit(ele: BasicElement): IPPacket {
        const { name, readerCreator, content } = ele;
        const prefix = name + '/nbns';
        const reader = readerCreator.createReader(content, prefix, false);
        const opt = reader.read8();
        const htype = reader.read8();
        const hlen = reader.read8();
        const hops = reader.read8();
        const transactionId = reader.read32();
        const sec = reader.read16(false);
        const flag = reader.read16(false);
        const clientAddress = reader.read32Hex();
        const yourAddress = reader.read32Hex();
        const nextServerAddress = reader.read32Hex();
        const relayAddress = reader.read32Hex();
        const macAddress = reader.readHex(6, ':');
        reader.skip(10)//padding
        reader.skip(64)//sname
        reader.skip(128)//file
        const magicCookie = reader.read32();
        ele.log('dhcp:', opt, htype, clientAddress)
        const data = new DHCP(ele.packet, null, Protocol.DHCP);
        data.transactionId = transactionId;
        data.clientAddress = clientAddress;
        data.yourAddress = yourAddress;
        data.nextServerAddress = nextServerAddress;
        data.relayAddress = relayAddress;
        data.macAddress = macAddress;
        return data;
    }
}

// export default {
//     createVisitor(sub: AbstractVisitor, sourcePort: number, destPort: number): AbstractVisitor {
//         //https://en.wikipedia.org/wiki/List_of_IP_protocol_numbers
//         switch (destPort) {
//             case 53://DNS
//                 return new DNSVisitor(sub, 'dns',sourcePort, destPort);
//             case 137://NBNS
//                 return new NBNSVisitor(sub, 'nbns', sourcePort, destPort)
//             case 67://DHCP
//             case 68://DHCP
//                 return new DHCPVisitor(sub, 'dhcp', sourcePort, destPort);
//             // // case 80:
//             default:
//                 return new PayloadVisitor(sub, 'payload', sourcePort, destPort);
//         }
//     }
// }