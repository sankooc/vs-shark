import React, { ReactElement, useEffect, useState } from "react";
import { MenuItem } from 'primereact/menuitem';
import { Badge } from 'primereact/badge';
import { Menu } from 'primereact/menu';
import { ComMessage, IContextInfo, IConversation, IDNSRecord, ILines, IStatistic, ITLS } from '../common';
import Loading from './loading';
import ErrPage from './error';
import { onMessage, log, emitMessage } from '../connect';

import Overview from './overview';
import FrameList from './frames';
import TCPList from './tcp';
import DNSList from './dns';
import HttpComponnet from './http';
import TLSComponent from './tls';

// import overview_json from '../mock/overview2.json';
// import meta_json from '../mock/meta.json';
// import http_json from '../mock/stat.json';
// import mock_ip from '../mock/ip.json';
// import mock_iptype from '../mock/iptype.json';
// import _dnsRecords from '../mock/dns.json';
import _httpRecords from '../mock/http.json';
import { IConnect, IHttpMessage } from "../gen";
// import _tlsRecords from '../mock/tls.json';


const itemRenderer = (item, options) => {
  return <a className="flex align-items-center px-3 py-2 cursor-pointer" onClick={options.onClick}>
    {item.icon && <span className={item.icon} />}
    <span className={`mx-2 ${item.items && 'font-semibold'}`}>{item.label}</span>
    {item.data && <Badge className="ml-auto" value={item.data} />}
  </a>
};

// framedata: ILines;
// metadata: IContextInfo;
// httpdata: IStatistic;
const eventMapper = {};

let _start = 0;
const Main = () => {
  const [select, setSelect] = useState('overview');
  const [status, setStatus] = useState<number>(0);
  const [meta, setMeta] = useState<IContextInfo>(null);
  const [framedata, setFramedata] = useState<ILines>(null);
  const [httpdata, setHttpdata] = useState<IStatistic>(null);
  const [dnsRecords, setDnsRecords] = useState<IDNSRecord[]>([]);
  const [conversations, setConversations] = useState<IConversation[]>([]);
  const [https, setHttps] = useState<IConnect<IHttpMessage>[]>([]);
  const [tlsRecords, setTlsRecords] = useState<ITLS[]>([]);
  const deserialize = (str) => {
    return JSON.parse(str)
  }
  useEffect(() => {
    onMessage('message', (e: any) => {
      const { type, body, requestId } = e.data;
      switch (type) {
        case '_info': {
          setMeta(body);
          setStatus(1);
          break;
        }
        case '_http': {
          setHttps(deserialize(body));
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
        case '_tls': {
          setTlsRecords(body);
          break;
        }
        case '_conversation': {
          setConversations(body);
          break;
        }
        case '_frame_statistic': {
          setFramedata(body)
          break;
        }
        case '_http_statistic': {
          setHttpdata(body)
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
    if (meta.frame_count) addPanel('frame', 'Frame', meta.frame_count + '', 'pi pi-list');
    if (meta.tcp_count) addPanel('tcp', 'TCP', meta.tcp_count + '', 'pi pi-server');
    if (meta.dns_count) addPanel('dns', 'DNS', meta.dns_count + '', 'pi pi-address-book');
    if (meta.http_count) addPanel('http', 'Http', meta.http_count + '', 'pi pi-sort-alt');
    if (meta.tls_count) addPanel('tls', 'TLS', meta.tls_count + '', 'pi pi-lock');
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
        return <HttpComponnet items={https} />
      case 'tls':
        return <TLSComponent items={tlsRecords}/>
    }
    return <Overview framedata={framedata} metadata={meta} httpdata={httpdata} />;
  };
  if (status == 0) {
    return <Loading/>
    // return <TLSComponent items={tlsRecords}/>
    // return <HttpComponnet items={_httpRecords} />
    // return <DNSList items={_dnsRecords}/>
    // return <Overview framedata={overview_json} metadata={meta_json} httpdata={http_json.statistic} />
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