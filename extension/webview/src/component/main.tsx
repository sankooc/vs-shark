import React, { ReactElement, useEffect, useState } from "react";
import { MenuItem } from 'primereact/menuitem';
import { Badge } from 'primereact/badge';
import { Menu } from 'primereact/menu';
import { ComMessage, IContextInfo } from '../common';
import Loading from './loading';
import { onMessage, log, emitMessage } from '../connect';

import Overview from './overview';
import FrameList from './frames';
import TCPList from './tcp';
import DNSList from './dns';


const itemRenderer = (item, options) => {
  return <a className="flex align-items-center px-3 py-2 cursor-pointer" onClick={options.onClick}>
    {item.icon && <span className={item.icon} />}
    <span className={`mx-2 ${item.items && 'font-semibold'}`}>{item.label}</span>
    {item.data && <Badge className="ml-auto" value={item.data} />}
  </a>
}; //pi-chart-bar

let _start = 0;
const Main = () => {
  const [select, setSelect] = useState('overview');
  const [loading, setLoading] = useState<boolean>(true);
  const [meta, setMeta] = useState<IContextInfo>(null);
  useEffect(() => {
    onMessage('message', (e: any) => {
      const { type, body, requestId } = e.data;
      switch (type) {
        case '_info': {
          setMeta(body);
          setLoading(false);
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
        return <TCPList/>
      case 'dns':
        return <DNSList/>
    }
    return <Overview/>;
  };
  if (loading) {
    return <Loading />
  }

  const navItems = convert();
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