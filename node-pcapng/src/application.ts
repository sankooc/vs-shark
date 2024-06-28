import { PacketElement, IPPacket, PVisitor, Protocol } from './common';
import { Uint8ArrayReader } from './io';
import { UDP } from './transportLayer';


/**
 *  NB         0x0020   NetBIOS general Name Service Resource Record
   NBSTAT     0x0021   NetBIOS NODE STATUS Resource Record (See NODE
                       STATUS REQUEST)
 */
export class NBNS extends IPPacket {
    transactionId: string;
    flag: number;
    question: number;
    answer: number;
    authority: number;
    addtional: number;
    payload: Uint8Array;
    queries: Query[] = [];
    answers: Answer[] = [];
    authorities: Authority[] = [];
    addtionals: Addtional[] = [];
    isAnswer: boolean;
    getQhost(): string {
        if(this.queries?.length){
            return this.queries.map((qr: Query) => { return qr.name }).join(',')
        }
        return '';
    }
    toString(): string {
        if(this.isAnswer){
            return `Standard query response ${this.transactionId}`;
        }
        return `Standard query ${this.transactionId} ${this.getQhost()}`;
    }
    info(): string {
        return this.toString();
    }
    summary(): string {
        return 'NetBIOS Name Service';
    }
}

class NSFact {
    protocol!: Protocol;
}
export class Query extends NSFact {
    name: string;
    type: number;
    cls: number; //0x0001 Internet class
    constructor(name: string, type: number, cls: number) {
        super();
        this.name = name;
        this.type = type;
        this.cls = cls;
    }
}
export class Answer extends NSFact {
    name: number;
    type: number;
    cls: number;
    ttl: number;
    len: number;
    host: string;
}

export class Authority extends NSFact {

}
export class Addtional extends NSFact {
    name: number;
    type: number;
    cls: number;
    ttl: number;

}
export class DNS extends NBNS {
    summary(): string {
        return 'Domain Name System';
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


export class NBNSVisitor implements PVisitor {
    createPacket(ele: PacketElement): [NBNS,Uint8ArrayReader] {
        const parent = ele.getPacket();
        const { reader } = parent;
        return [new NBNS(parent, reader, Protocol.NBNS), reader];
    }
    isAnswer(ele: PacketElement): boolean {
        const udp: UDP = ele.getPacket().getProtocal(Protocol.UDP) as UDP;
        return udp.targetPort === 137;
    }
    readQuery(reader: Uint8ArrayReader): Query {
        const domain = reader.readNBNSQuery();
        const type = reader.read16(false);
        const cls = reader.read16(false);
        const q = new Query(domain, type, cls);
        q.protocol = Protocol.NBNS;
        return q;
    }
    readAnswer(reader: Uint8ArrayReader): Answer {
        //TODO
        return null;
    }
    readAuthority(reader: Uint8ArrayReader): Authority {
        // TODO
        return null;
    }
    readAddtional(reader: Uint8ArrayReader): Addtional {
        //TODO 
        return null;
    }
    visit(ele: PacketElement): IPPacket {
        //https://datatracker.ietf.org/doc/html/rfc1001
        //https://datatracker.ietf.org/doc/html/rfc1002
        //https://blog.csdn.net/CodingMen/article/details/105056639
        
        const [data, reader] = this.createPacket(ele);
        const transactionId = reader.readHex(2)
        const flag = reader.read16(false);
        const question = reader.read16(false);
        const answer = reader.read16(false);
        const authority = reader.read16(false);
        const addtional = reader.read16(false);
        
        data.transactionId = transactionId;
        data.flag = flag;
        data.question = question;
        data.answer = answer;
        data.authority = authority;
        data.addtional = addtional;
        data.isAnswer = this.isAnswer(ele);
        for (let i = 0; i < data.question; i += 1) {
            data.queries.push(this.readQuery(reader));
        }
        for (let i = 0; i < data.answer; i += 1) {
            data.answers.push(this.readAnswer(reader))
        }
        for (let i = 0; i < data.authority; i += 1) {
            data.authorities.push(this.readAuthority(reader));
        }
        for (let i = 0; i < data.addtional; i += 1) {
            data.addtionals.push(this.readAddtional(reader))
        }
        return data;
    }
}

export class DNSVisitor extends NBNSVisitor {

    createPacket(ele: PacketElement): [NBNS,Uint8ArrayReader] {
        const parent = ele.getPacket();
        const { reader } = parent;
        return [new DNS(parent, reader, Protocol.DNS), reader];
    }
    isAnswer(ele: PacketElement): boolean {
        const udp: UDP = ele.getPacket().getProtocal(Protocol.UDP) as UDP;
        return udp.targetPort === 53;
    }
    readQuery(reader: Uint8ArrayReader): Query {
        const domain = reader.readDNSQuery();
        const type = reader.read16(false);
        const cls = reader.read16(false);
        const q = new Query(domain, type, cls);
        q.protocol = Protocol.DNS;
        return q;
    }

    readAnswer(reader): Answer {
        const ans = new Answer();
        ans.name = reader.read16(false);
        ans.type = reader.read16(false);
        ans.cls = reader.read16(false);
        ans.ttl = reader.read32(false);
        ans.len = reader.read16(false);
        ans.protocol = Protocol.DNS;
        if (ans.type === 5) {
            const [domain, id] = reader.readDNSAnswer(ans.len);
            ans.host = domain;
        } else {
            ans.host = reader.readIp();
        }
        return ans;
    }
}

export class DHCPVisitor implements PVisitor {
    // https://en.wikipedia.org/wiki/Dynamic_Host_Configuration_Protocol
    visit(ele: PacketElement): IPPacket {
        const parent = ele.getPacket();
        const { reader } = parent;
        const data = new DHCP(parent, reader, Protocol.DHCP);

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
    getHeader(key: string): string {
        for (const v of this.headers) {
            const [k, val] = v.split(': ');
            if (k.toUpperCase() == key.toUpperCase()) {
                return val;
            }
        }
        return null;
    }
}


export class HTTPVisitor implements PVisitor {
    visit(ele: PacketElement): IPPacket {
        const parent = ele.getPacket();
        const { reader } = parent;
        const data = new HttpPT(parent, reader, Protocol.HTTP);

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
        return data;
    }

}