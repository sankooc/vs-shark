import { load, WContext, Conf } from "rshark";
import { ComLog, ComMessage, ComRequest, ComType, MessageCompress, PcapFile } from "./common";
import mitt, { Emitter } from "mitt";
import { IVHttpConnection } from "./gen";

export const BATCH_SIZE = 1024 * 1024 * 1;

function concatLargeUint8Arrays(arrays: Uint8Array[]): Uint8Array {
  const totalLength = arrays.reduce((acc, arr) => acc + arr.length, 0);
  const buffer = new ArrayBuffer(totalLength);
  const result = new Uint8Array(buffer);
  let offset = 0;
  for (let arr of arrays) {
      result.set(arr, offset);
      offset += arr.length;
  }
  return result;
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
  init(): void {
    if (!this.ctx) {
      this.ctx = load(Conf.new(false, BATCH_SIZE));
    }
  }
  private async update(data: Uint8Array): Promise<string> {
    if (!this.ctx) {
      this.init();
    }
    if (this.ctx) {
      try {
        const rs = await this.ctx.update(data);
        this.emitMessage(ComMessage.new(ComType.PRGRESS_STATUS, rs));
        return rs;
      } catch (e) {
        console.error(e);
        this.emitMessage(new ComMessage(ComType.error, "failed to open file"));
      }
    }
    return "";
  }
  abstract appendData(data: Uint8Array): void;
  abstract printLog(log: ComLog): void;
  abstract emitMessage(msg: ComMessage<any>): void;
  abstract pickData(start: number, end: number): Promise<Uint8Array>;
  // private async frameData(index: number): Promise<Uint8Array> {
  //   const range = this.ctx!.frame_range(index);
  //   return this.pickData(range.data.start, range.data.end);
  // }

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
            rs = this.ctx.list_conversations(start, size);
            this.emitMessage(ComMessage.new(ComType.CONVERSATIONS, rs, requestId));
            return;
          case "connection":
            rs = this.ctx.list_connections(param.conversionIndex, start, size);
            this.emitMessage(ComMessage.new(ComType.CONNECTIONS, rs, requestId));
            return;
          case "http_connection":
            rs = this.ctx.list_http(start, size);
            this.emitMessage(ComMessage.new(ComType.HTTP_CONNECTIONS, rs, requestId));
            return;
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
          case "frame":
            const range = this.ctx!.frame_range(index);
            const data = await this.pickData(range.data.start, range.data.end);
            const frameResult = this.ctx.select_frame(index, data);
            const rs: any = {};
            if (range.compact()) {
              rs.data = data;
            } else {
              const _start = range.frame.start - range.data.start;
              const _end = range.frame.end - range.data.start;
              rs.data = data.slice(_start, _end);
            }
            rs.start = range.frame.start;
            rs.end = range.frame.end;
            rs.liststr = frameResult.list();
            if (frameResult.extra()?.length > 0) {
              rs.extra = frameResult.extra();
            }
            this.emitMessage(
              ComMessage.new(ComType.FRAMES_SELECT, rs, requestId),
            );
            return;
          default:
            return;
        }
      } catch (e) {
        this.emitMessage(new ComMessage(ComType.error, "failed"));
      }
    }
    this.emitMessage(
      ComMessage.new(ComType.error, "failed", requestId),
    );
  }
  private async http_detail(http_connection: IVHttpConnection): Promise<MessageCompress[]> {
    const {request, response} = http_connection;
    const list = [];
    if (request) {
      const { request_headers: headers, request_body: body } = http_connection;
      const header_data = await this.pickMultiData(headers);
      const body_data = await this.pickMultiData(body);
      list.push({
        json: this.ctx!.http_header_parse(request, header_data, body_data),
        data: body_data
      });
    }
    if (response) {
      const { response_headers: headers, response_body: body } = http_connection;
      const header_data = await this.pickMultiData(headers);
      const body_data = await this.pickMultiData(body);
      list.push({
        json: this.ctx!.http_header_parse(response, header_data, body_data),
        data: body_data
      });
    }
    return list;
  }
  private async pickMultiData(segments: [number, number][]): Promise<Uint8Array> {
    if (segments.length === 0) {
      return new Uint8Array(0);
    }
    const list = [];
    for (const segment of segments) {
      list.push(await this.pickData(segment[0], segment[1]));
    }
    return concatLargeUint8Arrays(list);
  }

  
  // private async scope(requestId: string, catelog: string, index: number): Promise<void> {
  //   if (this.ctx) {
  //     try {
  //       switch (catelog) {
  //         case "frame":
  //           const range = this.ctx!.frame_range(index);
  //           this.emitMessage(
  //             ComMessage.new(ComType.FRAME_SCOPE_RES, {start: range.start, end: range.end}, requestId),
  //           );
  //           return;
  //         default:
  //           return;
  //       }
  //     } catch (e) {
  //       console.error(e);
  //     }
  //   }
  //   this.emitMessage(
  //     ComMessage.new(ComType.error, "failed", requestId),
  //   );
  //   return;
  // }
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
          this.ready = true;
          try {
            this.init();
          } catch (e) {
            console.error(e);
            this.printLog(new ComLog("error", "failed to open file"));
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
          const data = body.data as Uint8Array;
          await this.update(data);
          // this.emitMessage(ComMessage.new(ComType.PRGRESS_STATUS, rs));
          break;
        case ComType.log:
          this.printLog(body as ComLog);
          break;
        case ComType.REQUEST:
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

        case ComType.HTTP_DETAIL_REQ:
          const rs = await this.http_detail(body as IVHttpConnection);
          this.emitMessage(
            ComMessage.new(ComType.HTTP_DETAIL_RES, rs, id),
          );
          break;
        default:
        // console.log(msg.body);
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
