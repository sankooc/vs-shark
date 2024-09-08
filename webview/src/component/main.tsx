import React, { ReactElement, useEffect, useState } from "react";
import { MenuItem } from 'primereact/menuitem';
import { Badge } from 'primereact/badge';
import { Menu } from 'primereact/menu';
import { ComMessage } from '../common';
import Loading from './loading';
import { onMessage, log, emitMessage } from '../connect';
import Overview from './overview';

import FrameList from './frames';
import TCPList from './tcp';
// import ARPReplies from './arp';
import DNSList from './dns';
import { CProto } from "../wasm";
import init, { load, WContext,FrameInfo } from 'rshark';


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
            try {
              // const start = Date.now();
              const ctx = load(body as Uint8Array);
              setData(new CProto(ctx))
              // console.log('spend', Date.now() - start);
            }catch(e){
              console.error(e);
              log('error', 'parse_failed');
            }
          });
        }
      }
    });
    emitMessage(new ComMessage('ready', 'demo'));
  }, []);

  const convert = (props: CProto): MenuItem[] => {
    const mitems: MenuItem[] = [];
    const addPanel = (id: string, label: string, extra: string, icon: string = ''): void => {
      mitems.push({
        id, data: extra, template: itemRenderer, label, icon, className: select === id ? 'active' : '', command: (env) => {
          setSelect(env.item.id);
        }
      });
    };
    const frameCount = props.getFrames().length;
    const tcpCount = props.ctx.get_conversations_count();
    const dnsCount = props.ctx.get_dns_count();
    addPanel('overview', 'Overview', '', 'pi pi-chart-bar');
    addPanel('frame', 'Frame', frameCount + '', 'pi pi-list');
    if (tcpCount) addPanel('tcp', 'TCP', tcpCount + '', 'pi pi-server');
    // if (props.arpGraph?.nodes?.length) addPanel('arp', 'ARP', props.arpGraph?.nodes?.length + '', 'pi pi-chart-pie');
    if (dnsCount) addPanel('dns', 'DNS', dnsCount + '', 'pi pi-address-book');
    return mitems;
  };
  const buildPage = (): ReactElement => {
    switch (select) {
      case 'frame':
        const items: FrameInfo[] = data.ctx.get_frames();
        return <FrameList instance={data}/>;
      case 'tcp':
        return <TCPList instance={data}/>
      case 'dns':
        return <DNSList instance={data}/>
    }
    // const items: FrameInfo[] = data.ctx.get_frames();
    // return <FrameList items={items} ctx={data.ctx} />;
    return <Overview instance={data}/>;
  };
  if (!data || !data.ctx) {
    return <Loading />
  }

  const navItems = convert(data);
  return (<>
    <div className="card h-full">
      <div className="flex flex-row h-full">
        <div className="w-full flex flex-grow-1">
          {buildPage()}
        </div>
        <div className="w-1 flex flex-column flex-grow-0 flex-shrink-0">
          <Menu model={navItems} className="w-full h-full" />
        </div>
      </div>
    </div>
  </>
  );
}

export default Main;