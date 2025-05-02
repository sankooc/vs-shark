import { PCAPClient } from "../core/client";
import { ComLog, ComMessage } from "../core/common";
import init from "rshark";
import { _log } from "../view/util";
const ready = init();
ready.then((rs) => {
  _log("wasm loaded", rs);
});
const ctx: Worker = self as any;

class Client extends PCAPClient {
  emitMessage(msg: ComMessage<any>): void {
    ctx.postMessage(msg);
  }
  data?: Uint8Array;
  constructor() {
    super();
  }
  printLog(log: ComLog): void {
    console.log(log.level, log.msg);
  }
}

const client = new Client();
ctx.addEventListener("message", (event: MessageEvent<any>) => {
  // console.log('-- worker');
  // console.dir(event.data);
  client.handle(event.data);
});

export default null as any;
