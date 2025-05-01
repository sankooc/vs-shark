export function deserialize<T>(content: string): T {
  return JSON.parse(content);
}

export enum ComType {
  SERVER_REDAY = "ready",
  CLIENT_REDAY = "_ready",
  REQUEST = "request",
  RESPONSE = "response",
  log = "log",
  error = "error",
}

export class ComMessage<T> {
  type: ComType;
  body: T;
  id!: string;
  constructor(type: ComType, body: T) {
    this.type = type;
    this.body = body;
    this.id = Date.now().toString();
  }
  static new(type: ComType, body: any): ComMessage<any> {
    return new ComMessage(type, body);
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
  cost: number;
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

export interface IOverviewData {
  legends: any[];
  labels: any[];
  datas: any[];
}

export class Pagination {
  page?: number;
  size?: number;
  filter?: string;
}

export interface IResult {
  items: any[];
  total: number;
  page: number;
  size: number;
}

export interface CField {
  summary: string;
  children?: CField[];
}
