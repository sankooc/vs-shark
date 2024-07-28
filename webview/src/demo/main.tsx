import React, { ReactElement, SyntheticEvent, useEffect, useState } from "react";
import { MenuItem } from 'primereact/menuitem';
import { Badge } from 'primereact/badge';
import { Menu } from 'primereact/menu';
import { ComMessage, IDNSRecord } from '../common';
import Loading from './loading';
import { onMessage, emitMessage } from '../connect';
import Overview from './overview';

import FrameList from './frames';
import TCPList from './tcp';
import ARPReplies from './arp';
import DNSList from './dns';
import { DNSRecord } from "nshark/built/src/common";
import { Client, CProto } from "../client";
import { ComLog, Panel, MainProps, HexV } from "../common";
import init, { load, WContext,FrameInfo } from 'rshark';

class BrowserClient extends Client {
  selectFrame(no: number): void {
  }
  renderHexView(data: HexV): void {
  }
  emitMessage(panel: Panel, msg: ComMessage<any>): void {
  }
  printLog(log: ComLog): void {

  }

}


const itemRenderer = (item, options) => {
  return <a className="flex align-items-center px-3 py-2 cursor-pointer" onClick={options.onClick}>
    {item.icon && <span className={item.icon} />}
    <span className={`mx-2 ${item.items && 'font-semibold'}`}>{item.label}</span>
    {item.data && <Badge className="ml-auto" value={item.data} />}
  </a>
}; //pi-chart-bar

const initPro = init();
const Main = () => {
  const [select, setSelect] = useState('overview');
  const [data, setData] = useState<CProto>(null);
  useEffect(() => {
    onMessage('message', (e: any) => {
      const { type, body, requestId } = e.data;
      switch (type) {
        case 'raw-data': {
          initPro.then(() => {
            const ctx = load(body as Uint8Array);
            setData({ctx})
            // const client = new BrowserClient();
            // client.initData(body);
            // try {
            //   const ret = client.init();
            //   setData(ret);
            // } catch (e) {
            //   console.error(e);
            //   emitMessage(new ComMessage<ComLog>('log', new ComLog('error', 'invalid_file_format')));
            // }
          });
        }
      }
    });
    emitMessage(new ComMessage('ready', 'demo'));
  }, []);

  const convert = (props: CProto): MenuItem[] => {
    const mitems: MenuItem[] = [];
    // if (!props) return [];
    const addPanel = (id: string, label: string, extra: string, icon: string = ''): void => {
      mitems.push({
        id, data: extra, template: itemRenderer, label, icon, className: select === id ? 'active' : '', command: (env) => {
          setSelect(env.item.id);
        }
      });
    };
    // addPanel('overview', 'Overview', '', 'pi pi-chart-bar');
    addPanel('frame', 'Frame', '', 'pi pi-list');
    // if (props.tcps?.length) addPanel('tcp', 'TCP', props.tcps.length + '', 'pi pi-server');
    // if (props.arpGraph?.nodes?.length) addPanel('arp', 'ARP', props.arpGraph?.nodes?.length + '', 'pi pi-chart-pie');
    // if (props.dnsRecords?.length) addPanel('dns', 'DNS', props.dnsRecords?.length + '', 'pi pi-address-book');
    return mitems;
  };
  const buildPage = (): ReactElement => {
    // switch (select) {
    //   case 'frame':
    //     const items: FrameInfo[] = data.ctx.get_frames();
    //     return <FrameList items={items} ctx={data.ctx} />;
      // case 'tcp':
      //   return <TCPList items={data.tcps} />

      // case 'arp':
      //   return <ARPReplies graph={data.arpGraph} legends={['sender', 'target']} />
      // case 'dns':
      //   return <DNSList items={data.dnsRecords.map((record: DNSRecord, inx: number) => {
      //     const r = new IDNSRecord(record);
      //     r.no = inx + 1;
      //     return r;
      //   })} />
    // }
    const items: FrameInfo[] = data.ctx.get_frames();
    return <FrameList items={items} ctx={data.ctx} />;
    // return <Overview data={data.overview} />;
  };
  if (!data || !data.ctx) {
    return <Loading />
  }

  const navItems = convert(data);
  return (<>
    <div className="card h-full">
      <div className="flex flex-row h-full">
        <div className="w-2 flex flex-column flex-grow-0 flex-shrink-0">
          <Menu model={navItems} className="w-full h-full" />
        </div>
        <div className="w-full flex flex-grow-1">
          {buildPage()}
        </div>
      </div>
    </div>
  </>
  );
}

export default Main;