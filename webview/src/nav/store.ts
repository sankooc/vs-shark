// userStore.ts

import { create } from "zustand";
import { onMessage } from "../core/connect";
import { ComMessage, ComType } from "../core/common";

const worker = new Worker(new URL("./worker.ts", import.meta.url), {
  type: "module",
});

// class Client extends PCAPClient {
//   data?: Uint8Array;
//   iframe?: HTMLIFrameElement;
//   constructor() {
//     super();
//   }
//   setIframe(iframe: HTMLIFrameElement) {
//     this.iframe = iframe;
//   }
//   printLog(log: ComLog): void {
//     console.log(log.level, log.msg);
//   }
//   emitMessage(msg: ComMessage<any>): void {
//     this.iframe?.contentWindow?.postMessage(msg, "*");
//   }
// }
const _log = console.log.bind(console);

interface PcapState {
  // fileInfo?: PcapFile;
  iframe?: HTMLIFrameElement;
  // loadFile: (file: PcapFile) => void;
  // unloadFile: () => void;
  send: (message: ComMessage<any>) => void;
  loadData: (data: Uint8Array) => Promise<void>;
  loadIFrame: (iframe: HTMLIFrameElement | null) => void;
}

export const useStore = create<PcapState>()((set, get) => {
  _log("create pcap  web server store");
  onMessage("message", (e: MessageEvent) => {
    const data = e.data;
    if (data.type) {
      _log('client accept', data.type);
      worker.postMessage(data);
    }
  });

  worker.onmessage = (e: MessageEvent<any>) => {
    const iframe = get().iframe;
    if (iframe) {
      iframe.contentWindow?.postMessage(e.data, "*");
    }
  };
  return {
    // loading: false,
    // status: "",
    // init: false,
    // client: new Client(),
    send: (message: ComMessage<any>) => {
      worker.postMessage(message);
    },
    // loadFile: (fileInfo: PcapFile) => {
    //   set((state) => ({ ...state, fileInfo }));
    //   // worker.postMessage(fileInfo);
    //   // const client = get().client;
    //   // if(client){
    //   //   client.emitMessage(ComMessage.new(ComType.TOUCH_FILE, fileInfo));
    //   // }
    // },
    // unloadFile: () => {
    //   set((state) => ({ ...state, fileInfo: undefined }));
    //   // const client = get().client;
    //   // if(client){
    //   //   client.emitMessage(ComMessage.new(ComType.FILE_CLOSE, ""));
    //   // }
    // },
    loadData: async (data: Uint8Array) => {
      // console.log("in data", data.length);
      const message = ComMessage.new(ComType.PROCESS_DATA, { data });
      // worker.postMessage({ type: 'upload_data', body: data }, [data.buffer]);
      worker.postMessage(message, [data.buffer]);
      // console.log("in data", data.length);
      // worker.postMessage(fileInfo);
      // const client = get().client;
      // const rs = await client.update(data);
      // console.log(rs);
    },
    loadIFrame: (iframe: HTMLIFrameElement | null) => {
      const frame = get().iframe;
      if (!frame && iframe) {
        set((state) => ({ ...state, iframe }));
      }
    },
  };
});
