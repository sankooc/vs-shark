import { load, WContext, FrameInfo, Field } from 'rshark';
import { pick } from 'lodash';
import { ComLog, ComMessage, IContextInfo, OverviewSource, IOverviewData, IFrameInfo, Pagination, IResult, IConversation, IDNSRecord, CField, HexV, IHttp } from './common';

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
        this.ctx = load(this.data as Uint8Array);
        this._info();
      }catch(e){
        this.emitMessage(new ComMessage('_error', "failed to open file"));
      }
    }
  }
  getInfo(): IContextInfo {
    const rs = JSON.parse(this.ctx.info());
    return rs;
    // const statistic = this.ctx.statistic();
    // return { frame, conversation, dns, http, statistic:JSON.parse(statistic) }
  }
  _protocols(): void {
    if (this.ready && this.ctx) {
      const data = this.ctx.get_aval_protocals();
      const options = (data || []).map(f => ({name:f, code: f}));
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
      // val = val.[inx[i]];
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
    const rs = this.ctx.select_frames(start, size, pag.filter || []);
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
  getConversations(): IConversation[] {
    const _data = this.ctx.get_conversations();
    return _data.map((f) => {
      const source = f.source;
      const target = f.target;
      return {
        source: pick(source, 'ip', 'port', 'host', 'count', 'throughput', 'retransmission', 'invalid'),
        target: pick(target, 'ip', 'port', 'host', 'count', 'throughput', 'retransmission', 'invalid'),
      }
    })
    // return _data.map(f => pick(f, 'source_ip', 'source_host','source_port', 'target_ip', 'target_host','target_port', 'count', 'throughput'));
  }
  _conversation(): void {
    if (this.ready && this.ctx) {
      const data = this.getConversations()
      this.emitMessage(new ComMessage('_conversation', data));
    }
  }
  getDNS(): IDNSRecord[] {
    return this.ctx.get_dns_record();
  }
  _dns(): void {
    if (this.ready && this.ctx) {
      const data = this.getDNS().map(f => pick(f, 'name', '_type', 'content', 'class', 'ttl'));
      this.emitMessage(new ComMessage('_dns', data));
    }
  }
  getFields(index: number): CField[] {
    return this.ctx.get_fields(index).map((_f) => new FieldImlp(_f))
  }
  _fields(index: number): void {
    this.emitMessage(new ComMessage('_fields', this.getFields(index)));
  }
  http(): IHttp [] {
    const rs = this.ctx.select_http(0, 1000, []).map(f => {
      const _rs = pick(f, 'req', 'res', 'status', 'method');
      return {
        status: _rs.status,
        method: _rs.method,
        ttr: Number(f.ttr),
        req: pick(_rs.req, 'host', 'port', 'head', 'header', 'content_len', 'content'),
        res: pick(_rs.res, 'host', 'port', 'head', 'header', 'content_len', 'content'),
      }
    });

    return rs;
  }
  _http(): void {
    this.emitMessage(new ComMessage('_http', this.http()));
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
        case 'hex':
          this._hex(body.index, body.key);
          break;
        case 'conversation':
          this._conversation();
          break;
        default:
          console.log('unknown type', msg.type);
      }
    } catch (e) {
      console.error(e);
    }
  }
}