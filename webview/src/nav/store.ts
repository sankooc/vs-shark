import { create } from "zustand";
import { onMessage } from "../common/connect";
import { ComMessage, ComType, PcapFile } from "../share/common";

const worker = new Worker(new URL("./worker.ts", import.meta.url), {
  type: "module",
});

export interface PFile {
  name: string;
  size: number;
}
const _log = console.log.bind(console);

interface PcapState {
  // fileInfo?: PcapFile;
  iframe?: HTMLIFrameElement;
  // loadFile: (file: PcapFile) => void;
  // unloadFile: () => void;
  send: (message: ComMessage<any>) => void;
  reset: () => void;
  loadData: (pfile: PcapFile, data: Uint8Array) => Promise<void>;
  loadIFrame: (iframe: HTMLIFrameElement | null) => void;
}

export const useStore = create<PcapState>()((set, get) => {
  _log("create pcap  web server store");
  onMessage("message", (e: MessageEvent) => {
    const data = e.data;
    if (data.type) {
      // _log("server accept", data.type);
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
    loadData: async (pfile: PcapFile, data: Uint8Array) => {
      {
        const message = ComMessage.new(ComType.TOUCH_FILE, pfile);
        worker.postMessage(message);
      }
      {
        const message = ComMessage.new(ComType.PROCESS_DATA, { data });
        worker.postMessage(message, [data.buffer]);
      }
    },
    reset: () => {
      const message = ComMessage.new(ComType.RESET, {});
      worker.postMessage(message);
    },
    loadIFrame: (iframe: HTMLIFrameElement | null) => {
      const frame = get().iframe;
      if (!frame && iframe) {
        set((state) => ({ ...state, iframe }));
      }
    },
  };
});
