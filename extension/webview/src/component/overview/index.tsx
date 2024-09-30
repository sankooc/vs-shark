import React, { useEffect } from "react";
import { emitMessage } from '../../connect';
import './index.css';
import { ComMessage, IContextInfo, ILines, IStatistic } from "../../common";
import Frame from './frame';
import Head from './head';
import Http from './http';

class Props {
  framedata: ILines;
  metadata: IContextInfo;
  httpdata: IStatistic;
}
function Overview(props: Props) {
  const mountHook = () => {
    emitMessage(new ComMessage('overview', null));
  };
  useEffect(mountHook, []);

  return (<div className="w-full overview">
    {props.metadata && <Head data={props.metadata}/>}
    {props.framedata && <Frame data={props.framedata}/>}
    {props.framedata && <Http data={props.httpdata}/>}
  </div>);
}

export default Overview;