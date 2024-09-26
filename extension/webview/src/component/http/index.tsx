import React, { useEffect } from "react";
import { ComMessage, IDNSRecord } from "../../common";
import DTable from '../dataTable2';
import { emitMessage } from '../../connect';
import ReactECharts from 'echarts-for-react';
import { Card } from 'primereact/card';
import { Panel } from 'primereact/panel';
import TypePie from './type';
import MethodPie from './method';
import StatusPie from './status';
import "./index.css";
import { BreadCrumb } from 'primereact/breadcrumb';
import bt from '../ui';

class Proto {
  // items: IDNSRecord[]
}
const props = {
  items: [{"req":{"host":"10.109.185.160","port":41243,"head":"GET /api/health HTTP/1.1","header":["Host: 10.107.148.92","User-Agent: Uptime-Kuma/1.23.2","Connection: close","Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9","Accept-Encoding: gzip"]},"res":{"host":"10.107.148.92","port":80,"head":"HTTP/1.1 200 ","header":["Server: nginx/1.22.1","Date: Thu, 19 Oct 2023 01:54:45 GMT","Content-Type: text/html;charset=ISO-8859-1","Content-Length: 2","Connection: close","Vary: Origin","Vary: Access-Control-Request-Method","Vary: Access-Control-Request-Headers"]}},{"req":{"host":"10.109.185.160","port":41246,"head":"GET /api/health HTTP/1.1","header":["Host: 10.107.148.92","User-Agent: Uptime-Kuma/1.23.2","Connection: close","Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9","Accept-Encoding: gzip"]},"res":{"host":"10.107.148.92","port":80,"head":"HTTP/1.1 200 ","header":["Server: nginx/1.22.1","Date: Thu, 19 Oct 2023 01:55:05 GMT","Content-Type: text/html;charset=ISO-8859-1","Content-Length: 2","Connection: close","Vary: Origin","Vary: Access-Control-Request-Method","Vary: Access-Control-Request-Headers"]}}],
  "statistic":{"http_method":[{"name":"GET","value":2},{"name":"POST","value":1}],"http_status":[{"name":"200","value":2}],"http_type":[{"name":"text/plain","value":1},{"name":"application/json","value":1},{"name":"text/html","value":2}]}};

  const createPanel = () => {
    return (<Panel className="" header="Header" toggleable>
            <code className="code-desc">
                Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
                Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
                consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. 
                Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
            </code>
    </Panel>)
  }

  const HttpComponnet = (_props: Proto) => {
  const statistic = props.statistic;
  return (<div className="flex flex-column h-full w-full" id="http-page">
    <Card className="head-card">
    <Card style={{width: '30%'}}>
        <TypePie items={statistic.http_method} title="HTTP Method Usage" tooltip="http method"/>
    </Card>
    <Card style={{width: '30%'}}>
        <TypePie items={statistic.http_status} title="Web Traffic Response Code Analysis" tooltip="status code"/>
    </Card>
    <Card style={{width: '30%'}}>
        <TypePie items={statistic.http_type} title="Content-Type Distribution" tooltip="resp type"/>
    </Card>
    </Card>
    <Panel className="" header="Header" toggleable>
            <code className="code-desc">
                Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
                Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
                consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. 
                Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
            </code>
    </Panel>
    
  </div>
  );
};

export default HttpComponnet;
