const read64 = (arr: Uint8Array, offset: number, littleEndian = true): bigint => {
    const dataView = new DataView(arr.buffer, offset, 8);
    return dataView.getBigUint64(0, littleEndian);
}
const read32 = (arr: Uint8Array, offset: number, littleEndian = true): number => {
    const dataView = new DataView(arr.buffer, offset, 4);
    return dataView.getUint32(0, littleEndian);
}
export const read16 = (arr: Uint8Array, offset: number, littleEndian = true): number => {
    const dataView = new DataView(arr.buffer, offset, 2);
    return dataView.getUint16(0, littleEndian);
}
export const read8 = (arr: Uint8Array, offset: number): number => {
    const dataView = new DataView(arr.buffer, offset, 1);
    return dataView.getUint8(0)
}

export interface IPAddress {
    getAddress(): string;
}
export class IP4Address implements IPAddress {
    constructor(private arr: Uint8Array){}
    getAddress(): string {
        return this.arr.reduce((acc, current, index) => acc + (index ? '.' : '') + current, '');
    }
}

export class IP6Address implements IPAddress {
    constructor(private arr: Uint8Array){}
    getAddress(): string {
        const groups: string[] = [];
        for(let i =0 ; i < 8; i += 1){
            groups.push(read16(this.arr, i * 2, true).toString(16));
        }
        const ad = groups.join(':');
        return ad.replace(/(:0)+/i, ':');
    }
}

const textDecoder = new TextDecoder('utf-8');
export class Uint8ArrayReader {
    arr: Uint8Array;
    cursor: number;
    size: number;
    constructor(arr: Uint8Array) {
        this.arr = arr;
        this.cursor = 0;
        this.size = arr.length;

    }
    read8(): number {
        const v = read8(this.arr, this.cursor);
        this.cursor += 1;
        return v;
    }
    readSpace(max: number = 10): string | null {
        for (let i = 0; i < max; i += 1) {
            if (this.arr[this.cursor + i] == 32) {
                const data = this.arr.slice(this.cursor, this.cursor + i);
                return textDecoder.decode(data);
            }
        }
        return null;
    }
    readEnter(): string {
        for (let i = 0; i < this.left(); i += 1) {
            if (this.arr[this.cursor + i] == 13 && this.arr[this.cursor + i + 1] == 10) {
                const data = this.arr.slice(this.cursor, this.cursor + i);
                this.skip(i + 2);
                return textDecoder.decode(data);
            }
        }
        return '';
    }
    read16(littleEndian = true): number {
        const v = read16(this.arr, this.cursor, littleEndian);
        this.cursor += 2;
        return v;
    }
    read32(littleEndian = true): number {
        const v = read32(this.arr, this.cursor, littleEndian);
        this.cursor += 4;
        return v;
    }
    tryReadTLS(): [number, number, number, number] {
        const type = read8(this.arr, this.cursor);
        const major = read8(this.arr, this.cursor + 1);
        const minor = read8(this.arr, this.cursor + 2);
        const len = read16(this.arr, this.cursor + 3, false);
        return [type, major, minor, len];
    }
    tryTLSMessage(): [number, number] {
        const type = read8(this.arr, this.cursor);
        const len = read8(this.arr, this.cursor + 1) * 256 * 256 + read16(this.arr, this.cursor + 2, false);
        // const major = read8(this.arr, this.cursor + 4);
        // const minor = read8(this.arr, this.cursor + 5);
        return [type, len];
    }
    readBig64(littleEndian = true): bigint {
        const v = read64(this.arr, this.cursor, littleEndian);
        this.cursor += 8;
        return v;
    }
    read32Hex(): string {
        return this.read32().toString(16).toLowerCase().padStart(8, '0');
    }
    readHex(len: number, flag = ''): string {
        const v = this.slice(len);
        return v.reduce((acc, current, index) => acc + (index ? flag : '') + current.toString(16).padStart(2, '0'), '');
    }
    readDec(len: number, flag = '.'): string {
        const v = this.slice(len);
        return v.reduce((acc, current, index) => acc + (index ? flag : '') + current.toString(10), '');
    }
    readIp(): IP4Address {
        return new IP4Address(this.slice(4));
    }
    readIpv6(): IP6Address {
        return new IP6Address(this.slice(16));
    }
    readString(len: number): string {
        const data = this.slice(len)
        return new TextDecoder().decode(data);
    }
    readNBNSString(len: number) {
        const words = Math.floor(len / 2);
        const real = [];
        for (let i = 0; i < words; i += 1) {
            const n = this.read8() - 65;
            const m = this.read8() - 65;
            const v = n * 16 + m;
            if (v === 32) {
                continue;
            }
            if (v === 0) {
                continue;
            }
            real.push(v);
        }
        return new TextDecoder().decode(new Uint8Array(real));
    }
    readNBNSQuery(): string {
        let _size = 0;
        const list = [];
        do {
            _size = this.read8();
            if (_size > 0) {
                const str = this.readNBNSString(_size)
                list.push(str)
            }
        } while (_size > 0)
        return list.join('.')
    }
    readDNSQuery(): string {
        let _size = 0;
        const list = [];
        do {
            _size = this.read8();
            if (_size > 0) {
                const str = this.readString(_size)
                list.push(str)
            }
        } while (_size)
        return list.join('.')
    }
    
