import { AbstractReaderCreator, Uint8ArrayReader } from './io';
import { ARP } from './networkLayer';
import { TCP } from './transportLayer';
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
export class Packet {
    packet: Uint8Array;
    constructor(packet: Uint8Array) {
        this.packet = packet;
    }
    getPacket(): Uint8Array {
        return this.packet
    }
    toString(): string {
        return 'packet';
    }
}

export class IPPacket extends Packet {
    index: number;
    protocol: Protocol;
    parent: IPPacket;
    constructor(parent: IPPacket, packet: Uint8Array, protocol: Protocol) {
        super(packet);
        this.protocol = protocol;
        this.parent = parent;
    }
    createSubElement(name: string, parent: BasicElement): BasicElement {
        const ele = new BasicElement(name, parent.readerCreator, this.packet.length, this.packet, parent.context);
        ele.packet = this;
        return ele;
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
}

export class EtherPacket extends IPPacket {
    interface: number;
    ts: number;
    nano: number;
    captured: number;
    origin: number;
    constructor(packet: Uint8Array) {
        super(null, packet, Protocol.ETHER);
    }
}

export interface PElement {
    accept(visitor: PVisitor): IPPacket;
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

export class BasicElement implements PElement {
    name: string;
    readerCreator: AbstractReaderCreator;
    len: number;
    content: Uint8Array;
    packet!: IPPacket;
    context!: Context;
    constructor(name: string, readerCreator: AbstractReaderCreator, len: number, content: Uint8Array, ctx: Context) {
        this.name = name;
        this.readerCreator = readerCreator;
        this.len = len;
        this.content = content;
        this.context = ctx;
    }
    accept(visitor: PVisitor): IPPacket {
        return visitor.visit(this)
    }
    log(...arg) {
        if (process.env.NODE_ENV === "DEBUG") {
            console.log(this.name, ...arg)
        }
    }
    info(...arg) {
        console.log(this.name, ...arg)
    }
}

export interface Visitor {
    visit(ele: BasicElement): Packet;
}
export interface PVisitor extends Visitor {
    visit(ele: BasicElement): IPPacket;
}
export abstract class AbstractVisitor implements PVisitor {
    abstract visit(ele: BasicElement): IPPacket;
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
    constructor(ip: string, mac: string){
        this.ip = ip;
        this.mac = mac;
    }
}
export class ARPReply {
    host: CNode;
    clients: CNode[] = [];
    constructor(host: CNode){
        this.host = host;
    }
}
export interface Context {
    getFrames():IPPacket[];
    getCurrentIndex(): number;
    getFileInfo(): FileInfo;
    resolve(p: ARP):void;
    getARPReplies(): ARPReply[];
    resolveTCP(p: TCP): PVisitor;
    getTCPConnections(): TCPConnect[];
    // resolve( n: string):void;
    // resolve(p: Packet):void;
}

export abstract class AbstractRootVisitor implements Visitor, Context {
    resolver: Resolver = new Resolver();
    readonly packets: IPPacket[] = []
    index: number = 0;
    getFrames(): IPPacket[] {
        return this.packets;
    }
    getCurrentIndex(): number {
        return this.index;
    }
    abstract getFileInfo(): FileInfo;
    protected getNextIndex(): number{
        this.index += 1;
        return this.index;
    }
    protected addPacket(packet: IPPacket): void {
        packet.setIndex(this.getNextIndex());
        this.packets.push(packet);
    };
    public getContext(): Context {
        return this;
    }
    resolve(p: ARP):void {
        const { oper } = p;
        if(oper === 2){
            const sourceKey = `${p.senderMac}@${p.senderIp}`;
            let list = this.resolver.arpMap.get(sourceKey);
            if(!list){
                list = new Set();
                this.resolver.arpMap.set(sourceKey, list);
            }
            list.add(`${p.targetMac}@${p.targetIp}`);
        }
    }
    resolveTCP(p: TCP): PVisitor{
        const resolver = this.resolver;
        const noContent = p.ack && !p.psh && p.packet.length < 10;
        const [arch, ip1, port1, ip2, port2] = p.mess();
        const key = `${ip1}${port1}-${ip2}${port2}`;
        let connect = resolver.tcpCache.get(key);
        if (!connect) {
            if (noContent) return;
            connect = new TCPConnect(ip1, port1, ip2, port2);
            resolver.tcpCache.set(key, connect);
        }
        const sequence = p.sequence;
        const nextSequence = p.sequence + p.packet.length;
        const stack = connect.getStack(arch);
        const dump = stack.checkDump(sequence, nextSequence);
        p.isDump = dump;
        connect.count += 1;
        connect.total += p.getProtocal(Protocol.ETHER).packet.length;
        connect.tcpSize += p.packet.length;
        if (dump) {
            return null;
        }
        connect.tcpUse += p.packet.length;
        connect.countUse += 1;
        stack.sequence = sequence;
        stack.next = nextSequence;
        const stackRec = connect.getStack(!arch);
        stackRec.ack = p.acknowledge;
        if (p.ack) {

        }
        if (p.ack && !p.psh) {
            if (p.packet.length > 10) {
                const len = p.getProtocal(Protocol.ETHER).packet.length;
            }
        }
        if (p.psh) {
        }

        const payload = p.packet;

        return null;
    }
    getTCPConnections(): TCPConnect[]{
        return this.resolver.tcpConnections;
    }
    getARPReplies(): ARPReply[]{
        const arp = this.resolver.arpMap;
        const hostnames = arp.keys();
        const rs: ARPReply[] = [];
        arp.forEach((values, hostname) => {
            const [mac, ip] = hostname.split('@');
            const reply = new ARPReply(new CNode(ip,mac));
            values.forEach((val) => {
                const [mac, ip] = val.split('@');
                reply.clients.push(new CNode(ip,mac));
            });
            rs.push(reply);
        });
        return rs;
    }

    getHTTPConnects(): void {}
    abstract _visit(ele: BasicElement): void;
    visit(ele: BasicElement): Packet {
        const { readerCreator, content } = ele;
        const reader = readerCreator.createReader(content, 'root', false);
        const start = Date.now();
        this._visit(ele);
        const per = Date.now() - start;
        this.resolver.flush(null);
        return null;
    }
}

export class BasicEmptyVisitor extends AbstractVisitor {
    persist: boolean
    name: string
    constructor(name: string) {
        super()
        this.name = name;
        this.persist = false;
    }
    visit(ele: BasicElement): IPPacket {
        const { name, readerCreator, content } = ele;
        const prefix = name + '/' + this.name;
        const reader = readerCreator.createReader(content, prefix, this.persist);
        return this.next(ele, reader);
    }
    next(ele: BasicElement, reader: Uint8ArrayReader): IPPacket {
        if (process.env.NODE_ENV === "DETECT") {
            process.exit(0)
        }
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