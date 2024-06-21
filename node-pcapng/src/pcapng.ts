import { Option, AbstractVisitor, Visitor, Packet, Protocol, IPPacket, Resolver, PVisitor, BasicElement, AbstractRootVisitor } from "./common";
import { Uint8ArrayReader, AbstractReaderCreator } from './io';
import { DataLaylerVisitor } from './dataLinkLayer';
import {TCP, UDP} from './transportLayer';
// import { EventEmitter } from 'events';
//https://www.ietf.org/archive/id/draft-tuexen-opsawg-pcapng-05.html

const opt_endofopt = 0
const OPTION_CODE = {
    2: 'if_name',
    3: 'if_description',
    4: 'if_IPv4addr',
    5: 'if_IPv6addr',
    6: '',
}

const readOption = (opt: BasicElement, reader: Uint8ArrayReader): Option => {
    const optionCode = reader.read16();
    const optionLen = reader.read16();
    if (optionLen === opt_endofopt) { return }
    const optionValue = reader.slice(optionLen);
    // reader.pad(4)
    const mod = optionLen % 4;
    if (mod > 0) {
        reader.skip((4 - mod))
    }
    opt.log("option:" + optionLen)
    const textDecoder = new TextDecoder('utf-8');
    const text = textDecoder.decode(optionValue);
    return new Option(optionCode, text)
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
    visit(ele: BasicElement) {
        console.error('unimplement pcapng type:' + this.type);
        if (process.env.NODE_ENV === "DETECT") {
            process.exit(0)
        }
        return null;
    }
}

class SectionHeaderPacket extends Packet {
    options: Option[] = [];
    major: number;
    minor: number;
    orderMagic: string;
}
class SectionHeaderVisitor extends BasicVisitor {
    visit(ele: BasicElement): SectionHeaderPacket {
        const { name, readerCreator, content } = ele;
        const reader = readerCreator.createReader(content, name + ':section', false);
        const orderMagic = reader.read32Hex();
        const major = reader.read16();
        const minor = reader.read16();
        reader.skip(8)
        ele.log('parseSectionHeader', orderMagic, major, minor);
        ele.info('section option');
        const data = new SectionHeaderPacket(null);
        data.orderMagic = orderMagic;
        data.major = major;
        data.minor = minor;
        while (true) {
            const option = readOption(ele, reader);
            if (!option) {
                break;
            }
            data.options.push(option)
            ele.info(option.code, option.value)
        }
        return data;
    }
}
class InterfaceDescription extends BasicVisitor {
    visit(ele: BasicElement): Packet {
        const { name, readerCreator, content } = ele;
        const reader = readerCreator.createReader(content, name + ':interface', false);
        const linktype = reader.read16();
        const reserved = reader.read16();
        const snapLen = reader.read32();
        ele.info('interface', linktype, reserved, snapLen)
        return null;
    }

}

class EnhancedPacketVisitor extends BasicVisitor {
    visitor: DataLaylerVisitor;
    index: number = 1;
    constructor(type: string){
        super(type);
        this.visitor = new DataLaylerVisitor();
    }
    visit(ele: BasicElement): IPPacket {
        const { name, readerCreator, content } = ele;
        const prefix = `${name}:enhanced`
        const reader = readerCreator.createReader(content, prefix, false);
        const interfaceId = reader.read32();
        const highTS = reader.read32();
        const lowTS = reader.read32();
        ele.log('interface', interfaceId, highTS, lowTS)
        const capturedPacketLength = reader.read32();
        const originalPacketLength = reader.read32();
        ele.log('packet size', capturedPacketLength, originalPacketLength)
        const packet = reader.slice(capturedPacketLength)
        const mod = originalPacketLength % 4;
        if (mod > 0) {
            reader.skip((4 - mod))
        }
        const subPacket = new IPPacket(null, packet, Protocol.ETHER);
        subPacket.index = this.index;
        this.index += 1;

        return subPacket.createSubElement(prefix, ele).accept(this.visitor);
    }
}

class InterfaceStatistic extends BasicVisitor {
    visit(ele: BasicElement): Packet {
        const { name, readerCreator, content } = ele;
        const reader = readerCreator.createReader(content, name + ':statistic', false);
        const interfaceId = reader.read32();
        const highTS = reader.read32();
        const lowTS = reader.read32();
        ele.log('statistic', interfaceId, highTS, lowTS)
        return null;
    }
}
export class RootVisitor implements Visitor {
    head: SectionHeaderPacket;
    sectionHeaderVisitor: SectionHeaderVisitor = new SectionHeaderVisitor('0a0d0d0a');
    interfaceDescription: InterfaceDescription = new InterfaceDescription('00000001');
    interfaceStatistic: InterfaceStatistic = new InterfaceStatistic('00000005');
    enhancedPacketVisitor: EnhancedPacketVisitor = new EnhancedPacketVisitor('00000006');
    resolver: Resolver = new Resolver();
    packets: IPPacket[] = []
    eventTable: EventTarget = new EventTarget();
    batchSize: number = 10;
    emitPacket(packet: IPPacket[]): void {
        const evt: CustomEvent<IPPacket[]> = new CustomEvent("frame", {detail: packet})
        this.eventTable.dispatchEvent(evt);
    }
    addEventListener(type:string, cb: (evt: CustomEvent<any>) => void){
        this.eventTable.addEventListener(type, cb);
    }
    createElement(name: string, readerCreator: AbstractReaderCreator, content: Uint8Array): BasicElement{
        const ele = new BasicElement(name, readerCreator, content.length, content);
        ele.resolver = this.resolver;
        return ele
    }
    visit(ele: BasicElement): Packet {
        const { readerCreator, content } = ele;
        const reader = readerCreator.createReader(content, 'root', false);
        const start = Date.now();
        let count = 0;
        this.eventTable.dispatchEvent(new CustomEvent<any>("init"));

        let templateArray = [];
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
                        this.interfaceDescription.visit(ele);
                        break;
                    case '00000005':
                        this.interfaceStatistic.visit(ele);
                        break;
                    case "00000006":
                        try {
                            const packet: IPPacket = this.enhancedPacketVisitor.visit(ele);
                            this.packets.push(packet);
                            templateArray.push(packet);
                            if((templateArray.length % this.batchSize) === 0){
                                this.emitPacket(templateArray)
                                templateArray = [];
                            }
                        } catch(e) {
                            console.error(e)
                        }
                        break;
                    // case '00000003':
                    // simplePacket(opt)
                    // case '00000004':
                    // nameResolution(opt)
                    // case "00000009":
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
        if(templateArray.length){
            this.emitPacket(templateArray);
            templateArray = [];
        }
        this.resolver.flush(null);
        this.eventTable.dispatchEvent(new CustomEvent<any>("finish"));
        console.log('finish')
        return null;
    }
}