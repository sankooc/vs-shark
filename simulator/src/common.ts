
import { DNSRecord } from 'protocols/built/src/common';

export class ComMessage<T> {
    type: string;
    body: T;
    id!: string;
    constructor(type: string, body: T){
        this.type = type;
        this.body = body;
    }
}

export class ComLog {
    level: string;
    msg: any;
    constructor(level: string, msg: any){
        this.level = level;
        this.msg = msg;
    }
}

export enum Panel {
    MAIN,
    TREE,
    DETAIL,
}


export abstract class PCAPClient {
    level: string = 'trace';
    data!: Uint8Array;
    initData(data: Uint8Array): void{
        this.data = data;
    };
    abstract emitMessage(panel: Panel, msg: ComMessage<any>): void;
    abstract printLog(log: ComLog): void;
    abstract selectFrame(no: number): void;

    abstract renderHexView(data: HexV): void;
    abstract init(): void;

    handle(msg: ComMessage<any>){
        if(!msg) return;
        const { type, body } = msg
        try {
            switch(type){
                case 'ready':
                    this.init();
                    break;
                case 'log':
                    this.printLog(body as ComLog);
                    break;
                case 'webpackWarnings':
                    break;
                case 'frame-select':
                    this.selectFrame(body.index as number);
                    break;
                case 'hex-data':
                    this.renderHexView(body as HexV);
                    break;
                default:
                    console.log('unknown type', msg.type);
            }
        }catch(e){
            console.error(e);
        }
    }
    
}

export interface ColumnItem {
    getIndex(): number;
    getStyle(inx: number): string;
}

export class TCPCol implements ColumnItem {
    no!: number;
    ep1!: string;
    ep2!: string;
    total!: number;
    tcp!: number;
    tcpUse!: number;
    count!: number;
    countUse!: number;
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
    constructor(name: string){
        this.name = name;
    }
}
export class GrapNode {
    private static index:number = 0;
    public static create(name: string, category: number): GrapNode{
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
    constructor(name: string){
        this.name = name;
    }
}
export class GrapLink {
    readonly source: number;
    readonly target: number;
    constructor(source: number, target: number){
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
    source: string = 'n/a';
    dest: string = 'n/a';
    protocol!: string;
    iRtt: number = 0;
    len: number = 0;
    style: string='';
    info!: string;
    getIndex(): number {
        return this.no;
    }
    getStyle(inx: number): string {
        if(this.no === inx){
            return 'active';
        }
        return this.style;
    }

}


export class CTreeItem {
    label: string;
    index?: [number, number];
    children: CTreeItem[] = [];
    constructor(label: string){
        this.label = label;
    }
    append(label:string): CTreeItem {
        const item = new CTreeItem(label);
        this.children.push(item);
        return item;
    }
    addIndex(label:string, start: number, size: number): CTreeItem{
        if(!size){
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
    constructor(data: Uint8Array){
        this.data = data;
    }
}

export class OverviewSource {
    legends!: string[];
    labels!: number[];
    counts!: number[];
    valMap: any;
}
export class MainProps {
    status!: string;
    items!: Frame[];
    tcps!: TCPCol[];
    arpGraph!: Grap;
    overview!: OverviewSource;
    dnsRecords!: DNSRecord[];
}