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
  MessageCompress,
  PcapFile,
  VRange,
} from "../share/common";
import { IFrameInfo, IListResult, IProgressStatus, IVConnection, IVConversation, IVHttpConnection } from "../share/gen";
import mitt from "mitt";


// import convMock from '../mock/conversation.json';
// import connMock from '../mock/connection.json';

interface PcapState {
  fileinfo?: PcapFile;
  progress?: IProgressStatus;
  frameResult?: IListResult<IFrameInfo>;
  frameSelect?: string;
  sendReady: () => void;
  request: <F>(data: any) => Promise<F>;
  requestData: (data: VRange) => Promise<DataResponse>;
  conversations: (data: any) => Promise<IListResult<IVConversation>>;
  connections: (data: any) => Promise<IListResult<IVConnection>>;
  httpConnections: (data: any) => Promise<IListResult<IVHttpConnection>>;
  httpDetail: (data: IVHttpConnection) => Promise<MessageCompress[]>
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
      case ComType.CONVERSATIONS:
      case ComType.CONNECTIONS:
      case ComType.HTTP_CONNECTIONS:
        emitter.emit(id, deserialize(body));
        break;
      case ComType.FRAME_SCOPE_RES:
      case ComType.HTTP_DETAIL_RES:
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
    conversations: (data: any): Promise<IListResult<IVConversation>> => {
      const req = new ComMessage(ComType.REQUEST, data);
      return doRequest<IListResult<IVConversation>>(req);
      // return Promise.resolve(convMock);
    },
    connections: (data: any): Promise<IListResult<IVConnection>> => {
      const req = new ComMessage(ComType.REQUEST, data);
      return doRequest<IListResult<IVConnection>>(req);
      // return Promise.resolve(connMock);
    },
    httpConnections: (data: any): Promise<IListResult<IVHttpConnection>> => {
      const req = new ComMessage(ComType.REQUEST, data);
      return doRequest<IListResult<IVHttpConnection>>(req);
    },
    httpDetail: (data: IVHttpConnection): Promise<MessageCompress[]> => {
      const req = new ComMessage(ComType.HTTP_DETAIL_REQ, data);
      return doRequest<MessageCompress[]>(req);
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
