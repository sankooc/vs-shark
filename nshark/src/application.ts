import { PacketElement, IPPacket, PVisitor, Protocol, Packet, FolderField, PosReader, PacketField } from './common';
import { Uint8ArrayReader } from './io';
import { UDP } from './transportLayer';
import { ARP_HARDWARE_TYPE_MAP, DHCP_TYPE_MAP, DHCP_OPTION_TYPE_MAP, DNS_TYPE_MAP, DNS_CLASS_MAP } from './constant';



export class StringRef {
    tmp?: string;
    constructor(private content: Uint8Array, private str: string, private ref: number) { }
    toString(): string {
        if (this.tmp) {
            return this.tmp
        }
        if (!this.ref) {
            return this.str;
        }
        const inx = this.ref & 0x3fff;
        const _reader = new Uint8ArrayReader(this.content);
        _reader.cursor = inx;
        let [pre, ref] = _reader.readCompressStringWithRef();
        if (this.str) {
            pre = this.str + '.' + pre;
        }

        if (ref) {
            this.tmp = new StringRef(this.content, pre, ref).toString();
        } else {
            this.tmp = pre;
        }
        return this.tmp;
    }
}
export class ResourceRecord extends Packet {
    owner?: StringRef;
    type?: number;
    clz?: number;
    ttl?: number;
    len?: number;
    rdata?: Uint8Array;
    extra?: RR;

    summary(): string {
        return `${this.owner.toString()}: type: ${this.getType()}, class ${this.getClass()}`;
    }
    public getClass(): string {
        return DNS_CLASS_MAP[this.clz];
    }
    public getType(): string {
        return DNS_TYPE_MAP[this.type];
    }
    createSummary(field: string): string {
        switch (field) {
            case 'owner':
                return `Name: ${this.owner.toString()}`;
            case 'type':
                return `Type: ${this.getType()} (${this.type}) `;
            case 'clz':
                return `Class: ${this.getClass()} (${this.clz})`;
            case 'ttl':
                return `Time to Live: ${this.ttl}`;
            case 'len':
                return `Data Length: ${this.len}`;
        }
        if (this.extra) {
            return this.extra.createSummary(field);
        }
        return null;
    }
}
export class RR {
    createSummary(field: string): string {
        return null;
    }
}

