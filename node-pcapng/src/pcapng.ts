import { Option, AbstractVisitor, Visitor, Packet, Protocol, FileInfo,IPPacket, EtherPacket, Context, Resolver, InputElement, PVisitor, AbstractRootVisitor, PacketElement } from "./common";
import { Uint8ArrayReader, AbstractReaderCreator } from './io';
import { DataLaylerVisitor } from './dataLinkLayer';
import { linktypeMap } from './constant';
//https://www.ietf.org/archive/id/draft-tuexen-opsawg-pcapng-03.html

const opt_endofopt = 0

const OPTION_CODE = {
    2: 'if_name',
    3: 'if_description',
    4: 'if_IPv4addr',
    5: 'if_IPv6addr',
    6: '',
}

const readOption = (opt: PacketElement, reader: Uint8ArrayReader, isRaw: boolean = false): Option => {
    const optionCode = reader.read16();
    const optionLen = reader.read16();
    if (optionLen === opt_endofopt) { return }
    const optionValue = reader.slice(optionLen);
    // reader.pad(4)
    const mod = optionLen % 4;
    if (mod > 0) {
        reader.skip((4 - mod))
    }
    if (isRaw) {
        return new Option(optionCode, optionValue, optionLen)
    }
    const textDecoder = new TextDecoder('utf-8');
    const text = textDecoder.decode(optionValue);
    return new Option(optionCode, text, optionLen)
}

//https://www.ietf.org/staging/draft-tuexen-opsawg-pcapng-02.html#name-enhanced-packet-block

//https://www.ietf.org/staging/draft-tuexen-opsawg-pcapng-02.html#name-simple-packet-block
const simplePacket = (opt) => { }
//https://www.ietf.org/staging/draft-tuexen-opsawg-pcapng-02.html#name-name-resolution-block
const nameResolution = (opt) => { }
//https://www.ietf.org/staging/draft-tuexen-opsawg-pcapng-02.html#name-systemd-journal-export-bloc
const systemJournal = (opt) => { }

class BasicVisitor implements Visitor {
    type: string;
    constructor(type: string) {
        this.type = type;
    }
    visit(ele: SectionPacket) {
        console.error('unimplement pcapng type:' + this.type);
        if (process.env.NODE_ENV === "DETECT") {
            process.exit(0)
        }
        return null;
    }
}

class SectionHeaderPacket extends Packet {
    major: number;
    minor: number;
    orderMagic: string;
    hardware!: string;
    os!: string;
    userapp!: string;
    toString(): string {
        let str = `version: ${this.major}.${this.minor}`;
        if(this.hardware) {
            str += ` hw: ${this.hardware}`
        }
        if(this.os) {
            str += ` os: ${this.os}`
        }
        if(this.userapp) {
            str += ` client: ${this.userapp}`
        }
        return str;
    }
}
class InterfaceDescriptionPacket extends Packet {
    if!: InterfaceInfo;
}
class SectionHeaderVisitor extends BasicVisitor {
    visit(ele: SectionPacket): SectionHeaderPacket {
        const { readerCreator, content } = ele;
        const reader = readerCreator.createReader(content, 'section', false);
        const data = new SectionHeaderPacket(reader);
        const orderMagic = reader.read32Hex();
        const major = reader.read16();
        const minor = reader.read16();
        reader.skip(8)
        data.orderMagic = orderMagic;
        data.major = major;
        data.minor = minor;
        while (true) {
            const option = readOption(ele, reader);
            if (!option) {
                break;
            }
            switch (option.code) {
                case 2:
                    data.hardware = option.value;
                    break;
                case 3:
                    data.os = option.value;
                    break;
                case 4:
                    data.userapp = option.value;
                    break;

            }
        }
        data._end();
        return data;
    }
}
class InterfaceInfo {
    type: number;
    name!: string;
    description!: string;
    ip4addr: string[] = [];
    ip6addr: string[] = [];
    macaddr!: string;
    tsresol!: string;
    speed!: string;
    os!: string;
    hardwire!: string;
    txspeed!: string;
    rxspeed!: string;

}
class InterfaceDescription extends BasicVisitor {
    visit(ele: SectionPacket): InterfaceDescriptionPacket {
        const { readerCreator, content } = ele;
        const reader = readerCreator.createReader(content, 'interface', false);
        const data = new InterfaceDescriptionPacket(reader);
        const linktype = reader.read16();
        const reserved = reader.read16();
        const snapLen = reader.read32();
        data.if = new InterfaceInfo();
        data.if.type = linktype;
        while (true) {
            const option = readOption(ele, reader);
            if (!option) {
                break;
            }
            switch (option.code) {
                case 2:
                    data.if.name = option.value;
                    break;
                case 3:
                    data.if.description = option.value;
                    break
                case 4:
                    data.if.ip4addr.push(option.value)
                    break
                case 5:
                    data.if.ip6addr.push(option.value)
                    break
                case 6:
                    data.if.macaddr = option.value;
                    break
                case 8:
                    data.if.speed = option.value;
                    break
                case 9:
                    data.if.tsresol = option.value;
                    break
                case 12:
                    data.if.os = option.value;
                    break
                //TODO 
            }
        }
        data._end();
        return data;
    }

}

