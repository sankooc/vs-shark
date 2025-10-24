export interface IProgressStatus {
    total: number;
    cursor: number;
    count: number;
    left: number;
}
export interface IListResult<T> {
    items: T[];
    total: number;
    start: number;
}
export interface IFrameInfo {
    index: number;
    time: number;
    source: string;
    dest: string;
    protocol: string;
    len: number;
    irtt: number;
    info: string;
    status: string;
}
export interface IField {
    source: number;
    start: number;
    size: number;
    summary: string;
    children?: IField[];
}
export interface IVConversation {
    key: number;
    sender: string;
    receiver: string;
    sender_packets: number;
    receiver_packets: number;
    sender_bytes: number;
    receiver_bytes: number;
    connects: number;
}
export interface ITCPStatistic {
    count: number;
    throughput: number;
    clean_throughput: number;
    retransmission: number;
    invalid: number;
}
export interface IVConnection {
    primary: IVEndpoint;
    second: IVEndpoint;
    protocol: string;
}
export interface IVEndpoint {
    host: string;
    port: number;
    statistic: ITCPStatistic;
}
export interface IVHttpConnection {
    request?: string;
    response?: string;
    rt: string;
    content_type: string;
    hostname: string,
    length: number;
    request_headers: [number, number][];
    response_headers: [number, number][];
    request_body: [number, number][];
    response_body: [number, number][];
}

export interface ICounterItem {
    key: string;
    count: number;
}

export interface ILineData {
    x_axis: number[];
    y_axis: string[];
    data: number[][];
}

export interface IUDPConversation {
    index: number;
    sender: string;
    receiver: string;
    sender_port: number;
    receiver_port: number;
    packets: number;
    bytes: number;
    records: number[][];
}