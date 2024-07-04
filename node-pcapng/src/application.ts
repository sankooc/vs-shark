import { PacketElement, IPPacket, PVisitor, Protocol } from './common';
import { Uint8ArrayReader } from './io';
import { UDP } from './transportLayer';
import { DHCP_TYPE_MAP, DHCP_OPTION_TYPE_MAP, DNS_TYPE_MAP,DNS_CLASS_MAP } from './constant';



export class StringRef {
    constructor(private content: Uint8Array, private str: string, private ref: number){}
    toString(): string{
        if(!this.ref){
            return this.str;
        }
        const inx = this.ref & 0x3fff;
        const _reader = new Uint8ArrayReader(this.content);
        _reader.cursor = inx;
        let [pre, ref] =  _reader.readCompressStringWithRef();
        if(this.str){
            pre = this.str + '.' + pre;
        }

        if(ref) {
            return new StringRef(this.content, pre, ref).toString();
        }
        return pre;
    }
}
export class ResourceRecord {
    onwer: StringRef;
    // onwer_str: string;
    type: number;
    clz: number;
    ttl: number;
    len: number;
    rdata: Uint8Array;
}
export class RR {
    constructor(public record: ResourceRecord){}
    public getClass(): string {
        return DNS_CLASS_MAP[this.record.clz];
    }
    public getType(): string {
        return DNS_TYPE_MAP[this.record.type];
    }
    convert(record: ResourceRecord) {
    };
    summary(): string{
        return `${this.record.onwer.toString()}: type: ${this.getType()}, class ${this.getClass()}`;
    }
}

export class RR_A extends RR {
    ip: string;
    summary(): string{
        return `${super.summary()}, addr ${this.ip}`;
    }
}
export class RR_CNAME extends RR {
    host: StringRef;
    summary(): string{
        return `${super.summary()}, addr ${this.host.toString()}`;
    }
}
export class RR_SOA extends RR {
    primary: StringRef;
    ram: StringRef;
    serialNumber: number;
    refleshInterval: number;
    retryInterval: number;
    expireLimit: number;
    minTTL: number;
}
export class RR_PRT extends RR {
    domain: StringRef;
}

/**
 *  NB         0x0020   NetBIOS general Name Service Resource Record
    NBSTAT     0x0021   NetBIOS NODE STATUS Resource Record (See NODE STATUS REQUEST)
 */
