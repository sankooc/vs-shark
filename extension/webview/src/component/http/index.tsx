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
const HttpComponnet = (props: Proto) => {
  const data = {
    methods: [{name: 'GET', value: 3},{name: 'PUT', value: 5},{name: 'RESP', value: 4}],
    status: [{name: '200', value: 13},{name: '404', value: 1}],
    types: [{name: 'application/json', value: 3},{name: 'image/png', value: 5},{name: 'text/html', value: 4}],
  };
  const title_1 = 'HTTP Method Usage';
  const title_2 = 'Web Traffic Response Code Analysis';
  const title_3 = 'Content-Type Distribution';

  return (<div className="flex flex-column h-full w-full" id="http-page">
    <Card className="head-card">
    <Card style={{width: '30%'}}>
        <TypePie items={data.methods} title={title_1}/>
    </Card>
    <Card style={{width: '30%'}}>
        <TypePie items={data.status} title={title_2}/>
    </Card>
    <Card style={{width: '30%'}}>
        <TypePie items={data.types} title={title_3}/>
    </Card>
    </Card>
    <Panel header="Header" toggleable>
            <p className="m-0">
                Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
                Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
                consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. 
                Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
            </p>
        </Panel>
    
  </div>
  );
};

export default HttpComponnet;
