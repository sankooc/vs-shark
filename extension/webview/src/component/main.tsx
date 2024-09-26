import React, { ReactElement, useEffect, useState } from "react";
import { MenuItem } from 'primereact/menuitem';
import { Badge } from 'primereact/badge';
import { Menu } from 'primereact/menu';
import { ComMessage, IContextInfo, IConversation, IDNSRecord } from '../common';
import Loading from './loading';
// import Loading from './error/loading';
import ErrPage from './error';
import { onMessage, log, emitMessage } from '../connect';

import Overview from './overview';
import FrameList from './frames';
import TCPList from './tcp';
import DNSList from './dns';
import HttpComponnet from './http';


const itemRenderer = (item, options) => {
  return <a className="flex align-items-center px-3 py-2 cursor-pointer" onClick={options.onClick}>
    {item.icon && <span className={item.icon} />}
    <span className={`mx-2 ${item.items && 'font-semibold'}`}>{item.label}</span>
    {item.data && <Badge className="ml-auto" value={item.data} />}
  </a>
};

let _start = 0;
const Main = () => {
  const [select, setSelect] = useState('overview');
  const [status, setStatus] = useState<number>(0);
  const [meta, setMeta] = useState<IContextInfo>(null);
  const [dnsRecords, setDnsRecords] = useState<IDNSRecord[]>([]);
  const [conversations, setConversations] = useState<IConversation[]>([]);
  useEffect(() => {
    onMessage('message', (e: any) => {
      const { type, body, requestId } = e.data;
      switch (type) {
        case '_info': {
          // console.log(JSON.stringify(body));
          setMeta(body);
          setStatus(1);
          
          emitMessage(new ComMessage('http', ''));
          break;
        }
        case '_http': {
          console.log(JSON.stringify(body));
          break;
        }
        case '_error': {
          setStatus(2);
          break;
        }
        case '_dns': {
          setDnsRecords(body);
          break;
        }
        case '_conversation': {
          setConversations(body);
          break;
        }
      }
    });
    _start = Date.now();
    emitMessage(new ComMessage('ready', 'demo'));
  }, []);
  

  const convert = (): MenuItem[] => {
    const mitems: MenuItem[] = [];
    const addPanel = (id: string, label: string, extra: string, icon: string = ''): void => {
      mitems.push({
        id, data: extra, template: itemRenderer, label, icon, className: select === id ? 'active' : '', command: (env) => {
          setSelect(env.item.id);
        }
      });
    };
    addPanel('overview', 'Overview', '', 'pi pi-chart-bar');
    if (meta.frame) addPanel('frame', 'Frame', meta.frame + '', 'pi pi-list');
    if (meta.conversation) addPanel('tcp', 'TCP', meta.conversation + '', 'pi pi-server');
    if (meta.dns) addPanel('dns', 'DNS', meta.dns + '', 'pi pi-address-book');
    return mitems;
  };
  const buildPage = (): ReactElement => {
    switch (select) {
      case 'frame':
        return <FrameList />;
      case 'tcp':
        return <TCPList items={conversations}/>
      case 'dns':
        return <DNSList items={dnsRecords}/>
      case 'http':
        return <DNSList items={dnsRecords}/>
    }
    return <Overview/>;
  };
  if (status == 0) {
    return <HttpComponnet />
  }
  if (status == 2) {
    return <ErrPage />
  }
  const navItems = convert();
  const items = [{ label: select }];
    const home = { icon: 'pi pi-home' }
  return (<>
    <div className="card h-full">
      {/* <BreadCrumb model={items} home={home} /> */}
      <div className="flex flex-row h-full">
        <div className="w-full flex flex-grow-1">
          {buildPage()}
        </div>
        <div className="flex flex-column flex-grow-0 flex-shrink-0" style={{width: '10vw'}}>
          <Menu model={navItems} className="w-full h-full" />
        </div>
      </div>
    </div>
  </>
  );
}

export default Main;