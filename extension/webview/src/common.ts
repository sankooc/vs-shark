import { DNSRecord, TCPConversation, FrameInfo } from 'rshark';

export class ComMessage<T> {
    type: string;
    body: T;
    id!: string;
    constructor(type: string, body: T) {
        this.type = type;
        this.body = body;
    }
}

export class ComLog {
    level: string;
    msg: any;
    constructor(level: string, msg: any) {
        this.level = level;
        this.msg = msg;
    }
}

export interface ColumnItem {
    style?: string;
    no?: number;
    getIndex(): number;
    getStyle(inx: number): string;
}

// export class IDNSRecord implements ColumnItem {
//     constructor(public readonly record: DNSRecord) { }
//     style?: string;
//     no?: number;
//     getIndex(): number {
//         return 0;
//     };
//     getStyle(inx: number): string {
//         return '';
//     }

// }

export class TCPCol implements ColumnItem {
    constructor(public item: TCPConversation){}
    no!: number;
    // ep1!: string;
    // ep2!: string;
    // total!: number;
    // tcp!: number;
    // tcpUse!: number;
    // count!: number;
    // countUse!: number;
    getIndex(): number {
        return 0;
    }
    getStyle(inx: number): string {
        return '';
    }
}


export class Category {
    name: string;
    index: number = 0;
    constructor(name: string) {
        this.name = name;
    }
}
export class GrapNode {
    private static index: number = 0;
    public static create(name: string, category: number): GrapNode {
        const instance = new GrapNode(name);
        instance.category = category;
        instance.id = GrapNode.index;
        GrapNode.index += 1;
        return instance;
    }
    id!: number;
    category!: number;
    name: string;
    extra!: string;
    constructor(name: string) {
        this.name = name;
    }
}
export class GrapLink {
    readonly source: number;
    readonly target: number;
    constructor(source: number, target: number) {
        this.source = source;
        this.target = target;
    }
}

export class Grap {
    categories: Category[] = [];
    nodes: GrapNode[] = [];
    links: GrapLink[] = [];
}

export class Frame implements ColumnItem {
    no!: number;
    time!: number;
    time_str?: string;
    source: string = 'n/a';
    dest: string = 'n/a';
    protocol!: string;
    iRtt: number = 0;
    len: number = 0;
    style: string = '';
    info!: string;
    public getIndex(): number {
        return this.no;
    }
    public getStyle(inx: number): string {
        if (this.no === inx) {
            return 'active';
        }
        return this.style;
    }

}
export class CTreeItem {
    key?: string;
    label: string;
    index?: [number, number];
    children: CTreeItem[] = [];
    constructor(label: string) {
        this.label = label;
    }
    append(label: string): CTreeItem {
        const item = new CTreeItem(label);
        this.children.push(item);
        return item;
    }
    addIndex(label: string, start: number, size: number): CTreeItem {
        if (!size) {
            return this.append(label);
        }
        const item = new CTreeItem(label);
        item.index = [start, size];
        this.children.push(item);
        return item;
    }
}

export class HexV {
    data: Uint8Array;
    index!: [number, number];
    constructor(data: Uint8Array) {
        this.data = data;
    }
}

export class OverviewSource {
    legends!: string[];
    labels!: number[];
    counts!: number[];
    valMap: any;
}


export abstract class MainProps {
    // client?: PCAPClient;
    items?: Frame[];
    // tcps?: TCPCol[];
    // arpGraph?: Grap;
    // overview?: OverviewSource;
    // dnsRecords?: DNSRecord[];
    abstract _frames(): Frame[];
    public getFrames():  Frame[]{
        if(this.items){
            return this.items;
        }
        this.items = this._frames();
        return this.items;
    }
}

export class IDNSRecord implements ColumnItem {
    constructor(public readonly record: DNSRecord) { }
    style?: string;
    no?: number;
    getIndex(): number {
        return 0;
    };
    getStyle(inx: number): string {
        return '';
    }
}

export class Statc {
    size: number = 0;
    count: number = 0;
    start!: number;
    end!: number;
    stc: Map<string, number> = new Map();
    public addLable(label: string, packet: FrameInfo): void {
      const count = this.stc.get(label) || 0;
      const size = packet.len || 0;
      this.stc.set(label, count + size);
    }
    public static create(ts: number, per: number) {
      const item = new Statc();
      item.start = ts;
      item.end = ts + per;
      return item;
    }
  }

  const parseTime = (time: number): string => {
    const date = new Date(time);
    const [hour, minutes, seconds, ms] = [
      date.getHours(),
      date.getMinutes(),
      date.getSeconds(),
      date.getMilliseconds()
    ];
    return `${minutes}:${seconds} ${ms}`;
  }
  
export interface IContextInfo {
    frame: number,
    conversation: number,
    dns: number,
}

export interface IOverviewData {
    legends: any[],
    labels: any[],
    datas: any[],
}

export class Pagination {
    page: number;
    size: number;
    filter: string;
}
export interface IFrameInfo {
    no: number;
    time: number;
    source: string;
    dest: string;
    protocol: string;
    // iRtt: number;
    len: number;
    // style: string;
    info: string;
    status: string;
}

export interface IFrameResult {
    data: IFrameInfo[],
    total: number,
    page: number;
    size: number;
}