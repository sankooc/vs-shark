// userStore.ts
import { create } from "zustand";
import { onMessage, emitMessage } from "../common/connect";
import { _log } from "./util";
import {
  ComMessage,
  ComType,
  DataResponse,
  deserialize,
  IFrameSelect,
  PcapFile,
  VRange,
} from "../share/common";
import { IFrameInfo, IListResult, IProgressStatus } from "../share/gen";
import mitt from "mitt";

interface PcapState {
  fileinfo?: PcapFile;
  progress?: IProgressStatus;
  frameResult?: IListResult<IFrameInfo>;
  frameSelect?: string;
  sendReady: () => void;
  request: <F>(data: any) => Promise<F>;
  requestData: (data: VRange) => Promise<DataResponse>;
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
const emitter = mitt();

const doRequest = <F>(data: ComMessage<any>): Promise<F> => {
  emitMessage(data);
  const id = data.id;
  return new Promise<F>((resolve, reject) => {
    emitter.on(id, (event: any) => {
      emitter.off(id);
      if (event == "error"){
        reject("error");
      } else {
        resolve(event as F);
      }
    });
  });
};

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
      // case ComType.FRAME_SCOPE_RES:
      //   const range = body as VRange;
      //   break;
      
      case ComType.FRAMES_SELECT:
        let fr: IFrameSelect = { start: body.start, end: body.end, data: body.data, fields: deserialize(body.liststr), extra: body.extra };
        emitter.emit(id, fr);
        break;
      case ComType.FRAMES:
        emitter.emit(id, deserialize(body));
        break;
      case ComType.FRAME_SCOPE_RES:
        emitter.emit(id, body);
        break;
      case ComType.RESPONSE:
        emitter.emit(id, body);
        break;
      case ComType.error:
        emitter.emit(id, "error");
        break;
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
    requestData: (data: VRange): Promise<DataResponse> => {
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
