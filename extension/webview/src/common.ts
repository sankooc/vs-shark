import { ITLSHS } from "./gen";

export function deserialize<T>(content: string): T {
    return JSON.parse(content)
}

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
export interface ICase {
    name: string;
    value: number;
}
export interface IStatistic {
    http_method: ICase[];
    http_status: ICase[];
    http_type: ICase[];
    ip: ICase[];
    ip_type: ICase[];
}
export interface IContextInfo {
    file_type: string;
    start_time: number;
    end_time: number;
    frame_count: number;
    http_count: number;
    dns_count: number;
    tcp_count: number;
    tls_count: number;
    cost: number,
    // conversation: number,
    // dns: number,
    // http: number,
    // statistic: IStatistic,
}

export interface ILineData {
    name: string,
    data: number[],
}
export interface ILines {
    x: string[],
    y: string[],
    data: ILineData[],
}

export interface IOverviewData {
    legends: any[],
    labels: any[],
    datas: any[],
}

export class Pagination {
    page: number;
    size: number;
    filter: string[];
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

export interface IResult {
    items: any[],
    total: number,
    page: number;
    size: number;
}

// export interface IEndpoint {
//     ip: string,
//     port: number,
//     host: string,
//     count: number,
//     throughput: number,
//     retransmission: number,
//     invalid: number,
// }

// export interface IConversation {
//     source: IEndpoint,
//     target: IEndpoint,
//     // source_ip: string,
//     // source_port: number,
//     // source_host: string,
//     // target_ip: string,
//     // target_port: number,
//     // target_host: string,
//     // count: number,
//     // throughput: number,
// }

// export interface IDNSRecord {
//     name: string,
//     _type: string,
//     content: string,
//     class: string,
//     ttl: number,
// }

export interface CField {
    summary: string,
    children?: CField[],
}

export interface ITLSHandshake extends ITLSHS {
    index?: number,
    status?: string,
}