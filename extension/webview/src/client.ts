import init, { load, WContext, FrameInfo } from 'rshark';
import { ComLog, ComMessage, IContextInfo, OverviewSource, Statc, IOverviewData, IFrameInfo, Pagination, IFrameResult } from './common';


function convert(frames: FrameInfo[]): any {
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

export abstract class PCAPClient {
  public static readonly STATUS = init();
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
      this.ctx = load(this.data as Uint8Array);
    }
    this._info();
  }
  getInfo(): IContextInfo {
    const frame = this.ctx.get_frames().length;
    const conversation = this.ctx.get_conversations_count();
    const dns = this.ctx.get_dns_count();
    return { frame, conversation, dns }
  }
  getOverview(): IOverviewData {
    const { legends, labels, valMap } = convert(this.ctx.get_frames());
    const keys = Object.keys(valMap);
    const datas = keys.map((key) => {
      const data = valMap[key];
      const rs: any = {
        name: key,
        yAxisIndex: 1,
        smooth: true,
        type: 'line',
        data
      };
      if (key === 'total') {
        rs.areaStyle = {};
      }
      return rs;
    });
    return { legends, labels, datas };
  }
  _overview(): void {
    if (this.ready && this.ctx) {
      const data = this.getOverview();
      this.emitMessage(new ComMessage('_overview', data));
    }
  }
  getFrames(pag: Pagination): IFrameResult {
    const {page, size} = pag;
    const start = (page - 0) * size;
    const end = start + size;
    const total = this.ctx.get_frame_count();

    const items = this.ctx.select_frames(start, end);
    const data = items.map((f, inx) => {
      const data = { ...f, no: inx + 1 };
      return data;
    });
    return { data, page, size, total };
  }
  _frame(pag: Pagination): void {
    if (this.ready && this.ctx) {
      const data = this.getFrames(pag);
      this.emitMessage(new ComMessage('_frame', data));
    }
  }
  handle(msg: ComMessage<any>) {
    if (!msg) return;
    const { type, body } = msg
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
          this._info();
          break;
        case 'hex-data':
          // this.renderHexView(body as HexV); 
          break;
        default:
          console.log('unknown type', msg.type);
      }
    } catch (e) {
      console.error(e);
    }
  }
}