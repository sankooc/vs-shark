import React, { ReactElement, SyntheticEvent, useEffect, useState } from "react";
import { MenuItem } from 'primereact/menuitem';
import { Badge } from 'primereact/badge';
import { Menu } from 'primereact/menu';
import { MainProps, ComMessage, IDNSRecord } from '../common';
import Loading from './loading';
import { onMessage, emitMessage } from '../connect';
import Overview from './overview';

import FrameList from './frames';
import TCPList from './tcp';
import ARPReplies from './arp';
import DNSList from './dns';
import { DNSRecord } from "nshark/built/src/common";

const itemRenderer = (item, options) => {
  return <a className="flex align-items-center px-3 py-2 cursor-pointer" onClick={options.onClick}>
    {item.icon && <span className={item.icon} />}
    <span className={`mx-2 ${item.items && 'font-semibold'}`}>{item.label}</span>
    {item.data && <Badge className="ml-auto" value={item.data} />}
  </a>
}; //pi-chart-bar
const Main = () => {
  const [select, setSelect] = useState('overview');
  const [data, setData] = useState<MainProps>(null);
  useEffect(() => {
    onMessage('message', (e: any) => {
      const { type, body, requestId } = e.data;
      switch (type) {
        case 'init':
          {
            if (body) {
              const { status } = body;
              // setLoad({loaded: true, status});
              return;
            } else {
              return;
            }
          }
        case 'data':
          {
            console.log(body);
            setData(body as MainProps);
            // setLoad({loaded: true, status: 'done'});

          }
          break;
      }
    });
    emitMessage(new ComMessage('ready', 'demo'));
  }, []);

  const convert = (props: MainProps): MenuItem[] => {
    const mitems: MenuItem[] = [];
    if(!props) return [];
    const addPanel = (id: string, label: string, extra: string, icon: string=''): void => {
      mitems.push({
        id, data: extra, template: itemRenderer, label, icon, className: select === id ? 'active' : '', command: (env) => {
          setSelect(env.item.id);
        }
      });
    };
    addPanel('overview', 'Overview', '', 'pi pi-chart-bar');
    if(props.items?.length) addPanel('frame', 'Frame', props.items.length + '', 'pi pi-list');
    if(props.tcps?.length) addPanel('tcp', 'TCP', props.tcps.length + '', 'pi pi-server');
    if(props.arpGraph?.nodes?.length) addPanel('arp', 'ARP', props.arpGraph?.nodes?.length + '', 'pi pi-chart-pie');
    if(props.dnsRecords?.length) addPanel('dns', 'DNS',props.dnsRecords?.length + '', 'pi pi-address-book');
    return mitems;
  };

  // case 'arp':
  //   return <ARPReplies graph={props.arpGraph} legends={['sender', 'target']} />
  // case 'dns':
  //   return <DNSList items={props.dnsRecords} />
  const buildPage = (): ReactElement => {
    switch(select){
      case 'frame':
        return <FrameList items = {data.items}/>;
      case 'tcp':
        return <TCPList items={data.tcps} />
        
      case 'arp':
        return <ARPReplies graph={data.arpGraph} legends={['sender', 'target']} />
        case 'dns':
          return <DNSList items={data.dnsRecords.map((record:DNSRecord, inx: number) => {
            const r = new IDNSRecord(record);
            r.no = inx + 1;
            return r;
          })} />
    }
    return <Overview data={data.overview} />;
  };
  if (!data || !data.items?.length) {
    return <Loading />
  }

  const navItems = convert(data);
  return (<>
    <div className="card h-full">
      <div className="flex flex-column md:flex-row h-full">
        <div className="w-2 flex flex-column">
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