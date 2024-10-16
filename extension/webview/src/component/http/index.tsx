import React, { useEffect, useState } from "react";
import "./index.css";
import { ComMessage } from "../../common";
import { emitMessage } from "../../connect";
import SubComponnet from './subview';
import { IConnect, IHttpMessage } from "../../gen";
import { TreeNode } from "primereact/treenode";

class Proto {
  items: IConnect<IHttpMessage>[];
}
const getHeaderValue = (header, key): string => {
  if(!header || !key){
    return '';
  }
  for(const head of header){
    let toks = head.split(':');
    if(toks && toks.length > 1){
      const [hk, vl] = toks;
      if(hk.toLowerCase() == key.toLowerCase()){
        const [v] = vl.split(';');
        return v.trim();
      }
    }
  }
  return '';
};
const HttpComponnet = (props: Proto) => {
  const mountHook = () => {
    emitMessage(new ComMessage('http', null));
  };
  useEffect(mountHook, []);

  
  const columes = [
    {
    field: 'index',
    bodyClassName: 'tree-head',
    header: 'no', style: { width: '7vw' }, expander: true},
    { 
      body: (item) => { 
      if(!!item.children) {
        return item.data.source
      }
      if(item.data.mes == 'request') return <i className="pi pi-cloud-upload"> Request</i>;
      return <i className="pi pi-cloud-download"> Response</i>;
    },
      header: 'source', style: { width: '15rem' }},
    { field: 'target', header: 'target', style: { width: '15rem' }},
    { field: 'path', header: 'path/status', style: { width: '30vw' }},
    // { field: 'method', header: 'method/code' },
    // { field: 'path', header: 'path/status'},
    { field: 'type', header: 'type'},
    { field: 'size', header: 'size'},
    // { field: 'ts', header: 'timestamp'},
    // { body: (item: IConnect<IHttpMessage>) => <span>{item.list.length}</span>, header: 'length'},
  ];

  // const items = fetchItems();
  const items = props.items;
  const _tstring = (ts: number) => {
    const d = new Date(ts);
    // return `${d.getMonth()}/${d.getDate()} ${d.getHours()}:${d.getMinutes()}:${d.getSeconds()} ${d.getMilliseconds()}`;
    return `${d.getHours()}:${d.getMinutes()}:${d.getSeconds()} ${d.getMilliseconds()}`;
  }
  const itemMapper = (item: IConnect<IHttpMessage>): TreeNode => {
    const _items = (item.list || []).sort((a, b) => (a.ts - b.ts));
    let _res_count = 0;
    let _req_count = 0;
    let __type = '';
    let total = 0;
    const messages = _items.map((msg) => {
      const { method, headers } = msg;
      const isResp = method.match(/^\d+$/);
      isResp ? _res_count ++ : _req_count ++;
      const _type = getHeaderValue(headers, "content-type");
      __type = __type || _type;
      const mes = isResp? 'response' :'request';
      total += msg.len;
      return { key: `${msg.ts}`, data: {
        mes,
        source: '', path: msg.path, target: method, 
        type: _type,
        size: msg.len +' Bytes',
        index: _tstring(msg.ts),
        data: {
          msg,
          param: [item.index, msg.ts],
        }
      }}
    });
    const rs = {
      key: item.index,
      data: { 
        index: item.index, 
        source: item.source, 
        target: item.target, 
        path: `req: ${_req_count} / res: ${_res_count}`,
        type: __type,
        size: `${total} Bytes`
      },
      children: messages,
    };
    return rs;
  }
  return (<div className="flex flex-column h-full w-full" id="http-page">
    <SubComponnet cols={columes} items={items.map(itemMapper)}/>
  </div>
  );
};

export default HttpComponnet;
