import { load, WContext, FrameInfo, Field } from 'rshark';
import { ComLog, ComMessage, IContextInfo, OverviewSource, IOverviewData, Pagination, IResult, CField, HexV } from './common';

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
    const stack = inx.map((f) => {return parseInt(f)});
    const ff = this.ctx.pick_field(index, new Uint16Array(stack));
    return ff;
  }
  _hex(index: number, key: string): void {
    const field = this.getHex(index, key);
    if (field && field.data) {
      const data = new HexV(field.data);
      data.index = [field.start, field.size];
      this.emitMessage(new ComMessage('_hex', data));
    }
  }
  getFrames(pag: Pagination): String {
    const { page, size } = pag;
    const start = (page - 1) * size;
    return this.ctx.select_frame_items(start, size, pag.filter || []);
    // const data = rs.items().map((f, inx) => {
    //   const emb = pick(f, 'index', 'time', 'status', 'len', 'info', 'irtt', 'protocol', 'dest', 'source');
    //   return emb;
    // });
    // return { items: data, page, size, total: rs.total };
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
  _pick_field(index: number, stack: number[]): void {

  }
  _fields(index: number): void {
    this.emitMessage(new ComMessage('_fields', this.ctx.get_fields(index)));
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