export class RR_A extends RR {
    ip: string;
    createSummary(field: string): string {
        if(field === 'a_ip'){
            return this.ip;
        }
        return null;
    }
}
export class RR_CNAME extends RR {
    host: StringRef;
    createSummary(field: string): string {
        if(field === 'host') return `CNAME: ${this.host.toString()}`;
        return null;
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
    createSummary(field: string): string {
        switch (field) {
            case 'primary':
                return `Primary name Server : ${this.primary.toString()}`;
            case 'ram':
                return `Responsible authority's mailbox: ${this.ram.toString()}`;
            case 'serialNumber':
                return `Serial Number: ${this.serialNumber}`;
            case 'refleshInterval':
                return `Refresh Interval: ${this.refleshInterval}`;
            case 'retryInterval':
                return `Retry Interval: ${this.retryInterval}`;
            case 'expireLimit':
                return `Expire limit: ${this.expireLimit}`;
            case 'minTTL':
                return `Minimum TTL: ${this.minTTL}`;
        }
        return null;
    }
}
export class RR_PRT extends RR {
    domain: StringRef;
    createSummary(field: string): string {
        if(field === 'domain'){
            return this.domain.toString(); 
        }
        return null;
    }
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

export class QueryPanel {
    summary(): string {
        return 'Queries';
    };
}

export class Query extends Packet {
    name?: string;
    type?: number;
    cls?: number;
    constructor(reader: Uint8ArrayReader) {
        super(reader);
    }
    public getClass(): string {
        return DNS_CLASS_MAP[this.cls];
    }
    public getType(): string {
        return DNS_TYPE_MAP[this.type];
    }
    createSummary(field: string): string {
        switch (field) {
            case 'name':
                return `Name: ${this.name}`;
            case 'type':
                return `Type: ${this.getType()} (${this.type})`;
            case 'cls':
                return `Class: ${this.getClass()} (0x${this.cls.toString(16).padStart(4, '0')})`
        }
        return '';
    }
    summary(): string {
        return `${this.name}: type ${this.getType()}, class ${this.getClass()}`;
    }
}
export class Answer extends Packet {
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
    type: string;
    flag: number;
    question: number;
    answer: number;
    authority: number;
    addtional: number;
    queries: Query[] = [];
    answers: ResourceRecord[] = [];
    authorities: ResourceRecord[] = [];
    addtionals: ResourceRecord[] = [];
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
    summary(): string {
        return 'Domain Name System';
    }
    createSummary(field: string): string {
        switch (field) {
            case 'transactionId':
                return `Transaction Id: ${this.transactionId}`;
            case 'question':
                return `Questions: ${this.question}`;
            case 'answer':
                return `Answer RRs: ${this.answer}`;
            case 'authority':
                return `Authority RRs: ${this.authority}`;
            case 'addtional':
                return `Addtional RRs: ${this.addtional}`;
        }
        return null;
    }
}

export class DHCPOptionPanel extends Packet {
    summary(): string {
        return `Options (${this.getSize()} bytes)`;
    }

}

export class DHCPOption extends Packet {
    len!: number;
    content!: Uint8Array;
    readonly type: number;
    constructor(type: number, reader: Uint8ArrayReader) {
        super(reader);
        this.type = type;
    }
    public isEnd(): boolean {
        return this.type === 255;
    }
    public getType(): string {
        return DHCP_OPTION_TYPE_MAP[this.type];
    }
    summary(): string {
        return `Option: (${this.type}) ${this.getType()}`;
    }
    getSize(): number {
        return this.len;
    }
}
export class DHCP extends IPPacket {
    op: number;
    type: string;
    htype: number;
    hlen: number;
    hops: number;
    sec: number;
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
    public getHardwareType(): string {
        return ARP_HARDWARE_TYPE_MAP[this.htype];
    }
    summary(): string {
        return `Dynamic Host Configuration Protocol (${this.type})`
    }
    createSummary(field: string): string {
        switch (field) {
            case 'op':
                return `Message type: ${this.type} (${this.op})`;
            case 'htype':
                return `Hardware type: ${this.getHardwareType()}`;
            case 'hlen':
                return `Hardware Address Len: ${this.hlen}`;
            case 'hops':
                return `Hops: ${this.hops}`;
            case 'transactionId':
                return `Transaction Id: ${this.transactionId}`;
            case 'sec':
                return `Second elapsed: ${this.sec}`;
            case 'clientAddress':
                return `Client Address: ${this.clientAddress}`;
            case 'yourAddress':
                return `Youre Address: ${this.clientAddress}`;
            case 'nextServerAddress':
                return `Next Server Address: ${this.clientAddress}`;
            case 'relayAddress':
                return `Relay Address: ${this.clientAddress}`;
            case 'macAddress':
                return `Client Mac Address: ${this.clientAddress}`;
            case 'magicCookie':
                return 'Magic Cookie: DHCP';
        }
        return null;
    }
}

export class NBNSVisitor implements PVisitor {
    createPacket(ele: PacketElement): [NBNS, Uint8ArrayReader] {
        const parent = ele.getPacket();
        const { reader } = parent;
        return [new NBNS(parent, reader, Protocol.NBNS), reader];
    }
    isAnswer(ele: PacketElement): boolean {
        const udp: UDP = ele.getPacket().getProtocol(Protocol.UDP) as UDP;
        return udp.targetPort === 137;
    }
    readQuery(reader: Uint8ArrayReader): Query {
        // const domain = reader.readNBNSQuery();
        // const type = reader.read16(false);
        // const cls = reader.read16(false);
        // const q = new Query(domain, type, cls);
        // return q;
        return null;
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
        const udp: UDP = ele.getPacket().getProtocol(Protocol.UDP) as UDP;
        return udp.targetPort === 53;
    }
    readQuery(reader: PosReader): Query {
        const q = new Query(reader.getReader());
        q.name = q.read('name', _reader => _reader.readDNSQuery());
        q.type = q.read16('type', false);
        q.cls = q.read16('cls', false);
        q._end();
        return q;
    }

