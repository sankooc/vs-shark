import { AbstractVisitor, BasicElement, BasicEmptyVisitor, IPPacket, PVisitor, Protocol } from './common';
import { Uint8ArrayReader } from './io';

export class NBNS extends IPPacket {
    transactionId: string;
    flag: number;
    question: number;
    answer: number;
    authority: number;
    addtional: number;
}

export class DNS extends NBNS {
    domain: string;
    type: number;
}

export class DHCP extends IPPacket {
    transactionId: number;
    clientAddress: string;
    yourAddress: string;
    nextServerAddress: string;
    relayAddress: string;
    macAddress: string;
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
        return data;
    }
    // next(ele: BasicElement, reader: Uint8ArrayReader): void {
    //     const transactionId = reader.readHex(2)
    //     const flag = reader.read16(false);
    //     const question = reader.read16(false);
    //     const answer = reader.read16(false);
    //     const authority = reader.read16(false);
    //     const addtional = reader.read16(false);
    //     Object.assign(this, {transactionId, flag, question, answer, authority, addtional});

    // }
}

// class PayloadVisitor extends BasicVisitor {
//     items: [];
//     // constructor(sub: AbstractVisitor, name: string,sourcePort: number, destPort: number){
//     //     super(sub,name, sourcePort, destPort)
//     // }
//     visit(ele: BasicElement): void {
//         const { name, readerCreator, content } = ele;
//         const prefix = name + '/' + this.name;
//         const type = content[0];
//         let _type = '';
//         let version = content[1] +':' + content[2];
//         const len = content[3] * 16 * 16 + content[4];
//         switch(type){
//             case 20:
//                 _type ='ChangeCipherSpec';
//                 break
//             case 21:
//                 _type ='Aler';
//                 break;
//             case 22:
//                 _type ='Handshake';
//                 break;
//             case 23:
//                 _type ='Application';
//                 break;
//             case 24:
//                 _type ='Heartbeat';
//                 break;
//             default:
//                 return this.raw(ele);
//         }
//         switch(version){
//             case '3:1':
//                 version = 'tls1.0';
//                 break;
//             case '3:2':
//                 version = 'tls1.1';
//                 break;
//             case '3:3':
//                 version = 'tls1.2';
//                 break;
//             case '3:4':
//                 version = 'tls1.3';
//                 break;
//             case '3:0':
//                 version = 'ssl3.0';
//                 break;
//             default:
//                 return this.raw(ele);
//         }

//         if((content.length - 4) > len){
//             const reader = readerCreator.createReader(content, prefix, false);
//             const _type = reader.read8();
//             const _version = reader.read16(false);
//             const _len = reader.read16(false);
//             console.log('sub', _type, _version, _len);
//             return this.tls(ele)
//         } else {
//             console.log('-------------', content[0].toString(16), version)
//             console.log('-------------', content.length, len)
//             console.log('-------------', this.sourcePort, this.destPort)
//             readerCreator.createReader(content, prefix, true);
//             process.exit(0)
//         }
//         Object.assign(this, {type, _type, version, len});

//     }
//     tls(ele: BasicElement): void {
//         const { name, readerCreator, content } = ele;
//         const prefix = name + '/' + this.name;

//         const reader = readerCreator.createReader(content, prefix, true);
//     }
//     raw(ele: BasicElement): void {

//     }
//     next(ele: BasicElement, reader: Uint8ArrayReader): void {

//     }
// }

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