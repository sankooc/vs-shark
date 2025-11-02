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
  UDP_CONNECTIONS = "UDP_CONNECTIONS",
  DNS_CONNECTIONS = "DNS_CONNECTIONS",
  HTTP_DETAIL_REQ = "http_detail_req",
  HTTP_DETAIL_RES = "http_detail_res",
  STAT_REQ = "STAT_REQ",
  STAT_RES = "STAT_RES",
  TLS_REQ = "TLS_REQ",
  TLS_RES = "TLS_RES"
  // HTTP_STATISTICS_REQ = "http_statistics_req",
  // HTTP_STATISTICS_RES = "http_statistics_res",
  // TLS_STATISTICS_REQ = "tls_statistics_req",
  // TLS_STATISTICS_RES = "tls_statistics_res",
}

export interface ComRequest {
  catelog: string;
  type: string;
  param: any;
}

export interface StatRequest {
  field: string
}

// export interface Pagination {
//   start: number
//   size?: number
// }

let counter = 0;

const getMessageId = (type: string) => {
  const cc = counter++;
  return `${cc}_${type}_${Date.now()}`
}

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
      return this.end - this.start;
    }
    return 0;
  }
}

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

export interface IDataSource {
  data: Uint8Array,
  range: VRange,
}

export interface IFrameSelect {
  fields: IField[];
  datasource: IDataSource[]
}

export interface HttpMessageWrap {
  headers: string[];
  mime: string;
  parsed_content?: string;
  raw?: Uint8Array
}

export interface MessageCompress {
  json: string,
  data: Uint8Array
}

export interface ITLSInfo {
  hostname: string,
  alpn: string[],
  count: number,
}
export interface IHttpDetail {
  headers: string[],
  raw?: Uint8Array,
  plaintext?: string,
  content_type?: string,
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
}


const timeunits = [
  { name: 'h', value: 60 * 60 * 1000 * 1000 * 1000 },
  { name: 'm', value: 60 * 1000 * 1000 * 1000 },
  { name: 's', value: 1000 * 1000 * 1000 },
  { name: 'ms', value: 1000 * 1000 },
  { name: 'μs', value: 1000 },
  { name: 'ns', value: 1 }
];
export const formatMicroseconds = (sample: number, _time: number): string => {
  let time = _time
  if (typeof time !== 'number' || time < 0 || !isFinite(time)) {
    return '0';
  }
  if (time == 0) {
    return '0';
  }
  const len = (sample + '').length;
  const lft = 19 - len;
  if (lft >= 0) {
    time = time * Math.pow(10, lft);
  }

  for (const unit of timeunits) {
    if (time >= unit.value) {
      let value = time / unit.value;
      if (value >= 1000 && unit.name !== 'μs') {
        continue;
      }
      value = Math.round(value * 10) / 10;
      if (value % 1 === 0) {
        value = Math.floor(value);
      }
      return `${value}${unit.name}`;
    }
  }

  return `${time}μs`;
}
