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
    constructor(name: string, type: number, cls: number) {
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
        for (let i = 0; i < data.question; i += 1) {
            const domain = reader.readDNSQuery();
            const type = reader.read16(false);
            const cls = reader.read16(false);
            data.queries.push(new Query(domain, type, cls));
        }
        for (let i = 0; i < data.answer; i += 1) {
            const ans = new Answer();
            ans.name = reader.read16(false);
            ans.type = reader.read16(false);
            ans.cls = reader.read16(false);
            ans.ttl = reader.read32(false);
            ans.len = reader.read16(false);
            if (ans.type === 5) {
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



export class HttpPT extends IPPacket {
    method!: string;
    path!: string;
    status!: string;
    code!: string;
    type!: string;
    version!: string;
    headers!: Set<string>;
    payload!: Uint8Array;
    toString(): string {
        if (this.type === 'request') {
            return `${this.method} ${this.path} ${this.version}`;
        }
        return `${this.version} ${this.code} ${this.status}`;
    }
    summary(): string {
        return `Hypertext Transfer Protocol (${this.type})`
    }
    getHeader(key: string): string{
        for(const v of this.headers){
            const [k , val] = v.split(': ');
            if(k.toUpperCase() == key.toUpperCase()){
                return val;
            }
        }
        return null;
    }
}


export class HTTPVisitor implements PVisitor {
    visit(ele: BasicElement): IPPacket {
        const { name, readerCreator, content } = ele;
        const prefix = name + '/http';
        const reader = readerCreator.createReader(content, prefix, false);

        const method = reader.readSpace(10);
        const type = method === 'HTTP/1.1' ? 'response' : 'request';
        let ext, path, version, status, code;
        if (type === 'request') {
            const line = reader.readEnter();
            [ext, path, version] = line.split(' ');
        } else {
            const line = reader.readEnter();
            [version, code, status] = line.split(' ');
        }
        let flag = true;
        const m: Set<string> = new Set()
        do {
            const line = reader.readEnter();
            m.add(line);
            if (!line) break;
        } while (flag);
        reader.skip(2);
        const payload = reader.extra();
        const data = new HttpPT(ele.packet, payload, Protocol.HTTP);
        data.type = type;
        data.version = version;
        if (type === 'request') {
            data.method = method;
            data.path = path;
        } else {
            data.code = code;
            data.status = status;
        }
        data.headers = m;
        data.payload = payload;
        // console.log('data', type);
        // console.log('data', type, data.version, data.toString());
        return data;
    }

}