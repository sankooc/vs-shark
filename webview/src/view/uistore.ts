// userStore.ts
import { create } from "zustand";
// import { onMessage, emitMessage } from "../common/connect";
import { _log } from "./util";
import {
  ComMessage,
  ComType,
  DataResponse,
  IHttpDetail,
  ITLSConnect,
  ITLSInfo,
  PcapState,
  StatRequest,
  VRange,
} from "../share/common";
import { IListResult, IVConnection, IVConversation, IVHttpConnection, IUDPConversation, IDNSResponse, IDNSRecord } from "../share/gen";
import mitt from "mitt";


// import convMock from '../mock/conversation.json';
// import connMock from '../mock/connection.json';
// import frameMock from '../mock/frame.json';
// import { buildTheme } from "./fluent/theme";


// const commandMap = new Map<string, any>();
const emitter = mitt();

const doRequest = <F>(data: ComMessage<any>): Promise<F> => {
  // emitMessage(data);
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
  _log("create pcap ui store");

  let httpCache: IVHttpConnection | null = null;

  // emitMessage(ComMessage.new(ComType.CLIENT_READY, Date.now()));
  fetch('/api/ready').then(() => {
    console.log('is ready');
      set((state) => ({ ...state, progress: { total: 0, cursor: 0, count: 0, left: 0} }));
  });
  return {
    // theme: ctheme,
    sendReady: () => {
      console.log('ready');
      // emitMessage(ComMessage.new(ComType.CLIENT_READY, Date.now()));
    },
    request: <F>(data: any): Promise<F> => {
      console.log('req', data);
      switch (data.type) {
        case 'list':
          {
            if(data.catelog === 'frame'){
              const { param } = data;
              return fetch(`/api/frames?start=${param.start}&size=${param.size}`).then((response) => response.json());
            }
            // const req = new ComMessage(data.type, data.payload);
            // return doRequest<F>(req);
          }
      }
      return Promise.resolve({} as F);
      // const req = new ComMessage(ComType.REQUEST, data);
      // return doRequest<F>(req);
      // return Promise.resolve(frameMock);
    },
    requestData: (data: VRange): Promise<DataResponse> => {
      const req = new ComMessage(ComType.DATA, data);
      return doRequest<DataResponse>(req);
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
    cachehttp: (conn: IVHttpConnection | null) => {
      httpCache = conn;
    },
    getHttpCache: () => {
      return httpCache;
    },
    stat: (request: StatRequest): Promise<any[]> => {
      const req = new ComMessage(ComType.STAT_REQ, request);
      return doRequest<any[]>(req);
    },
  };
});
