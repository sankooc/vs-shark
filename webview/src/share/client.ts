import { load, WContext, Conf, FrameResult, HttpDetail } from "rshark";
import { ComLog, ComMessage, ComRequest, ComType, IHttpDetail, PcapFile, StatRequest, VRange } from "./common";
import mitt, { Emitter } from "mitt";

export const BATCH_SIZE = 1024 * 1024 * 1;

function frameSelectConvert(frameSelect: FrameResult): any {
  // const rs = {};
  const str = frameSelect.list();
  const datasource = [];
  const len = frameSelect.data_count();
  for (let i = 0; i < len; i += 1) {
    const data = frameSelect.data(i);
    if (data) {
      const _range = frameSelect.range(i);
      let range = null;
      if (_range) {
        range = new VRange(_range.start, _range.end);
      }
      datasource.push({ data, range })
    }
  }
  return { str, datasource };
}

export abstract class PCAPClient {
  private emitter: Emitter<any> = mitt();
  private highPirityQueue: ComMessage<any>[] = [];
  private lowPirityQueue: ComMessage<any>[] = [];
  level: string = "trace";
  isPendding: boolean = false;
  ready: boolean = false;
  ctx?: WContext;
  info?: PcapFile;
  resourceId?: string;
  init(): void {
    if (!this.ctx) {
      this.resourceId = Date.now() + '';
      this.ctx = load(Conf.new(this.resourceId, false, BATCH_SIZE));
    }
  }
  private async update(data: Uint8Array): Promise<string> {
    if (!this.ctx) {
      this.init();
    }
    if (this.ctx) {
      try {
        const rs = await this.ctx.update(data);
        if(rs) {
          this.emitMessage(ComMessage.new(ComType.PRGRESS_STATUS, rs));
          return rs;
        }
      } catch (e) {
        console.error(e);
        this.emitMessage(new ComMessage(ComType.error, "failed to open file"));
      }
    }
    return "";
  }
  abstract doReady(): void;
  // abstract appendData(data: Uint8Array): void;
  abstract printLog(log: ComLog): void;
  abstract emitMessage(msg: ComMessage<any>): void;
  // abstract pickData(start: number, end: number): Promise<Uint8Array>;

  private touchFile(fileInfo: PcapFile): void {
    this.info = fileInfo;
    this.emitMessage(ComMessage.new(ComType.FILEINFO, fileInfo));
  }
  private list(
    requestId: string,
    catelog: string,
    param: any,
  ): void {
    const { start, size } = param;
    if (this.ctx) {
      try {
        let rs;
        switch (catelog) {
          case "frame":
            rs = this.ctx.list("frame", start, size);
            this.emitMessage(ComMessage.new(ComType.FRAMES, rs, requestId));
            return;
          case "conversation":
            rs = this.ctx.list_conversations(start, size, param.ip || '');
            this.emitMessage(ComMessage.new(ComType.CONVERSATIONS, rs, requestId));
            return;
          case "connection": {
              rs = this.ctx.list_connections(param.conversionIndex, start, size);
              this.emitMessage(ComMessage.new(ComType.CONNECTIONS, rs, requestId));
              return;
            }
          case "http_connection": {
              rs = this.ctx.list_http(start, size, param.host || '', '');
              this.emitMessage(ComMessage.new(ComType.HTTP_CONNECTIONS, rs, requestId));
              return;
            }
          case "udp": {
              rs = this.ctx.list_udp(start, size, param.ip || '');
              this.emitMessage(ComMessage.new(ComType.UDP_CONNECTIONS, rs, requestId));
              return;
            }
          case "dns": {
              rs = this.ctx.list_dns(start, size);
              this.emitMessage(ComMessage.new(ComType.DNS_CONNECTIONS, rs, requestId));
              return;
            }
          case "tls": {
            const rs = this.ctx.list_tls(start, size);
            this.emitMessage(ComMessage.new(ComType.TLS_CONNECTIONS, rs, requestId));
            return;
          }
          default:
            return;
        }
      } catch (e) {
        console.error(e);
        this.emitMessage(new ComMessage(ComType.error, "failed"));
      }
    }
    this.emitMessage(
      ComMessage.new(ComType.error, "failed", requestId),
    );
  }
  private async select(requestId: string, catelog: string, index: number): Promise<void> {
    if (this.ctx) {
      try {
        // let rs;
        switch (catelog) {
          case "frame": {
            // const range = this.ctx!.frame_range(index);
            // const data = await this.pickData(range.data.start, range.data.end);
            // const frameResult = this.ctx.select_frame(index, data);
            const frameResult = this.ctx.select_frame(index);
            const rs = frameSelectConvert(frameResult);
            // rs.data = frameResult.source()
            // if (range.compact()) {
            //   rs.data = data;
            // } else {
            //   const _start = range.frame.start - range.data.start;
            //   const _end = range.frame.end - range.data.start;
            //   rs.data = data.slice(_start, _end);
            // }
            // rs.start = range.frame.start;
            // rs.end = range.frame.end;
            // rs.liststr = frameResult.list();
            // if (frameResult.extra()?.length > 0) {
            //   rs.extra = frameResult.extra();
            // }
            this.emitMessage(
              ComMessage.new(ComType.FRAMES_SELECT, rs, requestId),
            );
            return;
          }
          default:
            return;
        }
      } catch (e) {
        console.error(e);
        this.emitMessage(new ComMessage(ComType.error, "failed"));
      }
    }
    this.emitMessage(
      ComMessage.new(ComType.error, "failed", requestId),
    );
  }
  private async http_detail2(index: number): Promise<IHttpDetail[]> {
    const rs = this.ctx?.http_detail(index) || [];
    const convert = (data: HttpDetail): IHttpDetail => {
      const headers = data.headers();
      const raw = data.raw_content();
      const plaintext = data.get_text_content();
      const content_type = data.content_type();
      return { headers, raw, plaintext, content_type }
    }
    if (rs) {
      return rs.map(convert)
    }
    return [];
  }
  private async stat(field: string): Promise<string> {
    if (this.ctx) {
      return this.ctx.stat(field) || '[]';
    }
    return "[]";
  }