    readResourceRecord(reader: PosReader, content: Uint8Array): ResourceRecord {
        const ans = new ResourceRecord(reader.getReader());
        ans.owner = new StringRef(content, '', ans.read16('owner', false));
        ans.type = ans.read16('type', false);
        ans.clz = ans.read16('clz', false);
        ans.ttl = ans.read32('ttl', false);
        ans.len = ans.read16('len', false);
        ans.rdata = ans.slice('rdata', ans.len);
        ans._end()
        ans.reader = new Uint8ArrayReader(ans.rdata);

        switch (ans.type) {
            case 1: {
                const r = new RR_A();
                r.ip = ans.readIp('a_ip').getAddress();
                ans.extra = r;
                break;
            }
            case 5: {
                const r = new RR_CNAME();
                const [str, ref] = ans.read('host', reader => reader.readCompressStringWithRef());
                r.host = new StringRef(content, str, ref);
                ans.extra = r;
                break;
            }
            case 6: {
                const r = new RR_SOA();
                let [str, ref] = ans.read('primary', reader => reader.readCompressStringWithRef());
                r.primary = new StringRef(content, str, ref);
                [str, ref] = ans.read('ram', reader => reader.readCompressStringWithRef());
                r.ram = new StringRef(content, str, ref);
                r.serialNumber = ans.read32('serialNumber', false);
                r.refleshInterval = ans.read32('refleshInterval', false);
                r.retryInterval = ans.read32('retryInterval', false);
                r.expireLimit = ans.read32('expireLimit', false);
                r.minTTL = ans.read32('minTTL', false);
                ans.extra = r;
                break;
            }
            case 12: {
                const r = new RR_PRT();
                const [str, ref] = ans.read('domain', reader => reader.readCompressStringWithRef());
                r.domain = new StringRef(content, str, ref);
                ans.extra = r;
                break;
            }
        }
        // return new RR(ans);
        return ans;
    }

    visit(ele: PacketElement): IPPacket {
        //https://www.rfc-editor.org/rfc/rfc1035.html
        //https://www.rfc-editor.org/rfc/rfc1034
        const [data, reader] = this.createPacket(ele);
        const _content = reader.extra2();
        data.transactionId = data.readHex('transactionId', 2, '');
        const flag = data.read16('flag', false);
        data.question = data.read16('question', false);
        data.answer = data.read16('answer', false);
        data.authority = data.read16('authority', false);
        data.addtional = data.read16('addtional', false);
        data._isResponse = (flag >> 15) > 0;
        const opcode = (flag >> 11) & 0xf
        data.flag = flag;

        if (data.question > 0) {
            const qPanel = new FolderField(reader, "Queries");
            data.fields.push(qPanel);
            for (let i = 0; i < data.question; i += 1) {
                const _q = this.readQuery(qPanel);
                qPanel.fields.push(_q);
                data.queries.push(_q);
            }
            qPanel._end();
        }
        if (data.answer > 0) {
            const qPanel = new FolderField(reader, "Answers");
            data.fields.push(qPanel);

            for (let i = 0; i < data.answer; i += 1) {
                const q = this.readResourceRecord(qPanel, _content);
                qPanel.fields.push(q);
                data.answers.push(q);
            }
            qPanel._end();
        }
        
        if (data.authority > 0) {
            const qPanel = new FolderField(reader, "Authoritative nameservers");
            data.fields.push(qPanel);
            for (let i = 0; i < data.authority; i += 1) {
                const q = this.readResourceRecord(qPanel, _content);
                qPanel.fields.push(q);
                data.authorities.push(q);
            }
            qPanel._end();
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
        data.htype = data.read8('htype');
        data.hlen = data.read8('hlen');
        data.hops = data.read8('hops');
        data.transactionId = data.read32('transactionId');
        data.sec = data.read16('sec', false);
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
        const oPanel = new DHCPOptionPanel(reader);
        data.fields.push(oPanel);
        while (true) {
            const opt = new DHCPOption(reader.read8(), oPanel.getReader());
            data.options.push(opt);
            oPanel.fields.push(opt);
            if (opt.isEnd()) {
                opt._end();
                break;
            }
            opt.len = reader.read8();
            opt.content = reader.slice(opt.len);
            if (opt.type === 53) {
                const t = opt.content[0];
                data.type = DHCP_TYPE_MAP[t];
            }
            opt._end();
            if (!reader.eof()) {
                break;
            }
        }
        oPanel._end();
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
            const [line, pk] = data.readEnter('head');
            pk.render = (f: string) => line;
            [ext, path, version] = line.split(' ');
        } else {
            const [line, pk] = data.readEnter('head');
            pk.render = (f: string) => line;
            [version, code, status] = line.split(' ');
        }
        let flag = true;
        const m: Set<string> = new Set()
        do {
            const [line, pk] = data.readEnter('headers');
            pk.render = (f: string) => line;
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