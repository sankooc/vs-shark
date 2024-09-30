import React, { useEffect, useState } from "react";
import ReactECharts from 'echarts-for-react';
import { emitMessage, onMessage } from '../../connect';
import { TabView, TabPanel } from 'primereact/tabview';
import './index.css';
import { ComMessage, IContextInfo, ILines, IOverviewData, IStatistic } from "../../common";
import Frame from './frame';
import Head from './head';
import Http from './http';

class Props {
  framedata: ILines;
  metadata: IContextInfo;
  httpdata: IStatistic;
}
function Overview(props: Props) {
  // const [framedata, setData] = useState<ILines>({ x: [], y: [], data: [] });
  const mountHook = () => {
    // const remv = onMessage('message', (e: any) => {
    //   const { type, body, requestId } = e.data;
    //   switch (type) {
    //     case '_overview': {
    //       setData(body);
    //       break;
    //     }
    //   }
    // });
    emitMessage(new ComMessage('overview', null));
    // setData(overview_json);
    // return remv;
  };
  useEffect(mountHook, []);

  return (<div className="w-full overview">
    {props.metadata && <Head data={props.metadata}/>}
    {props.framedata && <Frame data={props.framedata}/>}
    {props.framedata && <Http data={props.httpdata}/>}
  </div>);
}

export default Overview;