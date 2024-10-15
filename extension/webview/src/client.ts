import { load, WContext, FrameInfo, Field } from 'rshark';
import { pick } from 'lodash';
import { ComLog, ComMessage, IContextInfo, OverviewSource, IOverviewData, IFrameInfo, Pagination, IResult, CField, HexV } from './common';


const convert = (frames: FrameInfo[]): any => {
  const scale = 24;
  const start = frames[0].time;
  const end = frames[frames.length - 1].time;
  const duration = end - start;
  const per = Math.floor(duration / scale);
  const result: Statc[] = [];
  let cur = start;
  let limit = cur + per;
  let rs = Statc.create(start, per);
  const ps = new Set<string>();
  const getArray = (num: number): Statc => {
    if (num < limit) {
      return rs;
    }
    result.push(rs);
    rs = Statc.create(limit, per);
    limit = limit + per;
    return getArray(num);
  }
  let _total = 0;
  for (const item of frames) {
    const origin = item.len;
    _total += item.len;
    const it = getArray(item.time);
    it.size += origin;
    it.count += 1;
    const pname = item.protocol?.toLowerCase() || '';
    it.addLable(pname, item);
    ps.add(pname);
  }

  const categories = ['total'];
  const map: any = {
    total: []
  };
  ps.forEach((c) => {
    categories.push(c);
    map[c] = [];
  });
  const labels = [];
  const countlist = [];
  for (const rs of result) {
    const { size, count, stc, start } = rs;
    labels.push(start);
    countlist.push(count);
    map.total.push(size)
    ps.forEach((c) => {
      map[c].push(stc.get(c) || 0);
    });
  }
  const overview = new OverviewSource();
  overview.legends = categories;
  overview.labels = labels;
  overview.counts = countlist;
  overview.valMap = map;
  return overview;
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
class FieldImlp implements CField {
  // start: number;
  // size: number;
  summary: string;
  children?: CField[];
  constructor(f: Field) {
    // this.start = f.start;
    // this.size = f.size;
    this.summary = f.summary;
    this.children = (f.children || []).map((_f) => new FieldImlp(_f));
  }
  // data(): Uint8Array {
  //   return this.f.data;
  // }
};
// const convertField = (f: Field) => {
//   const start = f.start;
//   const size = f.size;
//   const summary = f.summary;
//   const children = f.children;
// }
export abstract class PCAPClient {
  level: string = 'trace';
  ready: boolean = false;
  data!: Uint8Array;
  ctx!: WContext;
  cost: number;
  _cache: any = {};
  initData(data: Uint8Array) {
    this.data = data;
  }
  abstract printLog(log: ComLog): void;
  abstract emitMessage(msg: ComMessage<any>): void;
  _info(): void {
    if (this.ready && this.ctx) {
      const _info = this.getInfo();
      this.emitMessage(new ComMessage('_info', _info));
    }
  }
  init(): void {
    if (!this.ctx && this.data) {
      try {
        const _start = Date.now();
        this.ctx = load(this.data as Uint8Array);
        this.cost = Date.now() - _start;
        this._info();
      } catch (e) {
        this.emitMessage(new ComMessage('_error', "failed to open file"));
      }
    }
  }
  getInfo(): IContextInfo {
    if (!this._cache.info) {
      this._cache.info = JSON.parse(this.ctx.info());
      this._cache.info.cost = this.cost;
    }
    return this._cache.info;
  }
  _protocols(): void {
    if (this.ready && this.ctx) {
      if (!this._cache.protocols) {
        const data = this.ctx.get_aval_protocals();
        this._cache.protocols = (data || []).map(f => ({ name: f, code: f }));
      }
      const options = this._cache.protocols
      this.emitMessage(new ComMessage('_protocols', options));
    }
  }
  _overview(): void {
    if (this.ready && this.ctx) {
      this.emitMessage(new ComMessage('_frame_statistic', JSON.parse(this.ctx.statistic_frames())));
      this.emitMessage(new ComMessage('_http_statistic', JSON.parse(this.ctx.statistic())));
    }
  }
  getHex(index: number, key: string): Field {
    const inx = key.split('_');
    if (!inx.length) {
      return null;
    }
    const fields = this.ctx.get_fields(index);
    let val: Field = fields[parseInt(inx[0])];
    for (let i = 1; i < inx.length; i += 1) {
      val = val.children[parseInt(inx[i])];
      if (!val) {
        return null;
      }
    }
    return val;
  }
  _hex(index: number, key: string): void {
    const field = this.getHex(index, key);
    if (field && field.data) {
      const data = new HexV(field.data);
      data.index = [field.start, field.size];
      this.emitMessage(new ComMessage('_hex', data));
    }
  }
  getFrames(pag: Pagination): IResult {
    const { page, size } = pag;
    const start = (page - 1) * size;
    const rs = this.ctx.select_frame_items(start, size, pag.filter || []);
    const data = rs.items().map((f, inx) => {
      const emb = pick(f, 'index', 'time', 'status', 'len', 'info', 'irtt', 'protocol', 'dest', 'source');
      return emb;
    });
    return { items: data, page, size, total: rs.total };
  }
  _frame(pag: Pagination): void {
    if (this.ready && this.ctx) {
      const data = this.getFrames(pag);
      this.emitMessage(new ComMessage('_frame', data));
    }
  }
  _conversation(): void {
    if (this.ready && this.ctx) {
      if (!this._cache.conversation) {
        this._cache.conversation = this.ctx.select_conversation_items()
      }
      this.emitMessage(new ComMessage('_conversation', this._cache.conversation));
    }
  }
  _dns(): void {
    if (this.ready && this.ctx) {
      if (!this._cache.dns) {
        this._cache.dns = this.ctx.select_dns_items()
      }
      this.emitMessage(new ComMessage('_dns', this._cache.dns));
    }
  }
  getFields(index: number): CField[] {
    return this.ctx.get_fields(index).map((_f) => new FieldImlp(_f))
  }
  _fields(index: number): void {
    this.emitMessage(new ComMessage('_fields', this.getFields(index)));
  }
  _http(): void {
    if (!this._cache.http) {
      this._cache.http = this.ctx.select_http_items(0, 1000, []);
    }
    this.emitMessage(new ComMessage('_http', this._cache.http));
  }
  http_content(body) {
    const content = this.ctx.select_http_content(body[0], BigInt(body[1]));
    this.emitMessage(new ComMessage('_http-content', content));
  }
  _tls(): void {
    if (this.ready && this.ctx) {
      if (!this._cache.tls) {
        this._cache.tls = this.ctx.select_tls_items();
      }
      this.emitMessage(new ComMessage('_tls', this._cache.tls));
    }
  }
  handle(msg: ComMessage<any>) {
    if (!msg) return;
    const { type, body } = msg
    if (!type) return;
    try {
      switch (type) {
        case 'ready':
          this.ready = true;
          try {
            this.init();
          } catch (e) {
            console.error(e);
            this.printLog(new ComLog('error', 'failed to open file'));
          }
          break;
        case 'log':
          this.printLog(body as ComLog);
          break;
        case 'webpackWarnings':
          break;
        case 'info':
          this._info();
          break;
        case 'frame':
          this._frame(body);
          break;
        case 'overview':
          this._overview();
          break;
        case 'protocols':
          this._protocols();
          break;
        case 'dns':
          this._dns();
          break;
        case 'fields':
          this._fields(body);
          break;
        case 'http':
          this._http();
          break;
        case 'tls':
          this._tls();
          break;
        case 'hex':
          this._hex(body.index, body.key);
          break;
        case 'conversation':
          this._conversation();
          break;
        case 'http-content':
          this.http_content(body);
          break;
        default:
          console.log('unknown type', msg.type);
      }
    } catch (e) {
      console.error(e);
    }
  }
}