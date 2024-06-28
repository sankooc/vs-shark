import { AbstractReaderCreator, Uint8ArrayReader } from './io';
import { ARP } from './networkLayer';
import { TCP } from './transportLayer';
import { DNS, NBNS } from './application';
import { DataPacket } from './dataLinkLayer';
export enum Protocol {
    ETHER,
    MAC,
    IPV4,
    IPV6,
    ARP,
    TCP,
    UDP,
    ICMP,
    IGMP,
    DNS,
    NBNS,
    DHCP,
    TLS,
    SSL,
    HTTP,
    HTTPS,
    WEBSOCKET,
}
export enum FileType {
    PCAP,
    PCAPNG,
}



export class PacketField {
    constructor(public name: string, public start: number, public size: number) { }
}
export class Packet {
    readonly reader: Uint8ArrayReader;
    readonly start: number;
    end: number = 0;
    constructor(reader: Uint8ArrayReader) {
        this.reader = reader;
        this.start = reader.cursor;
    }
    getPacketData(): Uint8Array {
        return this.reader.arr;
    }
    getPacketSize(): number {
        return this.reader.arr.length;
    }
    getProtocolSize(): number {
        return this.reader.arr.length - this.start;
    }
    getPayLoad(): Uint8Array {
        return this.reader.extra2();
    }
    getPayloadSize(): number {
        return this.reader.left();
    }
    _end(): void {
        this.end = this.reader.cursor;
    }
    toString(): string {
        return 'packet';
    }
}

export class IPPacket extends Packet implements PacketElement {
    index: number;
    protocol: Protocol;
    parent!: IPPacket;
    fields: PacketField[] = [];
    constructor(parent: IPPacket, reader: Uint8ArrayReader, protocol: Protocol) {
        super(reader);
        this.protocol = protocol;
        this.parent = parent;
        if (this.parent) {
            this.parent._end();
        }
    }
    setIndex(inx: number): void {
        if (this.parent) {
            this.parent.setIndex(inx);
            return;
        }
        this.index = inx;
    }
    getIndex(): number {
        if (this.parent) {
            return this.parent.getIndex();
        }
        return this.index;
    }
    getProtocal(name: Protocol): IPPacket {
        if (this.protocol === name) {
            return this;
        }
        if (this.parent) {
            return this.parent.getProtocal(name);
        }
        return null;
    }

    readHex(name: string, len: number, flag: string): string {
        this.fields.push(new PacketField(name, this.reader.cursor, len))
        return this.reader.readHex(len, flag);
    };
    getContext(): Context {
        const ep = (this.getProtocal(Protocol.ETHER) as EtherPacket);
        if (ep) {
            return ep.context;
        }
        return null;
    }
    getPacket(): IPPacket {
        return this;
    }
    accept(visitor: PVisitor): IPPacket {
        if (!visitor) {
            return this;
        }
        return visitor.visit(this);
    }
}

export class EtherPacket extends IPPacket {
    interface: number;
    ts: number;
    nano: number;
    captured: number;
    origin: number;
    context: Context;
    constructor(reader: Uint8ArrayReader, context: Context, index: number) {
        super(null, reader, Protocol.ETHER);
        this.context = context;
        this.index = index;
    }
}

export interface PElement {
    accept(visitor: Visitor): IPPacket;
}
export class TCPStack {
    ip: string;
    port: number;
    start: number;
    sequence: number = 0;
    next: number = 0;
    ack: number = 0;
    finished: boolean = false;
    constructor(ip: string, port: number) {
        this.ip = ip;
        this.port = port;
    }
    checkDump(sequence: number, next: number) {
        return this.sequence == sequence && this.next == next;
    }
}

export class TCPConnect {
    ep1: TCPStack;
    ep2: TCPStack;
    total: number = 0;
    tcpSize: number = 0;
    tcpUse: number = 0;
    count: number = 0;
    countUse: number = 0;
    from: string;
    status: number;
    constructor(ip1: string, port1: number, ip2: string, port2: number) {
        this.ep1 = new TCPStack(ip1, port1);
        this.ep2 = new TCPStack(ip2, port2);
    }
    getConnectName(): string {
        return `${this.ep1.ip}:${this.ep1.port}-${this.ep2.ip}:${this.ep2.port}`;
    }
    getStack(arch: boolean): TCPStack {
        if (arch) return this.ep1;
        return this.ep2;
    }
    finish() { }
}

export class Resolver {
    tcpConnections: TCPConnect[] = [];
    tcpCache: Map<string, TCPConnect> = new Map();
    arpMap: Map<string, Set<string>> = new Map();
    flush(key: string): void {
        if (!key) {
            this.tcpCache.forEach((value) => {
                this.tcpConnections.push(value);
            })
            this.tcpCache.clear();
            return;
        }
        const connect = this.tcpCache.get(key);
        this.tcpCache.set(key, null);
        if (connect) {
            this.tcpConnections.push(connect)
        }
    }
}

// export class BasicElement implements PElement {
//     name: string;
//     readerCreator: AbstractReaderCreator;
//     len: number;
//     content: Uint8Array;
//     packet!: IPPacket;
//     context!: Context;
//     constructor(name: string, readerCreator: AbstractReaderCreator, len: number, content: Uint8Array, ctx: Context) {
//         this.name = name;
//         this.readerCreator = readerCreator;
//         this.len = len;
//         this.content = content;
//         this.context = ctx;
//     }
//     accept(visitor: PVisitor): IPPacket {
//         return visitor.visit(this)
//     }
//     log(...arg) {
//         if (process.env.NODE_ENV === "DEBUG") {
//             console.log(this.name, ...arg)
//         }
//     }
//     info(...arg) {
//         console.log(this.name, ...arg)
//     }
// }
export interface PacketElement extends PElement {
    getContext(): Context;
    getPacket(): IPPacket;
    accept(visitor: PVisitor): IPPacket;
}

