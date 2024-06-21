import { Uint8ArrayReader } from '../io.js';
export class IpLayer{
    version: number;
    headLenth: number;
    tos: number;
    totalLen: number;
    identification: number;
    flag: number;
    ttl: number;
    protocol: number;
    headCRC: number;
    reader: Uint8ArrayReader;
    source: string;
    target: string;
    constructor(packet: Uint8Array){
        this.reader = new Uint8ArrayReader(packet);
        const cv = this.reader.read8();
        this.version = cv >>> 4;
        this.headLenth = cv & 0x0f;
        this.tos = this.reader.read8();
        this.totalLen = this.reader.read16();
        this.identification = this.reader.read16();
        this.flag = this.reader.read16() >>> 13
        this.ttl = this.reader.read8();
        this.protocol = this.reader.read8()
        this.headCRC = this.reader.read16()
        this.source = this.reader.read32Hex()
        this.target = this.reader.read32Hex()
        console.log(this.version, this.headLenth)
        console.log(this.source, this.target)
    }
}