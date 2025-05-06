import { PCAPClient } from "../share/client";
import { ComLog, ComMessage, ComType } from "../share/common";
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
  data: Uint8Array = new Uint8Array();
  constructor() {
    super();
  }
  printLog(log: ComLog): void {
    console.log(log.level, log.msg);
  }
  appendData(data: Uint8Array): void {
    this.data = Uint8Array.from([...this.data, ...data]);
  }
}

const client = new Client();
ctx.addEventListener("message", (event: MessageEvent<any>) => {
  const id = event.data?.id;
  const type = event.data?.type;
  if (type == ComType.DATA && id) {
    const { start, size } = event.data.body;
    if (start >= 0 && size > 0 && client.data!.length > start + size) {
      const _data = client.data!.slice(start, start + size);
      ctx.postMessage({ type: ComType.RESPONSE, id, body: { data: _data } }, [
        _data.buffer,
      ]);
    } else {
      ctx.postMessage({ type: ComType.RESPONSE, id });
    }
    return;
  }
  client.handle(event.data);
});

export default null as any;
