import { PVisitor, PacketElement, IPPacket, Protocol, IPProvider } from '../common';
import { etypeMap, ipProtocolMap, ARP_OPER_TYPE_MAP, ARP_HARDWARE_TYPE_MAP } from '../common/constant';
import { IPAddress } from '../common/io';
import { TCPVisitor, UDPVisitor, ICMPVisitor, IGMPVisitor, ICMPV6Visitor } from './transportLayer';

export class IPPack extends IPProvider {
    _source: IPAddress;
    _target: IPAddress;
    getSourceIp(): IPAddress {
        return this._source;
    }
    getTargetIp(): IPAddress {
        return this._target;
    }
}
export class IPv4 extends IPPack {
    version: number;
    totalLen: number;
    identification: number;
    ttl: number;
    ipprotocol: number;
    extra: any;
    toString(): string {
        return `Internet Protocol Version : ${this.version} length: ${this.totalLen}`;
    }
    getProtocalType(): string {
        return ipProtocolMap[this.ipprotocol];
    }
    summary(): string {
        return `Internet Protocol Version: 4 Src: ${this._source.getAddress()}  Dst: ${this._target.getAddress()}`;
    }
    createSummary(field: string): string {
        switch (field) {
            // case 'head':
            //     return [`IP Protocol Vesion: (${this.source})`];
            case 'source':
                return `Source IP Address: (${this.getSourceIp().getAddress()})`;
            case 'target':
                return `Destination IP Address: (${this.getTargetIp().getAddress()})`;
            case 'totalLen':
                return `Total length: 0x${this.totalLen.toString(16).padStart(4, '0')} (${this.totalLen})`;
            case 'identification':
                return `Identification: 0x${this.identification.toString(16).padStart(4, '0')} (${this.identification})`;
            case 'ttl':
                return `Time to Live: ${this.ttl}`;
            case 'ipprotocol':
                return `Protocol: ${this.getProtocalType()} (${this.ipprotocol})`;
        }
        return null;
    }
}

export class IPv6 extends IPPack {
    nextHeader: number;
    hop: number;
    plen: number;
    summary(): string {
        return `Internet Protocol Version: 6 Src: ${this._source.getAddress()}  Dst: ${this._target.getAddress()}`;
    }
    createSummary(field: string): string {
        switch (field) {
            case 'nextHeader':
                return `Protocol: ${this.nextHeader}`;
            case 'source':
                return `Source IP Address: (${this.getSourceIp().getAddress()})`;
            case 'target':
                return `Destination IP Address: (${this.getTargetIp().getAddress()})`;
        }
        return null;
    }
}

export class ARP extends IPPack {
    //https://en.wikipedia.org/wiki/Address_Resolution_Protocol
    oper: number;//1 request 2 reply
    senderMac: string;
    senderIp: IPAddress;
    targetMac: string;
    targetIp: IPAddress;
    hardwareType: number;
    protocolType: number;
    hardwareSize: number;
    protocolSize: number;
    getSourceIp(): IPAddress {
        return this.senderIp;
    }
    getTargetIp(): IPAddress {
        return this.targetIp;
    }
    public getOperation(): string {
        return ARP_OPER_TYPE_MAP[this.oper];
    }
    public getHardwareType(): string {
        return ARP_HARDWARE_TYPE_MAP[this.hardwareType];
    }
    public getProtocolType(): string {
        const code = '0x' + this.protocolType.toString(16).padStart(4, '0');
        return etypeMap[code];
    }
    toString(): string {
        if (this.oper === 1) {
            if (this.senderIp.getAddress() === this.targetIp.getAddress()) {
                return `ARP Announcement for ${this.senderIp.getAddress()}`;
            }
            if (this.senderIp.getAddress() === '0.0.0.0') {
                return `who has ${this.targetIp.getAddress()}? (ARP probe)`;
            }
            return `who has ${this.targetIp.getAddress()}? tell ${this.senderIp.getAddress()}`;
        }
        return `${this.senderIp.getAddress()} at ${this.senderMac}`;
    }
    summary(): string {
        return `Address Resolution Protocol (${this.getOperation()})`
    }