  private async process(): Promise<void> {
    if (this.isPendding) {
      return;
    }
    this.isPendding = true;
    while (this.highPirityQueue.length > 0) {
      const msg = this.highPirityQueue.shift();
      await this._process(msg!);
      this.emitter.emit(msg!.id, {});
    }
    while (this.lowPirityQueue.length > 0) {
      const msg = this.lowPirityQueue.shift();
      await this._process(msg!);
      this.emitter.emit(msg!.id, {});
    }
    this.isPendding = false;
  }

  private async _process(msg: ComMessage<any>) {
    const { type, body, id } = msg;
    if (!type) {
      return;
    }
    try {
      switch (type) {
        case ComType.CLIENT_REDAY:
          if (!this.ready) {
            this.ready = true;
            try {
              this.doReady();
            } catch (e) {
              console.error(e);
              this.printLog(new ComLog("error", "failed to open file"));
            }
          }
          break;
        case ComType.TOUCH_FILE:
          this.touchFile(body as PcapFile);
          break;
        // case ComType.FRAME_SCOPE:
        //   const index = body as number;
        //   const range = this.ctx!.frame_range(index)
        //   this.emitMessage(ComMessage.new(ComType.FRAME_SCOPE_RES, { start: range.start, end: range.end }));
        //   break;
        case ComType.PROCESS_DATA:
          {
            const data = body.data as Uint8Array;
            await this.update(data);
          }
          // this.emitMessage(ComMessage.new(ComType.PRGRESS_STATUS, rs));
          break;
        case ComType.log:
          this.printLog(body as ComLog);
          break;
        case ComType.REQUEST: {
          const req: ComRequest = body;
          const { catelog, type, param } = req;
          switch (type) {
            case "list":
              this.list(id, catelog, param);
              break;
            case "select":
              this.select(id, catelog, param.index);
              break;
            // case "conversation":
            //   this.conversations(id, param.start, param.size);
            //   break;
            // case "scope":
            //   this.scope(id, catelog, param.index);
            //   break;
          }
          break;
        }
        case ComType.HTTP_DETAIL_REQ: {
          const index = body.index;
          if (index >= 0) {
            const rs = await this.http_detail2(index);
            this.emitMessage(
              ComMessage.new(ComType.HTTP_DETAIL_RES, rs, id),
            );
          }
          break;
        }
        // case ComType.TLS_REQ: {
        //   const result = this.tls_list() || '[]';
        //   this.emitMessage(
        //     ComMessage.new(ComType.TLS_RES, result, id),
        //   );
        //   break;
        // }
        case ComType.STAT_REQ: {
          const req: StatRequest = body;
          const rs = await this.stat(req.field);
          this.emitMessage(
            ComMessage.new(ComType.STAT_RES, rs, id),
          );
          break;
        }
        default:
      }
    } catch (e) {
      console.error(e);
    }
  }

  async handle(msg: ComMessage<any>): Promise<void> {
    if (!msg) {
      return;
    }
    const { type, id } = msg;
    if (type == ComType.PROCESS_DATA) {
      this.lowPirityQueue.push(msg);
    } else {
      this.highPirityQueue.push(msg);
    }
    return new Promise((resolve) => {
      this.emitter.on(id, () => {
        this.emitter.off(id);
        resolve();
      });
      this.process();
    });
  }
}
