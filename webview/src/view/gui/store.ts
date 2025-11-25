// userStore.ts
import { create } from "zustand";
import { _log } from "../util";
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event';
import {
  IHttpDetail,
  PcapFile,
  PcapState,
  StatRequest,
} from "../../share/common";
import { IListResult, IVConnection, IVConversation, IVHttpConnection, IUDPConversation, IDNSResponse, IDNSRecord } from "../../share/gen";

const httpdetail_convert = (data: any): IHttpDetail => {
  const { headers, raw, plaintext, content_type } = data;
  let _raw = undefined;
  if (raw && raw.length) {
    _raw = Uint8Array.from(raw);
  }
  return { headers, raw: _raw, plaintext, content_type }
}

export const useStore = create<PcapState>()((set) => {
  _log("create gui store");

  listen<PcapFile>('file_touch', (event) => {
    // _log(`file_touch ${event.payload}`);
    // let pf = event.payload;
    const fileinfo = event.payload;
    set((state) => ({ ...state, fileinfo}));
  });

  listen<boolean>('parse_complete', (event) => {
    if (event.payload) {
      set((state) => ({ ...state, progress: { total: 0, cursor: 0, count: 0, left: 0 } }));
    } else {
      _log('file closed');
      set((state) => ({ ...state, fileinfo: undefined, progress: undefined }));
    }
  });
  listen<string>('file_close', () => {
    _log('file closed');
    set((state) => ({ ...state, progress: undefined }));
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
            return invoke("frame", { index });
          }

          break;
        }
        case 'list':
          {
            if (data.catelog === 'frame') {
              const { param } = data;
              return invoke("frames", { start: param.start, size: param.size });
            }
          }
      }
      return Promise.resolve({} as F);
    },
    conversationList: async (data: any): Promise<IListResult<IVConversation>> => {
      const { start, size, ip } = data.param;
      return invoke("tcp_list", { start, size, ip: ip ? ip : undefined });
    },
    udpList: (data: any): Promise<IListResult<IUDPConversation>> => {
      const { start, size, asc, ip } = data.param;
      return invoke("udp_list", { start, size, asc, ip: ip ? ip : undefined });
    },
    dnsList: (data: any): Promise<IListResult<IDNSResponse>> => {
      return invoke("dns_records", { start: data.param.start, size: data.param.size, asc: data.param.asc });
    },
    dnsRecords: (data: any): Promise<IListResult<IDNSRecord>> => {
      return invoke("dns_record", { index: parseInt(data.param.index), start: data.param.start, size: data.param.size });
    },
    tlsList: (data: any) => {
      return invoke("tls_list", { start: data.param.start, size: data.param.size });
    },
    tlsConvList: (data: any) => {
      return invoke("tls_conv_list", { index: parseInt(data.param.index), start: data.param.start, size: data.param.size });
    },
    connectionList: (data: any): Promise<IListResult<IVConnection>> => {
      const { start, size, conversionIndex } = data.param;
      return invoke("tcp_conv_list", { start, size, index: parseInt(conversionIndex) });
    },
    httpList: (data: any): Promise<IListResult<IVHttpConnection>> => {
      return invoke("http_list", { start: data.param.start, size: data.param.size, host: data.param.host, asc: data.param.asc });
    },
    httpDetail: (index: number): Promise<IHttpDetail[]> => {
      return invoke("http_detail", { index }).then((rs) => { return (rs as any[]).map(httpdetail_convert); })
    },
    stat: (request: StatRequest): Promise<any[]> => {
      const { field } = request;
      return invoke("stat", { field }).then((rs) => (JSON.parse(rs as string) as any[]));
    },
  };
});
