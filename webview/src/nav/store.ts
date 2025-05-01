// userStore.ts
import { create } from "zustand";
import { onMessage, emitMessage } from "../core/connect";
import { ComLog, ComMessage, ComType } from "../core/common";
import { PCAPClient } from "../core/client";
import init from "rshark";

const ready = init();

interface PcapFile {
  name: string;
  size: number;
  start: number;
  state?: number;
}

class Client extends PCAPClient {
  data?: Uint8Array;
  iframe?: HTMLIFrameElement;
  constructor() {
    super();
  }
  setIframe(iframe: HTMLIFrameElement) {
    this.iframe = iframe;
  }
  printLog(log: ComLog): void {
    console.log(log.level, log.msg);
  }
  emitMessage(msg: ComMessage<any>): void {
    this.iframe?.contentWindow?.postMessage(msg, "*");
  }
}
const _log = console.log.bind(console);

interface PcapState {
  fileInfo?: PcapFile;
  // loading: boolean;
  // status: string;
  // init: boolean;
  client: Client;
  loadFile: (file: PcapFile) => void;
  unloadFile: () => void;
  loadData: (data: Uint8Array) => Promise<void>;
  // sendServerReady: () => void;
  // request: () => void;
  loadIFrame: (iframe: HTMLIFrameElement | null) => void;
}

export const useStore = create<PcapState>()((set, get) => {
  ready.then((rs) => {
    console.log("wasm loaded", rs);
  });
  _log("create pcap  web server store");
  onMessage("message", (e: MessageEvent) => {
    const client = get().client;
    if (client) {
      client.handle(e.data);
    }
  });
  return {
    // loading: false,
    // status: "",
    // init: false,
    client: new Client(),
    loadFile: (fileInfo: PcapFile) => {
      set((state) => ({ ...state, fileInfo }));
    },
    unloadFile: () => {
      set((state) => ({ ...state, fileInfo: undefined }));
    },
    loadData: async (data: Uint8Array) => {
      const client = get().client;
      const rs = await client.update(data);
      console.log(rs);
    },
    loadIFrame: (iframe: HTMLIFrameElement | null) => {
      const client = get().client;
      if (iframe && !client.iframe) {
        _log("set clinet iframe");
        client.setIframe(iframe);
      }
    },
  };
});
