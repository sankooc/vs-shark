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
  FRAME_SCOPE = "frame_scope",
  FRAME_SCOPE_RES = "FRAME_SCOPE_RES",
  DATA = "DATA",
  FILEINFO = "fileinfo",
  log = "log",
  error = "error",
  CONVERSATIONS = "conversations",
  CONNECTIONS = "connections",
  HTTP_CONNECTIONS = "http_connections",
  HTTP_DETAIL_REQ = "http_detail_req",
  HTTP_DETAIL_RES = "http_detail_res",
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

let counter = 0;

const getMessageId = (type: string) => {
  const cc = counter++;
  return `${cc}_${type}_${Date.now()}`;
};

export class ComMessage<T> {
  type: ComType;
  body: T;
  id: string;
  constructor(type: ComType, body: T, id?: string) {
    const _id = id ? id : getMessageId(type);
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
  source?: number;
  children: IField[] | null;
}

export interface Cursor {
  scope: VRange;
  data?: Uint8Array;
  tab: string,
  selected?: {
    start: number;
    size: number;
  };
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

export class VRange {
  constructor(public start: number, public end: number) { }
  public size(): number {
    if (this.end > this.start) {
      this.end - this.start;
    }
    return 0;
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

export interface IResult<T> {
  items: T[];
  total: number;
  page: number;
  size: number;
}


export interface IFrameSelect {
  data: Uint8Array;
  start: number;
  end: number;
  fields: IField[];
  extra?: Uint8Array;
}

export interface HttpMessageWrap {
  headers: string[];
  mime: string;
  parsed_content?: string;
  raw?: Uint8Array
}

export interface MessageCompress{
  json: string,
  data: Uint8Array
}


export const compute = (page: number, size: number): Pagination => {
  if (page < 1) {
    return { start: 0, size: size };
  }
  const start = (page - 1) * size;
  return { start, size };
};


const UNITS = ["B", "KB", "MB", "GB", "TB", "PB"];

export const format_bytes_single_unit = (bytes: number): string => {
  if (bytes <= 0) {
    return '0';
  }
  let size = bytes;
  let low = 0;
  let unit_index = 0;

  while (size >= 1024 && unit_index < UNITS.length - 1) {
    low = size % 1024;
    size = Math.floor(size / 1024);
    unit_index += 1;
  }

  return `${size}.${low} ${UNITS[unit_index]}`;
};