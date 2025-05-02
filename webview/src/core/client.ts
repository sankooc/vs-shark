import { load, WContext, Conf } from "rshark";
import { ComLog, ComMessage, ComRequest, ComType, PcapFile } from "./common";

export abstract class PCAPClient {
  level: string = "trace";
  ready: boolean = false;
  ctx?: WContext;
  info?: PcapFile;
  init(): void {
    if (!this.ctx) {
      this.ctx = load(Conf.new(false));
    }
  }
  async update(data: Uint8Array): Promise<string> {
    if (!this.ctx) {
      this.init();
    }
    if (this.ctx) {
      try {
        const rs = await this.ctx.update(data);
        this.emitMessage(ComMessage.new(ComType.PRGRESS_STATUS, rs));
        return rs;
      } catch (e) {
        this.emitMessage(new ComMessage(ComType.error, "failed to open file"));
      }
    }
    return "";
  }
  abstract printLog(log: ComLog): void;
  abstract emitMessage(msg: ComMessage<any>): void;

  touchFile(fileInfo: PcapFile): void {
    this.info = fileInfo;
    this.emitMessage(ComMessage.new(ComType.FILEINFO, fileInfo));
  }
  list(requestId: string, catelog: string, start: number, size: number): void {
    if (this.ctx) {
      try {
        let rs;
        switch (catelog) {
          case "frame":
            rs = this.ctx.select("frame", start, size);
            this.emitMessage(ComMessage.new(ComType.FRAMES, rs, requestId));
            break;
          default:
            return;
        }
      } catch (e) {
        this.emitMessage(new ComMessage(ComType.error, "failed"));
      }
    }
  }
  handle(msg: ComMessage<any>) {
    if (!msg) return;
    const { type, body, id } = msg;
    if (!type) return;
    try {
      switch (type) {
        case ComType.CLIENT_REDAY:
          this.ready = true;
          console.log("is ready");
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
        case ComType.PROCESS_DATA:
          const data = body.data as Uint8Array;
          this.update(data).then((rs) => {
            this.emitMessage(ComMessage.new(ComType.PRGRESS_STATUS, rs));
          });
          break;
        case ComType.log:
          this.printLog(body as ComLog);
          break;
        case ComType.REQUEST:
          const req: ComRequest = body;
          const { catelog, type, param } = req;
          switch (type) {
            case "list":
              this.list(id, catelog, param.start, param.size);
              break;
          }
          break;
        default:
          // console.log("unknown type", msg.type);
        // console.log(msg.body);
      }
    } catch (e) {
      console.error(e);
    }
  }
}
