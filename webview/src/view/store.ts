// userStore.ts
import { create } from "zustand";
import { onMessage, emitMessage } from "../common/connect";
import { _log } from "./util";
import { ComMessage, ComType, DataResponse, deserialize, PcapFile } from "../share/common";
import { IFrameInfo, IListResult, IProgressStatus } from "../share/gen";
import mitt from 'mitt'

interface PcapState {
  fileinfo?: PcapFile;
  progress?: IProgressStatus;
  frameResult?: IListResult<IFrameInfo>;
  frameSelect?: string;
  sendReady: () => void;
  request: <F>(data: any) => Promise<F>;
  requestData: (data: {start: number, size: number}) => Promise<DataResponse>;
  // frameList: (page: number, size: number) => Promise<IListResult<IFrameInfo>>;
}
// const compute = (page: number, size: number): Pagination => {
//   if (page < 1) {
//     return { start: 0, size: size };
//   }
//   const start = (page - 1) * size;
//   return { start, size };
// };


// const commandMap = new Map<string, any>();
const emitter = mitt()

const doRequest = <F>(data: ComMessage<any>): Promise<F>  => {
  emitMessage(data);
  const id = data.id;
  return new Promise<F>((resolve, _) => {
    emitter.on(id, (event: any) => {
      emitter.off(id);
      resolve(event as F);
    });
  });
}

export const useStore = create<PcapState>()((set) => {
  _log("create pcap store");
  onMessage("message", (e: any) => {
    const { type, body, id } = e.data;
    switch (type) {
      case ComType.SERVER_REDAY: {
        //   emitMessage(ComMessage.new(ComType.CLIENT_REDAY, Date.now()));
        break;
      }
      case ComType.FILEINFO:
        const fileinfo = body as PcapFile;
        set((state) => ({ ...state, fileinfo: fileinfo }));
        break;
      case ComType.PRGRESS_STATUS:
        const progress = deserialize(body) as IProgressStatus;
        set((state) => ({ ...state, progress }));
        break;
      case ComType.FRAMES:
      case ComType.FRAMES_SELECT:
        emitter.emit(id, deserialize(body));
        break;
      case ComType.RESPONSE:
        emitter.emit(id, body);
    }
  });
  return {
    sendReady: () => {
      emitMessage(ComMessage.new(ComType.CLIENT_REDAY, Date.now()));
    },
    request: <F>(data: any): Promise<F> => {
      const req = new ComMessage(ComType.REQUEST, data);
      return doRequest<F>(req);
    },
    requestData: (data: {start: number, size: number}): Promise<DataResponse> => {
      const req = new ComMessage(ComType.DATA, data);
      return doRequest<DataResponse>(req);
    },

    // frameList: (page: number, size: number): Promise<IListResult<IFrameInfo>> => {
    //   const _req = new ComMessage(ComType.REQUEST, {
    //     catelog: "frame",
    //     type: "list",
    //     param: compute(page, size),
    //   });
      
    // }
  };
});
