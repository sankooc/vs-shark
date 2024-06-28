import { PVisitor, PacketElement, IPPacket, Protocol } from './common';
import {IPv4Visitor, IPv6Visitor, ARPVisitor} from './networkLayer';

export class DataPacket extends IPPacket {
    target: string;
    source: string;
    type: string;
    toString(): string {
        return `Ethernet II, src: () ${this.source} Dst: () ${this.target}`;
    }
}
export class DataLaylerVisitor implements PVisitor {
    mapper: Map<string, PVisitor> = new Map();
    constructor(){
        this.mapper.set('0800', new IPv4Visitor());
        this.mapper.set('86dd', new IPv6Visitor());
        this.mapper.set('0806', new ARPVisitor());
    }
    visit(ele: PacketElement): IPPacket {
        const parent = ele.getPacket();
        const { reader } = parent;
        const data = new DataPacket(parent, reader, Protocol.MAC);
        data.target = reader.readHex(6, ':');
        data.source = reader.readHex(6, ':');
        data.type = reader.readHex(2, '');
        return data.accept(this.mapper.get(data.type));
    }
}