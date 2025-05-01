import { load, WContext, Conf } from "rshark";
import { ComLog, ComMessage, ComType } from "./common";

export abstract class PCAPClient {
  level: string = "trace";
  ready: boolean = false;
  ctx?: WContext;
  cost?: number;
  _cache: any = {};
  init(): void {
    if (!this.ctx) {
      this.ctx = load(Conf.new(false));
    }
    // if (!this.ctx && this.data) {
    // try {
    //   // const _start = Date.now();
    //   this.ctx = load(this.data as Uint8Array, Conf.new(false));
    //   // this.cost = Date.now() - _start;
    //   this._info();
    // } catch (e) {
    //   this.emitMessage(new ComMessage('_error', "failed to open file"));
    // }
    // } else {
    //   this._info();
    // }
  }
  async update(data: Uint8Array): Promise<string> {
    if (!this.ctx) {
      this.init();
    }
    if (this.ctx) {
      try {
        const rs = await this.ctx.update(data);
        console.log(rs);
      } catch (e) {
        this.emitMessage(new ComMessage(ComType.error, "failed to open file"));
      }
    }
    return "";
  }
  abstract printLog(log: ComLog): void;
  abstract emitMessage(msg: ComMessage<any>): void;

  handle(msg: ComMessage<any>) {
    if (!msg) return;
    const { type, body } = msg;
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
        case ComType.log:
          this.printLog(body as ComLog);
          break;

        default:
          console.log("unknown type", msg.type);
      }
    } catch (e) {
      console.error(e);
    }
  }
}
