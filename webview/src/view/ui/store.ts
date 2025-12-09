// userStore.ts
import { create } from "zustand";
// import { onMessage, emitMessage } from "../common/connect";
import { _log } from "../util";
import {
  IHttpDetail,
  PcapFile,
  PcapState,
  StatRequest,
} from "../../share/common";
import { IListResult, IVConnection, IVConversation, IVHttpConnection, IUDPConversation, IDNSResponse, IDNSRecord } from "../../share/gen";

const makeUrl = (base: string, params: Record<string, string>): string => {
  return `${base}?start=${params.start}&size=${params.size}`
}

const httpdetail_convert = (data: any): IHttpDetail => {
  const { headers, raw, plaintext, content_type } = data;
  let _raw = undefined;
  if (raw && raw.length) {
      _raw = Uint8Array.from(raw);
  }
  return { headers, raw: _raw, plaintext, content_type }
}

export const useStore = create<PcapState>()((set) => {
  _log("create pcap ui store");

  fetch('/api/ready').then((rs) => {
    if (rs && rs.ok) {
      return rs.json()
    }
    return Promise.reject();
  }).then((data) => {
    const total = data.size;
    const fileinfo = data as PcapFile;
    const progress = { total, cursor: total, count: 0, left: 0 };
    set((state: any) => ({ ...state, fileinfo, progress }));
  });
  return {
    sendReady: () => {
      // emitMessage(ComMessage.new(ComType.CLIENT_READY, Date.now()));
    },
    request: <F>(data: any): Promise<F> => {
      switch (data.type) {
        case 'select': {
          if (data.catelog === 'frame') {
            const { index } = data.param;
            return fetch(`/api/frame/${index}`).then((response) => response.json());
          }

          break;
        }
        case 'list':
          {
            if (data.catelog === 'frame') {
              const { param } = data;
              return fetch(`/api/frames?start=${param.start}&size=${param.size}`).then((response) => response.json());
            }
          }
      }
      return Promise.resolve({} as F);
    },
    conversationList: (data: any): Promise<IListResult<IVConversation>> => {
      const { start, size } = data.param;
      let url = `/api/tcp/list?start=${start}&size=${size}`
      if (data.param.ip) {
        url += `&ip=${data.param.ip}`
      }
      return fetch(url).then((response) => response.json());
    },
    udpList: (data: any): Promise<IListResult<IUDPConversation>> => {
      let url = `/api/udp/list?start=${data.param.start}&size=${data.param.size}&asc=${data.param.asc}`;
      if (data.param.ip) {
        url += `&ip=${data.param.ip}`
      }
      //asc
      return fetch(url).then((response) => response.json());
    },
    dnsList: (data: any): Promise<IListResult<IDNSResponse>> => {
      const url = `/api/dns/list?start=${data.param.start}&size=${data.param.size}&asc=${data.param.asc}`;
      return fetch(url).then((response) => response.json());
    },
    dnsRecords: (data: any): Promise<IListResult<IDNSRecord>> => {
      const url = `/api/dns/detail/${data.param.index}?start=${data.param.start}&size=${data.param.size}`;
      return fetch(url).then((response) => response.json());
    },
    tlsList: (data: any) => {
      const url = makeUrl('/api/tls/list', data.param);
      return fetch(url).then((response) => response.json());
    },
    tlsConvList: (data: any) => {
      const url = makeUrl(`/api/tls/detail/${data.param.index}`, data.param);
      return fetch(url).then((response) => response.json());
    },
    connectionList: (data: any): Promise<IListResult<IVConnection>> => {
      const { start, size, conversionIndex } = data.param;
      const url = `/api/tcp/conv/${conversionIndex}/list?start=${start}&size=${size}`;
      return fetch(url).then((response) => response.json());
    },
    httpList: (data: any): Promise<IListResult<IVHttpConnection>> => {
      let url = makeUrl('/api/http/list', data.param);
      if (data.param.host) {
        url += `&host=${data.param.host}`;
      }
      url += `&asc=${data.param.asc}`;
      return fetch(url).then((response) => response.json());
    },
    httpDetail: (index: number): Promise<IHttpDetail[]> => {
      const url = `/api/http/detail/${index}`;
      return fetch(url).then((response) => response.json()).then((rs) => { return rs.map(httpdetail_convert); });
    },
    stat: (request: StatRequest): Promise<any[]> => {
      const { field } = request;
      return fetch(`/api/stat/${field}`).then((response) => response.json());
    },
    openFile: async () => {
    },
    closeFile: async () => {
    }
  };
});
