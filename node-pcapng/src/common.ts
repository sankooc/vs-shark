import { AbstractReaderCreator, Uint8ArrayReader } from './io';
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
}

export class Packet {
    packet: Uint8Array;
    constructor(packet: Uint8Array) {
        this.packet = packet;
    }
    getPacket(): Uint8Array {
        return this.packet
    }
    toString(): string{
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
        const ele = new BasicElement(name, parent.readerCreator, this.packet.length, this.packet);
        ele.packet = this;
        ele.resolver = parent.resolver;
        return ele;
    }
    getIndex(): number {
        if(this.parent){
            return this.parent.getIndex();
        }
        return this.index;
    }
    // getPacket(): Uint8Array {
    //     return this.parent.getPacket();
    // }
    // getProtocal(): Protocol {
    //     return this.protocol;
    // }
    getProtocal(name: Protocol): IPPacket {
        if (this.protocol === name) {
            return this;
        }
        if(this.parent){
            return this.parent.getProtocal(name);
        }
        return null;
    }
}

// class ComposePacket extends IPPacket {
//     parent: IPPacket;
//     constructor(parent: IPPacket, protocol: Protocol) {
//         super();
//         this.protocol = protocol;
//         this.parent = parent;
//     }
//     getIndex(): number {
//         return this.parent.getIndex();
//     }
//     getPacket(): Uint8Array {
//         return this.parent.getPacket();
//     }
//     getProtocal(name: Protocol): IPPacket {
//         if (this.protocol === name) {
//             return this;
//         }
//         return super.getProtocal();
//     }
// }

export interface PElement {
    accept(visitor: PVisitor): IPPacket;
}
export class TCPStack {
    ip: string;
    port: number;
    sequence: number = 0;
    next: number = 0;
    ack: number = 0;
    finished: boolean = false;
    constructor(ip: string, port: number){
        this.ip = ip;
        this.port = port;
    }
    checkDump(sequence: number, next: number){
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
    constructor(ip1: string, port1: number,ip2: string, port2: number){
        this.ep1 = new TCPStack(ip1, port1);
        this.ep2 = new TCPStack(ip2, port2);
    }
    getConnectName(): string {
        return `${this.ep1.ip}:${this.ep1.port}-${this.ep2.ip}:${this.ep2.port}`;
    }
    getStack(arch: boolean): TCPStack{
        if(arch) return this.ep1;
        return this.ep2;
    }
    finish(){}
}
export class Resolver {
    tcpConnections: TCPConnect[] = [];
    tcpCache: Map<string, TCPConnect> = new Map();
    flush(key: string): void {
        if(!key){
            this.tcpCache.forEach((value) => {
                this.tcpConnections.push(value);
            })
            this.tcpCache.clear();
            return;
        }
        const connect = this.tcpCache.get(key);
        this.tcpCache.set(key, null);
        if(connect){
            this.tcpConnections.push(connect)
        }
    }
}

export class BasicElement implements PElement {
    name: string;
    readerCreator: AbstractReaderCreator;
    len: number;
    content: Uint8Array;
    packet: IPPacket;
    resolver: Resolver;
    constructor(name: string, readerCreator: AbstractReaderCreator, len: number, content: Uint8Array) {
        this.name = name;
        this.readerCreator = readerCreator;
        this.len = len;
        this.content = content;
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


export abstract class AbstractRootVisitor implements Visitor {
    // connections: Set<string> = new Set<string>();
    // packets: AbstractVisitor[] = [];
    abstract visit(ele: BasicElement): Packet;
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

// export class Block {
//     type: number;
//     len: number;
//     body: any;
//     init() { }
//     constructor(type, len, body) {
//         this.type = type;
//         this.len = len;
//         this.body = body;
//         this.init();
//     }
// }
// export class SectionHeader extends Block {
//     byteOrderMagic: any;
//     majorVersion: number;
//     minorVersion: number;
//     option: any;
//     init() {
//         // this.body;
//         this.byteOrderMagic;
//         this.majorVersion;
//         this.minorVersion;
//         this.option
//     }
// }
export class Option {
    code: number;
    value: any;
    constructor(code, value) {
        this.code = code;
        this.value = value;
    }
}

export class Spec {

}