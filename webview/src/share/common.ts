export function deserialize<T>(content: string): T {
  return JSON.parse(content);
}

export enum ComType {
  SERVER_REDAY = "ready",
  CLIENT_REDAY = "_ready",
  TOUCH_FILE = "file_touch",
  PROCESS_DATA = "process_data",
  PRGRESS_STATUS = "progress",
  FILE_CLOSE = "file_close",
  REQUEST = "request",
  RESPONSE = "response",
  FRAMES = "frames",
  FRAMES_SELECT = "frames_select",
  DATA = "DATA",
  FILEINFO = "fileinfo",
  log = "log",
  error = "error",
}

export interface ComRequest {
  catelog: string;
  type: string;
  param: any;
}

// export interface Pagination {
//   start: number
//   size?: number
// }

export class ComMessage<T> {
  type: ComType;
  body: T;
  id: string;
  constructor(type: ComType, body: T, id?: string) {
    const _id = id ? id : Date.now().toString();
    this.type = type;
    this.body = body;
    this.id = _id;
  }
  static new(type: ComType, body: any, id?: string): ComMessage<any> {
    return new ComMessage(type, body, id);
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

export interface PcapFile {
  name: string;
  size: number;
  start: number;
  state?: number;
}


export interface IField {
  summary: string;
  start?: number;
  size?: number;
  children: IField[] | null;
}


export interface Cursor {
  scope?: {
    start: number;
    size: number;
  }
  selected?: {
    start: number;
    size: number;
  }
}

export interface DataResponse {
  data: Uint8Array;
  start?: number;
  size?: number;
}

export class HexV {
  data: Uint8Array;
  index!: [number, number];
  constructor(data: Uint8Array) {
    this.data = data;
  }
}

// export class OverviewSource {
//   legends!: string[];
//   labels!: number[];
//   counts!: number[];
//   valMap: any;
// }
// export interface ICase {
//   name: string;
//   value: number;
// }
// export interface IStatistic {
//   http_method: ICase[];
//   http_status: ICase[];
//   http_type: ICase[];
//   ip: ICase[];
//   ip_type: ICase[];
// }
// export interface IContextInfo {
//   file_type: string;
//   start_time: number;
//   end_time: number;
//   frame_count: number;
//   http_count: number;
//   dns_count: number;
//   tcp_count: number;
//   tls_count: number;
//   cost: number;
// }

// export interface ILineData {
//   name: string;
//   data: number[];
// }
// export interface ILines {
//   x: string[];
//   y: string[];
//   data: ILineData[];
// }

// export interface IOverviewData {
//   legends: any[];
//   labels: any[];
//   datas: any[];
// }

export class Pagination {
  start?: number;
  size?: number;
  filter?: string;
}

export interface IResult {
  items: any[];
  total: number;
  page: number;
  size: number;
}

// export interface CField {
//   summary: string;
//   children?: CField[];
// }