class EnhancedPacketVisitor extends BasicVisitor {
    visitor: DataLaylerVisitor = new DataLaylerVisitor();
    visit(ele: SectionPacket): IPPacket {
        const { readerCreator, content } = ele;
        const reader = readerCreator.createReader(content, 'pre', false);
        const interfaceId = reader.read32();
        const highTS = reader.read32();
        const lowTS = reader.read32();
        const num = `0x${highTS.toString(16)}${lowTS.toString(16).padStart(8, '0')}`;
        const n = BigInt(num).toString();
        const capturedPacketLength = reader.read32();
        const originalPacketLength = reader.read32();
        const packet = reader.slice(capturedPacketLength)
        const mod = originalPacketLength % 4;
        if (mod > 0) {
            reader.skip((4 - mod))
        }
        
        const data = ele.getContext().createEtherPacket(readerCreator.createReader(reader.extra(), 'frame', false));
        data.interface = interfaceId;
        data.captured = capturedPacketLength;
        data.origin = originalPacketLength;
        data.ts = parseInt(n.substring(0, n.length - 3));
        data.nano = parseInt(n);
        return data.accept(this.visitor);
    }
}

class StaticsInfo extends Packet {
    isb_starttime!: Uint8Array; //2
    isb_endtime!: Uint8Array;//3
    isb_ifrecv!: Uint8Array;//4
    isb_ifdrop!: Uint8Array;//5
    isb_filteraccept!: Uint8Array;//6
    isb_osdrop!: Uint8Array;//7
    isb_usrdeliv!: Uint8Array;//8
}
class InterfaceStatistic extends BasicVisitor {
    visit(ele: SectionPacket): StaticsInfo {
        const { readerCreator, content } = ele;
        const reader = readerCreator.createReader(content, 'statistic', false);
        const data = new StaticsInfo(reader);
        const interfaceId = reader.read32();
        const highTS = reader.read32();
        const lowTS = reader.read32();
        while (true) {
            const option = readOption(ele, reader, true);
            if (!option) {
                break;
            }
            switch (option.code) {
                case 2:
                    data.isb_starttime = option.value;
                    break;
                case 3:
                    data.isb_endtime = option.value;
                    break;
                case 4:
                    data.isb_ifrecv = option.value;
                    break;
                case 5:
                    data.isb_ifdrop = option.value;
                    break;
                case 6:
                    data.isb_filteraccept = option.value;
                    break;
                case 7:
                    data.isb_osdrop = option.value;
                    break;
                case 8:
                    data.isb_usrdeliv = option.value;
                    break;
            }
        }
        data._end();
        return null;
    }
}


class SectionPacket implements PacketElement {
    constructor(public context: Context, public readerCreator: AbstractReaderCreator, public content: Uint8Array){}
    getContext(): Context{
        return this.context;
    }
    getPacket(): IPPacket {
        return null;
    }
    accept(visitor: PVisitor): IPPacket {
        return visitor.visit(this);
    }

}
export class PCAPNGVisitor extends AbstractRootVisitor {
    public head!: SectionHeaderPacket;
    public interface!: InterfaceInfo;
    public staticInfo!: StaticsInfo;
    private sectionHeaderVisitor: SectionHeaderVisitor = new SectionHeaderVisitor('0a0d0d0a');
    private interfaceDescription: InterfaceDescription = new InterfaceDescription('00000001');
    private interfaceStatistic: InterfaceStatistic = new InterfaceStatistic('00000005');
    private enhancedPacketVisitor: EnhancedPacketVisitor = new EnhancedPacketVisitor('00000006');

    getFileInfo(): FileInfo {
        const info = new FileInfo();
        if(this.head){
            info.os = this.head.os;
            info.client = this.head.userapp;
            info.hardware = this.head.hardware;
            info.majorVersion = this.head.major;
            info.minorVersion = this.head.minor;
        }
        if(this.interface){
            info.linkType = this.interface.type;
            info.interfaceName = this.interface.name;
            info.interfaceDesc = this.interface.description;
        }
        return info;
    }
    protected createElement(name: string, readerCreator: AbstractReaderCreator, content: Uint8Array): SectionPacket {
        return new SectionPacket(this, readerCreator, content);
    }
    _visit(ele: InputElement): void {
        const { readerCreator, content } = ele;
        const reader = readerCreator.createReader(content, 'root', false);
        let count = 1;
        const readBlock = () => {
            //https://www.ietf.org/staging/draft-tuexen-opsawg-pcapng-02.html
            const blockType = reader.read32Hex();
            const len = reader.read32();
            const _content = reader.slice(len - 12);
            const _len = reader.read32();
            if (len === _len) {
                const ele = this.createElement(`pkg:${count}`, readerCreator, _content);
                switch (blockType) {
                    case "0a0d0d0a":
                        this.head = this.sectionHeaderVisitor.visit(ele);
                        break;
                    case "00000001":
                        this.interface = this.interfaceDescription.visit(ele)?.if;
                        break;
                    case '00000005':
                        this.staticInfo = this.interfaceStatistic.visit(ele);
                        break;
                    case "00000006":
                        try {
                            const packet: IPPacket = this.enhancedPacketVisitor.visit(ele);
                            this.addPacket(packet);
                        } catch (e) {
                            console.error(e)
                        }
                        break;
                    default:
                        console.log('unknown type', blockType);
                }
                count += 1;
                return;
            }
            throw new Error("parse Error");
        }
        do {
            readBlock()
        } while (reader.eof())
    }
}