    readCompressStringWithRef(): [string, number] {
        const list = [];
        while(true) {
            if(this.left() === 2){
                return ['', this.read16(false)]
            }
            const _size = this.read8();
            if (_size > 0) {
                const str = this.readString(_size)
                list.push(str)
            }
            const nextVal = read8(this.arr, this.cursor);
            if(nextVal === 0) {
                return [list.join('.'), 0 ]
            }
            if(nextVal >= 0xc0){
                return [list.join('.'), this.read16(false)]
            }
            if(nextVal > this.left()){
                return [list.join('.'), 0 ]
            }
        }
    }
    readDNSAnswer(len: number): [string, number] {
        let _len = len;
        let _size = 0;
        const list = [];
        do {
            _size = this.read8();
            _len -= 1;
            if (_size > 0) {
                const str = this.readString(_size)
                _len -= _size;
                list.push(str)
            }
        } while (_size && _len > 2)
        return [list.join('.'), this.read16(false)]
    }
    left(): number {
        return this.size - this.cursor
    }
    slice(len: number): Uint8Array {
        const v = this.arr.slice(this.cursor, this.cursor + len);
        this.skip(len)
        return v;
    }
    extra(): Uint8Array {
        return this.slice(this.size - this.cursor)
    }
    extra2(): Uint8Array {
        return this.arr.slice(this.cursor, this.size);
    }
    skip(len: number): void {
        this.cursor += len;
    }
    // pad(size){
    //     const mod = this.cursor % size;
    //     if(mod > 0){
    //         this.skip(size - mod);
    //     }
    // }
    eof(): boolean {
        return this.cursor < this.size
    }
}

// export class DebugReader extends Uint8ArrayReader {
//     constructor(folder: string, name: string, arr: Uint8Array) {
//         super(arr);
//         const buf = Buffer.from(arr);
//         const ff = path.join(folder, name);
//         fs.mkdirSync(ff, { recursive: true });
//         fs.writeFileSync(path.join(ff, 'bin.hex'), buf)
//     }
// }

export class AbstractReaderCreator {
    folder?: string;
    createReader(arr: Uint8Array, name: string, debug: boolean): Uint8ArrayReader {
        return new Uint8ArrayReader(arr)
    }
}

// export class DebugReaderCreator extends AbstractReaderCreator {
//     constructor(folder: string) {
//         super()
//         this.folder = folder;
//     }
//     createReader(arr: Uint8Array, name: string, debug: boolean): Uint8ArrayReader {
//         if (debug) {
//             return new DebugReader(this.folder, name, arr)
//         }
//         return super.createReader(arr, name, debug);
//     }

// }

