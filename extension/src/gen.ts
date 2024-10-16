export interface IConnect<T> {
    index: number;
    source: string;
    target: string;
    list: T[];
}
export interface IHttpMessage {
    ts: number;
    head: string;
    headers: string[];
    method: string;
    _type?: string;
    path: string;
    len: number;
}
export interface IStatisticV {
    http_method: ICase[];
    http_status: ICase[];
    http_type: ICase[];
    ip: ICase[];
    ip_type: ICase[];
}
export interface ICase {
    name: string;
    value: number;
}
export interface ILineData {
    name: string;
    data: number[];
}
export interface ILines {
    x: string[];
    y: string[];
    data: ILineData[];
}
export interface IPCAPInfo {
    file_type: string;
    start_time: number;
    end_time: number;
    frame_count: number;
    http_count: number;
    dns_count: number;
    tcp_count: number;
    tls_count: number;
    cost: number;
}
export interface ITLSHS {
    source: string;
    target: string;
    server_name: string[];
    support_version: string[];
    support_cipher: string[];
    support_negotiation: string[];
    used_version: string;
    used_cipher: string;
    used_negotiation: string[];
}
export interface IDNSRecord {
    name: string;
    _type: string;
    proto: string;
    class: string;
    content: string;
    ttl: number;
}
export interface IWEndpoint {
    ip: string;
    port: number;
    host: string;
    count: number;
    throughput: number;
    retransmission: number;
    invalid: number;
}
export interface ITCPConversation {
    source: IWEndpoint;
    target: IWEndpoint;
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
export interface IListResult<T> {
    items: T[];
    total: number;
    start: number;
}
export interface IField {
    summary: string;
    children: IField[];
}
