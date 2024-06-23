

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
                default:
                    console.error('unknown type', msg.type);
            }
        }catch(e){
            console.error(e);
        }
    }
    
}

export class Frame {
    no!: number;
    time: string = '';
    source: string = 'n/a';
    dest: string = 'n/a';
    protocol!: string;
    iRtt: number = 0;
    len: number = 0;
    info!: string;
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
}

export class HexV {
    data: Uint8Array;
    index!: [number, number];
    constructor(data: Uint8Array){
        this.data = data;
    }
}