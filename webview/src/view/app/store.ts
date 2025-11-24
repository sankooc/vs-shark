// userStore.ts
import { create } from "zustand";
import { onMessage, emitMessage } from "../../common/connect";
import { _log } from "../util";
import {
  ComMessage,
  ComType,
  deserialize,
  IFrameSelect,
  IHttpDetail,
  ITLSConnect,
  ITLSInfo,
  PcapFile,
  PcapState,
  StatRequest,
} from "../../share/common";
import { IListResult, IProgressStatus, IVConnection, IVConversation, IVHttpConnection, IUDPConversation, IDNSResponse, IDNSRecord } from "../../share/gen";
import mitt from "mitt";


// import convMock from '../mock/conversation.json';
// import connMock from '../mock/connection.json';
// import frameMock from '../mock/frame.json';
// import { buildTheme } from "./fluent/theme";


// const commandMap = new Map<string, any>();
const emitter = mitt();

const doRequest = <F>(data: ComMessage<any>): Promise<F> => {
  emitMessage(data);
  const id = data.id;
  return new Promise<F>((resolve, reject) => {
    emitter.on(id, (event: any) => {
      emitter.off(id);
      if (event == "error") {
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
    // if (type === "vscode-theme-change") {
    //   const _theme = buildTheme();
    //   set((state) => ({ ...state, theme: _theme }));
    //   return;
    // }
    switch (type) {
      case ComType.SERVER_READY: {
        //   emitMessage(ComMessage.new(ComType.CLIENT_READY, Date.now()));
        break;
      }
      case ComType.FILEINFO:
        {
          const fileinfo = body as PcapFile;
          console.log('set fileinfo', fileinfo);
          set((state) => ({ ...state, fileinfo: fileinfo }));
          break;
        }
      case ComType.PRGRESS_STATUS: {
        const progress = deserialize(body) as IProgressStatus;
        set((state) => ({ ...state, progress }));
        break;
      }
      case ComType.STAT_RES:
      case ComType.TLS_RES:
      {
        emitter.emit(id, deserialize(body));
        break;
      }
      case ComType.FRAMES_SELECT:
        {
          const {str, datasource} = body;
          const fr: IFrameSelect = { fields: deserialize(str) || [], datasource };
          emitter.emit(id, fr);
          break;
        }
      case ComType.FRAMES:
      case ComType.CONVERSATIONS:
      case ComType.CONNECTIONS:
      case ComType.HTTP_CONNECTIONS:
      case ComType.UDP_CONNECTIONS:
      case ComType.DNS_CONNECTIONS:
      case ComType.DNS_RCD_CONNECTIONS:
      case ComType.TLS_CONNECTIONS:
      case ComType.TLS_CONVERSATION_ITEMS:
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
  emitMessage(ComMessage.new(ComType.CLIENT_READY, Date.now()));

  return {
    // theme: ctheme,
    sendReady: () => {
      emitMessage(ComMessage.new(ComType.CLIENT_READY, Date.now()));
    },
    request: <F>(data: any): Promise<F> => {
      const req = new ComMessage(ComType.REQUEST, data);
      return doRequest<F>(req);
      // return Promise.resolve(frameMock);
    },
    conversationList: (data: any): Promise<IListResult<IVConversation>> => {
      const req = new ComMessage(ComType.REQUEST, data);
      return doRequest<IListResult<IVConversation>>(req);
    },
    udpList: (data: any): Promise<IListResult<IUDPConversation>> => {
      const req = new ComMessage(ComType.REQUEST, data);
      return doRequest<IListResult<IUDPConversation>>(req);
    },
    dnsList: (data: any): Promise<IListResult<IDNSResponse>> => {
      const req = new ComMessage(ComType.REQUEST, data);
      return doRequest<IListResult<IDNSResponse>>(req);
    },
    dnsRecords: (data: any): Promise<IListResult<IDNSRecord>> => {
      const req = new ComMessage(ComType.REQUEST, data);
      return doRequest<IListResult<IDNSRecord>>(req);
    },
    tlsList: (data: any) => {
      const req = new ComMessage(ComType.REQUEST, data);
      return doRequest<IListResult<ITLSConnect>>(req);
    },
    tlsConvList: (data: any) => {
      const req = new ComMessage(ComType.REQUEST, data);
      return doRequest<IListResult<ITLSInfo>>(req);
    },
    connectionList: (data: any): Promise<IListResult<IVConnection>> => {
      const req = new ComMessage(ComType.REQUEST, data);
      return doRequest<IListResult<IVConnection>>(req);
      // return Promise.resolve(connMock);
    },
    httpList: (data: any): Promise<IListResult<IVHttpConnection>> => {
      const req = new ComMessage(ComType.REQUEST, data);
      return doRequest<IListResult<IVHttpConnection>>(req);
    },
    httpDetail: (index: number): Promise<IHttpDetail[]> => {
      const req = new ComMessage(ComType.HTTP_DETAIL_REQ, {index});
      return doRequest<IHttpDetail[]>(req);
    },
    stat: (request: StatRequest): Promise<any[]> => {
      const req = new ComMessage(ComType.STAT_REQ, request);
      return doRequest<any[]>(req);
    },
  };
});
