import { BATCH_SIZE, PCAPClient } from "../share/client";
import { ComLog, ComMessage, ComType, VRange } from "../share/common";
import init, { Range } from "rshark";
import { _log } from "../view/util";
const ready = init();


const self_ = self as any;
const g_map: any = {

};
self_.load_data = (_id: string, range: Range) => {
  if (g_map.current?.data) {
    return g_map.current.data.slice(range.start, range.end);
  }
  return new Uint8Array();
};

self_.loads_data = (id: string, ranges: Range[]) => {
  if (ranges.length === 0) {
    return new Uint8Array(0);
  }
  const list = [];
  for (const segment of ranges) {
    list.push(self_.load_data(id, segment));
  }
  return concatLargeUint8Arrays(list);
};

function concatLargeUint8Arrays(arrays: Uint8Array[]): Uint8Array {
  const totalLength = arrays.reduce((acc, arr) => acc + arr.length, 0);
  const buffer = new ArrayBuffer(totalLength);
  const result = new Uint8Array(buffer);
  let offset = 0;
  for (const arr of arrays) {
    result.set(arr, offset);
    offset += arr.length;
  }
  return result;
}

self_.wasm_log = (str: string) => {
  console.log('[wasm]: ', str);
};
ready.then((rs) => {
  _log("wasm loaded", rs);

  class Client extends PCAPClient {
    doReady(): void {
      this.init();
    }
    async pickData(start: number, end: number): Promise<Uint8Array> {
      const _data = client.data!.slice(start, end);
      return _data;
    }
    emitMessage(msg: ComMessage<any>): void {
      ctx.postMessage(msg);
    }
    public data: Uint8Array = new Uint8Array();
    constructor() {
      super();
    }
    printLog(log: ComLog): void {
      console.log(log.level, log.msg);
    }
    appendData(data: Uint8Array): void {
      const newData = new Uint8Array(this.data.length + data.length);
      newData.set(this.data, 0);
      newData.set(data, this.data.length);
      this.data = newData;
    }
  }

  const client = new Client();
  g_map.current = client;
  ctx.addEventListener("message", (event: MessageEvent<any>) => {
    const id = event.data?.id;
    const type = event.data?.type;
    if (type == ComType.DATA && id) {
      const r = event.data.body as VRange;
      const start = r.start;
      const size = r.end - r.start;
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
    if (type == ComType.PROCESS_DATA) {
      let body = event.data.body;
      const data = body.data as Uint8Array;
      client.data = data;
      if (data.length <= BATCH_SIZE) {
        client.handle(event.data);
      } else {
        for (let i = 0; i < data.length; i += BATCH_SIZE) {
          let _data = data.subarray(i, i + BATCH_SIZE);
          const e = { id, type, body: { data: _data } };
          client.handle(e);
        }
      }
      return;
    }
    client.handle(event.data);
  });
});
const ctx: Worker = self as any;

export default null as any;
