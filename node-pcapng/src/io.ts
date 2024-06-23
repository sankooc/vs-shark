// import fs from 'node:fs'fs
// import path from 'path'


const read64 = (arr: Uint8Array, offset: number, littleEndian = true): bigint => {
    const dataView = new DataView(arr.buffer, offset, 8);
    return dataView.getBigUint64(0, littleEndian);
}
const read32 = (arr: Uint8Array, offset: number, littleEndian = true): number => {
    const dataView = new DataView(arr.buffer, offset, 4);
    return dataView.getUint32(0, littleEndian);
}
const read16 = (arr: Uint8Array, offset: number, littleEndian = true): number => {
    const dataView = new DataView(arr.buffer, offset, 2);
    return dataView.getUint16(0, littleEndian);
}
const read8 = (arr: Uint8Array, offset: number): number => {
    const dataView = new DataView(arr.buffer, offset, 1);
    return dataView.getUint8(0)
}

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
    readIp(): string {
        return this.readDec(4, '.');
    }
    readString(len: number): string {
        const data = this.slice(len)
        return new TextDecoder().decode(data);
    }
    readDNSQuery(): string{
        let _size = 0;
        const list = [];
        do{
            _size = this.read8();
            if(_size > 0) {
                const str = this.readString(_size)
                list.push(str)
            }
        }while(_size)
        return list.join('.')
    }
    readDNSAnswer(len: number): [string, number]{
        let _len = len;
        let _size = 0;
        const list = [];
        do{
            _size = this.read8();
            _len -= 1;
            if(_size > 0) {
                const str = this.readString(_size)
                _len -= _size;
                list.push(str)
            }
        }while(_size && _len > 2)
        return [list.join('.'), this.read16(false)]
    }
    left(): number{
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
    skip(len:number): void {
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
    folder: string;
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

