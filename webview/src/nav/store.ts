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
  iframe?: HTMLIFrameElement;
  input?: HTMLInputElement;
  reset: () => void;
  loadData: (pfile: PcapFile, data: Uint8Array) => Promise<void>;
  bindElement: (iframe: HTMLIFrameElement | undefined, input: HTMLInputElement | undefined) => void;
}

export const useStore = create<PcapState>()((set, get) => {

  // _log("ui store");
  onMessage("message", (e: MessageEvent) => {
    const data = e.data;
    // _log('message', data);
    if (data.type) {
      if (data.type === ComType.OPEN_FILE) {
        const input = get().input;
        input?.click();
        return;
      }
      worker.postMessage(data);
    }
  });

  worker.onmessage = (e: MessageEvent<any>) => {
    const iframe = get().iframe;
    iframe?.contentWindow?.postMessage(e.data, "*");
  };
  return {
    loadData: async (pfile: PcapFile, data: Uint8Array) => {
      _log('loadData', pfile);
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
      const inputEle = get().input;
      if (inputEle) {
        inputEle.value = '';
      }
    },
    bindElement: (iframe: HTMLIFrameElement | undefined, input: HTMLInputElement | undefined) => {
      set((state) => ({ ...state, iframe, input }));
    },
  };
});