    createSummary(field: string): string {
        switch (field) {
            case 'htype':
                return `Hardware type: ${this.getHardwareType()} (${this.hardwareType})`;
            case 'ptype':
                const code = '0x' + this.protocolType.toString(16).padStart(4, '0');
                return `Protocol type: ${this.getProtocolType()} (${code})`;
            case 'hlen':
                return `Hardware size: ${this.hardwareSize} bytes`;
            case 'plen':
                return `Protocol size: ${this.protocolSize} bytes`;
            case 'oper':
                return `Operation code ${this.getOperation()} (${this.oper})`;
            case 'senderMac':
                return `Sender Mac address: ${this.senderMac}`;
            case 'senderIp':
                return `Sender IP address: ${this.senderIp.getAddress()}`;
            case 'targetMac':
                return `Target Mac address: ${this.targetMac}`;
            case 'targetIp':
                return `Target IP address: ${this.targetIp.getAddress()}`;
        }
        return null;
    }
}
// https://en.wikipedia.org/wiki/Internet_Protocol_version_4
export class IPv4Visitor implements PVisitor {
    mapper: Map<number, PVisitor> = new Map();
    constructor() {
        this.mapper.set(6, new TCPVisitor())
        this.mapper.set(17, new UDPVisitor())
        this.mapper.set(1, new ICMPVisitor())
        this.mapper.set(2, new IGMPVisitor())
        //https://en.wikipedia.org/wiki/List_of_IP_protocol_numbers
    }
    visit(ele: PacketElement): IPPacket {
        const parent = ele.getPacket();
        const { reader } = parent;
        const data = new IPv4(parent, reader, Protocol.IPV4);

        const cv = data.read8('head');
        data.version = cv >>> 4;
        const headLenth = cv & 0x0f;
        const tos = data.read8('tos');
        data.totalLen = data.read16('totalLen', false);
        data.identification = data.read16('identification');
        const flag = data.read16('flag', true) >>> 13
        data.ttl = data.read8('ttl');
        data.ipprotocol = data.read8('ipprotocol')
        const headCRC = data.read16('crc')
        data._source = data.readIp('source');
        data._target = data.readIp('target');
        if (headLenth > 5) {
            reader.skip((headLenth - 5) * 4)
        }

        data.extra = { cv, headLenth, tos, flag, headCRC }

        return data.accept(this.mapper.get(data.ipprotocol));
    }
}

//https://en.wikipedia.org/wiki/Address_Resolution_Protocol
export class ARPVisitor implements PVisitor {
    visit(ele: PacketElement): IPPacket {
        const parent = ele.getPacket();
        const { reader } = parent;
        const data = new ARP(parent, reader, Protocol.ARP);

        //This field specifies the network link protocol type. Example: Ethernet is 1
        data.hardwareType = data.read16('htype', false);
        // This field specifies the internetwork protocol for which the ARP request is intended
        data.protocolType = data.read16('ptype', false);
        //Length (in octets) of a hardware address. Ethernet address length is 6.
        data.hardwareSize = data.read8('hlen');
        //Length (in octets) of internetwork addresses. The internetwork protocol is specified in PTYPE. Example: IPv4 address length is 4.
        data.protocolSize = data.read8('plen');
        //Specifies the operation that the sender is performing: 1 for request, 2 for reply.
        data.oper = data.read16('oper', false);
        data.senderMac = data.readHex('senderMac', 6, ':');
        data.senderIp = data.readIp('senderIp')
        data.targetMac = data.readHex('targetMac', 6, ':');
        data.targetIp = data.readIp('targetIp');
        ele.getContext().resolve(data);
        return data;
    }
}

export class IPv6Visitor implements PVisitor {
    mapper: Map<number, PVisitor> = new Map();
    constructor() {
        this.mapper.set(6, new TCPVisitor())
        this.mapper.set(17, new UDPVisitor())
        this.mapper.set(58, new ICMPV6Visitor())
        //https://en.wikipedia.org/wiki/List_of_IP_protocol_numbers
    }
    visit(ele: PacketElement): IPPacket {
        const parent = ele.getPacket();
        const { reader } = parent;
        const data = new IPv6(parent, reader, Protocol.IPV6);

        reader.read32();
        data.plen = data.read16('plen', false);
        data.nextHeader = data.read8('nextHeader');
        data.hop = data.read8('hop');
        //https://en.wikipedia.org/wiki/IPv6_address
        data._source = data.readIp6('source');
        data._target = data.readIp6('target');
        return data.accept(this.mapper.get(data.nextHeader));
    }
}