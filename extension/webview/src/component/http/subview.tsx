import React, { useState } from "react";
import { IHttp, IHttpEnity } from "../../common";
import DTable, { Props } from '../dataTable';
import { Button } from 'primereact/button';
import { Dialog } from 'primereact/dialog';
import { Panel } from 'primereact/panel';
import { TabView, TabPanel } from 'primereact/tabview';
import Content from './view';

const SubComponnet = (props: Props) => {
  const [selection, setSelect] = useState<IHttp>(null);
  const [visible, setVisible] = useState<boolean>(false);
  const craeteContenView = (msg: IHttpEnity) => {
    const content = msg.content;
    const _content = Buffer.from(content).toString('base64');
    return <span className="base64-content">{_content}</span>;
  }
  const createMessage = (msg: IHttpEnity) => {
    return (
      <Panel header={msg.head} className={`http-panel`} toggleable>
        <TabView className="w-full">
          <TabPanel header="headers">
            <div className="code-block">
              {msg.header.map((l, inx) => (<code className="code-line" key={"line" + inx}>{l}</code>))}
            </div>
          </TabPanel>
          {msg.content_len > 0 ? <TabPanel header="body">
            <Content message={msg}/>
          </TabPanel>: null}
          
        </TabView>
      </Panel>
    )
  }
  const header = () => {
    if (!selection) {
      return "";
    }
    const { req, res, method } = selection;
    return `${req.host}:${req.port} -> ${res.host}:${res.port} [${method}]`;
  }
  const createPanel = () => {
    if (!selection) {
      return <></>
    }
    const { req, res } = selection;
    return (<>
      {createMessage(req)}
      {createMessage(res)}
    </>)
  }
  const disabled = !selection;
  const onSelect = setSelect;
  const footer = <div className="card flex flex-nowrap gap-3 p-fluid">
    <div className="flex align-items-right gap-3">
      <Button disabled={disabled} onClick={() => { setVisible(true) }} label="Detail" icon="pi pi-search" size="small" />
    </div>
  </div>
  return (<>
    <Dialog className="http-dialog" header={header()} visible={visible} style={{ width: '70vw' }} onHide={() => { if (!visible) return; setVisible(false); }}>
      {createPanel()}
    </Dialog>
    <DTable {...props} onSelect={onSelect} footer={footer}></DTable>
  </>
  );
};

export default SubComponnet;