export interface Visitor {
    visit(ele: PElement): Packet;
}
export interface PVisitor extends Visitor {
    visit(ele: PacketElement): IPPacket;
}
export abstract class AbstractVisitor implements Visitor {
    abstract visit(ele: PElement): IPPacket;
}


export class FileInfo {
    type: FileType;
    hardware!: string;
    os!: string;
    client!: string;
    majorVersion!: number;
    minorVersion!: number;
    linkType!: number;
    interfaceName!: string;
    interfaceDesc!: string;
}

export class CNode {
    ip: string;
    mac: string;
    constructor(ip: string, mac: string) {
        this.ip = ip;
        this.mac = mac;
    }
}
export class ARPReply {
    host: CNode;
    clients: CNode[] = [];
    constructor(host: CNode) {
        this.host = host;
    }
}
export interface Context {
    getFrames(): IPPacket[];
    getFrame(inx: number): IPPacket;//1 based
    getCurrentIndex(): number;
    getFileInfo(): FileInfo;
    resolve(p: ARP): void;
    getARPReplies(): ARPReply[];
    resolveTCP(p: TCP): void;
    getTCPConnections(): TCPConnect[];
    resolveDNS(p: DNS | NBNS): void;
    createEtherPacket(reader: Uint8ArrayReader): EtherPacket;
}

export abstract class AbstractRootVisitor implements Visitor, Context {
    resolver: Resolver = new Resolver();
    readonly packets: IPPacket[] = []
    index: number = 0;
    getFrames(): IPPacket[] {
        return this.packets;
    }
    getFrame(inx: number): IPPacket {
        return this.packets[inx - 1];
    }
    getCurrentIndex(): number {
        return this.index;
    }
    abstract getFileInfo(): FileInfo;
    protected getNextIndex(): number {
        this.index += 1;
        return this.index;
    }

    createEtherPacket(reader: Uint8ArrayReader): EtherPacket {
        return new EtherPacket(reader, this, this.getNextIndex());
    }
    protected addPacket(packet: IPPacket): void {
        this.packets.push(packet);
    };
    public getContext(): Context {
        return this;
    }
    resolveDNS(p: DNS | NBNS): void {

    }
    resolve(p: ARP): void {
        const { oper } = p;
        if (oper === 2) {
            const sourceKey = `${p.senderMac}@${p.senderIp}`;
            let list = this.resolver.arpMap.get(sourceKey);
            if (!list) {
                list = new Set();
                this.resolver.arpMap.set(sourceKey, list);
            }
            list.add(`${p.targetMac}@${p.targetIp}`);
        }
    }
    resolveTCP(p: TCP): void {
        const resolver = this.resolver;
        const payloadSize = p.getPayloadSize()
        const noContent = p.ack && !p.psh && payloadSize < 9;
        const [arch, ip1, port1, ip2, port2] = p.mess();
        const key = `${ip1}${port1}-${ip2}${port2}`;
        let connect = resolver.tcpCache.get(key);
        if (!connect) {
            if (noContent) return;
            connect = new TCPConnect(ip1, port1, ip2, port2);
            resolver.tcpCache.set(key, connect);
        }
        const sequence = p.sequence;
        const nextSequence = p.sequence + payloadSize
        const stack = connect.getStack(arch);
        const dump = stack.checkDump(sequence, nextSequence);
        p.isDump = dump;
        connect.count += 1;
        connect.total += p.getPacketSize();
        connect.tcpSize += payloadSize;
        if (dump) {
            return;
        }
        connect.tcpUse += payloadSize;
        connect.countUse += 1;
        stack.sequence = sequence;
        stack.next = nextSequence;
        const stackRec = connect.getStack(!arch);
        stackRec.ack = p.acknowledge;
        // if (p.ack) {

        // }
        // if (p.ack && !p.psh) {
        //     if (p.packet.length > 10) {
        //         const len = p.getProtocal(Protocol.ETHER).packet.length;
        //     }
        // }
        // if (p.psh) {
        // }
    }
    getTCPConnections(): TCPConnect[] {
        return this.resolver.tcpConnections;
    }
    getARPReplies(): ARPReply[] {
        const arp = this.resolver.arpMap;
        const hostnames = arp.keys();
        const rs: ARPReply[] = [];
        arp.forEach((values, hostname) => {
            const [mac, ip] = hostname.split('@');
            const reply = new ARPReply(new CNode(ip, mac));
            values.forEach((val) => {
                const [mac, ip] = val.split('@');
                reply.clients.push(new CNode(ip, mac));
            });
            rs.push(reply);
        });
        return rs;
    }

    getHTTPConnects(): void { }
    abstract _visit(ele: InputElement): void;
    visit(ele: InputElement): Packet {
        const { readerCreator, content } = ele;
        const start = Date.now();
        this._visit(ele);
        const per = Date.now() - start;
        this.resolver.flush(null);
        return null;
    }
}

export class Option {
    code: number;
    value: any;
    len: number;
    constructor(code: number, value: any, len: number) {
        this.code = code;
        this.value = value;
        this.len = len;
    }
}

export class InputElement implements PElement {
    accept(visitor: Visitor): IPPacket {
        return visitor.visit(this) as IPPacket;
    }
    constructor(public content: Uint8Array, public readerCreator: AbstractReaderCreator) { }
} 