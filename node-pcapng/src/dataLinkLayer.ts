import { PVisitor, BasicElement, IPPacket, Protocol } from './common';
import {IPv4Visitor, IPv6Visitor, ARPVisitor} from './networkLayer';

export class DataPacket extends IPPacket {
    target: string;
    source: string;
    type: string;
    toString(): string {
        return `src: () ${this.source} Dst: () ${this.target}`;
    }
}
export class DataLaylerVisitor implements PVisitor {
    mapper: Map<string, PVisitor> = new Map();
    constructor(){
        this.mapper.set('0800', new IPv4Visitor());
        this.mapper.set('86dd', new IPv6Visitor());
        this.mapper.set('0806', new ARPVisitor());
    }
    visit(ele: BasicElement): IPPacket {
        const { name, readerCreator, content } = ele;
        const prefix = `${name}/data`
        const reader = readerCreator.createReader(content, prefix, false);
        const target = reader.readHex(6, ':');
        const source = reader.readHex(6, ':');
        const type = reader.readHex(2, '');
        const _packet = reader.slice(content.length - 14);
        const data = new DataPacket(ele.packet, _packet, Protocol.MAC);
        data.target = target;
        data.source = source;
        data.type = type;
        const nextVisitor = this.mapper.get(type);
        if(nextVisitor){
            return data.createSubElement(prefix, ele).accept(nextVisitor);
        }
        return data;
    }
}