export class NBNS extends IPPacket {
    transactionId: string;
    flag: number;
    question: number;
    answer: number;
    authority: number;
    addtional: number;
    queries: Query[] = [];
    answers: Answer[] = [];
    authorities: Authority[] = [];
    addtionals: Addtional[] = [];
    isAnswer: boolean;
    isResponse(): boolean {
        return this.isAnswer;
    }
    getQhost(): string {
        if (this.queries?.length) {
            return this.queries.map((qr: Query) => { return qr.name }).join(',')
        }
        return '';
    }
    toString(): string {
        if (this.isAnswer) {
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
export class Query{
    constructor(public name: string, public type: number, public cls: number) {}
    public getClass(): string {
        return DNS_CLASS_MAP[this.cls];
    }
    public getType(): string {
        return DNS_TYPE_MAP[this.type];
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
export class DNS extends IPPacket {
    transactionId: string;
    flag: number;
    question: number;
    answer: number;
    authority: number;
    addtional: number;
    queries: Query[] = [];
    answers: RR[] = [];
    authorities: RR[] = [];
    addtionals: RR[] = [];
    _isResponse!: boolean;
    isResponse(): boolean {
        return this._isResponse;
    }
    getQhost(): string {
        if (this.queries?.length) {
            return this.queries.map((qr: Query) => { return qr.name }).join(',')
        }
        return '';
    }
    toString(): string {
        if (this.isResponse()) {
            return `Standard query response ${this.transactionId}`;
        }
        return `Standard query ${this.transactionId} ${this.getQhost()}`;
    }
    info(): string {
        return this.toString();
    }
    type: string;
    summary(): string {
        return 'Domain Name System';
    }
}
export class DHCPOption {
    len!: number;
    content!: Uint8Array;
    constructor(public readonly type: number) { }
    public isEnd(): boolean {
        return this.type === 255;
    }
    public summary(): string {
        return `Option: (${this.type})`;
    }
    public getType(): string {
        return DHCP_OPTION_TYPE_MAP[this.type];
    }
}
export class DHCP extends IPPacket {
    op: number;
    type: string;
    transactionId: number;
    clientAddress: string;
    yourAddress: string;
    nextServerAddress: string;
    relayAddress: string;
    macAddress: string;
    options: DHCPOption[] = [];
    toString(): string {
        return `DHCP ${this.type} -  transaction id: ${this.transactionId}`;
    }
}

export class NBNSVisitor implements PVisitor {
    createPacket(ele: PacketElement): [NBNS, Uint8ArrayReader] {
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

export class DNSVisitor implements PVisitor {

    createPacket(ele: PacketElement): [DNS, Uint8ArrayReader] {
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
        return new Query(domain, type, cls);
    }

    readResourceRecord(reader: Uint8ArrayReader, content: Uint8Array): RR {
        const ans = new ResourceRecord();
        ans.onwer = new StringRef(content, '', reader.read16(false));
        ans.type = reader.read16(false);
        ans.clz = reader.read16(false);
        ans.ttl = reader.read32(false);
        ans.len = reader.read16(false);
        ans.rdata = reader.slice(ans.len);
        switch(ans.type){
            case 1: {
                const r = new RR_A(ans);
                r.ip = new Uint8ArrayReader(ans.rdata).readIp();
                return r;
            }
            case 5: {
                const r = new RR_CNAME(ans);
                const [str, ref] = new Uint8ArrayReader(ans.rdata).readCompressStringWithRef();
                r.host = new StringRef(content, str, ref);
                return r;
            }
            case 6: {
                const r = new RR_SOA(ans);
                const _reader = new Uint8ArrayReader(ans.rdata);
                let [str, ref] = _reader.readCompressStringWithRef();
                r.primary = new StringRef(content, str, ref);
                [str, ref] = _reader.readCompressStringWithRef();
                r.ram = new StringRef(content, str, ref);
                r.serialNumber = _reader.read32(false);
                r.refleshInterval = _reader.read32(false);
                r.retryInterval = _reader.read32(false);
                r.expireLimit = _reader.read32(false);
                r.minTTL = _reader.read32(false);
                return r;
            }
            case 12: {
                const r = new RR_PRT(ans);
                const [str, ref] =  new Uint8ArrayReader(ans.rdata).readCompressStringWithRef();
                r.domain = new StringRef(content, str, ref);
                console.log(r.domain.toString());
                return r;
            }
        }
        return new RR(ans);
    }
    
    visit(ele: PacketElement): IPPacket {
        //https://www.rfc-editor.org/rfc/rfc1035.html
        //https://www.rfc-editor.org/rfc/rfc1034
        const [data, reader] = this.createPacket(ele);
        const _content = reader.extra2();
        const transactionId = data.readHex('transactionId', 2, '')
        const flag = data.read16('flag', false);
        const question = data.read16('question', false);
        const answer = data.read16('answer',false);
        const authority = data.read16('authority', false);
        const addtional = data.read16('addtional', false);
        data._isResponse = (flag >> 15) > 0;
        const opcode = (flag >> 11) & 0xf
        data.transactionId = transactionId;
        data.flag = flag;
        data.question = question;
        data.answer = answer;
        data.authority = authority;
        data.addtional = addtional;
        // data.isAnswer = this.isAnswer(ele);
        for (let i = 0; i < data.question; i += 1) {
            data.queries.push(this.readQuery(reader));
        }
        for (let i = 0; i < data.answer; i += 1) {
            data.answers.push(this.readResourceRecord(reader,_content));
        }
        for (let i = 0; i < data.authority; i += 1) {
            data.authorities.push(this.readResourceRecord(reader,_content));
            // data.authorities.push(this.readAuthority(reader));
        }
        // for (let i = 0; i < data.addtional; i += 1) {
        //     data.addtionals.push(this.readResourceRecord(reader,_content));
        // }
        ele.getContext().resolveDNS(data);
        return data;
    }
}

export class DHCPVisitor implements PVisitor {
    // https://en.wikipedia.org/wiki/Dynamic_Host_Configuration_Protocol
    visit(ele: PacketElement): IPPacket {
        const parent = ele.getPacket();
        const { reader } = parent;
        const data = new DHCP(parent, reader, Protocol.DHCP);

        data.op = data.read8('op');
        const htype = data.read8('htype');
        const hlen = data.read8('hlen');
        const hops = data.read8('hops');
        data.transactionId = data.read32('transactionId');
        const sec = data.read16('sec', false);
        const flag = data.read16('flag', false);
        // reader.read32Hex()
        data.clientAddress = data.read32Hex('clientAddress');
        data.yourAddress = data.read32Hex('yourAddress');
        data.nextServerAddress = data.read32Hex('nextServerAddress');
        data.relayAddress = data.read32Hex('relayAddress');
        data.macAddress = data.readHex('macAddress', 6, ':');
        reader.skip(10)//padding
        reader.skip(64)//sname
        reader.skip(128)//file
        const magicCookie = data.read32('magicCookie');
        while (true) {
            const opt = new DHCPOption(reader.read8());
            data.options.push(opt);
            if (opt.isEnd()) {
                break;
            }
            opt.len = reader.read8();
            opt.content = reader.slice(opt.len);
            if (opt.type === 53) {
                const t = opt.content[0];
                data.type = DHCP_TYPE_MAP[t];
            }
            if (!reader.eof()) {
                break;
            }
        }
        // TODO RESOLVE
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