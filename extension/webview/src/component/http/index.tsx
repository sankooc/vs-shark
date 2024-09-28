import React, { useEffect } from "react";
import { Card } from 'primereact/card';
import { Panel } from 'primereact/panel';
import TypePie from './type';
import "./index.css";
import { ComMessage, IHttp, IHttpEnity, IStatistic } from "../../common";
import { emitMessage } from "../../connect";

class Proto {
  items: IHttp[];
  statistic: IStatistic;
}
const createMessage = (msg: IHttpEnity) => {
  return (<>
  <div className="code-block">
      <code className="code-line">
        {msg.head}
      </code>
    </div>
    <br />
    <div className="code-block">
      {msg.header.map((l, inx) => (<code className="code-line" key={"line" + inx}>{l}</code>))}
    </div>
  </>)
}
const createPanel = (item: IHttp, index: number) => {
  const { req, res, method, status } = item;
  const header = `${req.host}:${req.port} -> ${res.host}:${res.port} [${method}]`;
  return (<Panel className={`http-panel http-status-${status} http-method-${method}`} header={header} collapsed={index !== 0} toggleable key={"http-panel" + index}>
    {createMessage(req)}
    <br />
    <br />
    {createMessage(res)}
  </Panel>)
}

const HttpComponnet = (props: Proto) => {
  const mountHook = () => {
    emitMessage(new ComMessage('http', null));
  };
  useEffect(mountHook, []);
  const statistic = props.statistic;
  statistic.http_type.forEach((cs) => {
    cs.name = cs.name.replace(/application./, '');
  })
  return (<div className="flex flex-column h-full w-full" id="http-page">
    <Card className="http-statistic-card">
      <Card>
        <TypePie items={statistic.http_method} title="HTTP Method Usage" tooltip="http method" />
      </Card>
      <Card>
        <TypePie items={statistic.http_status} title="Web Traffic Response Code Analysis" tooltip="status code" />
      </Card>
      <Card>
        <TypePie items={statistic.http_type} title="Content-Type Distribution" tooltip="resp type" />
      </Card>
    </Card>
    {props.items.map(createPanel)}
  </div>
  );
};

export default HttpComponnet;
