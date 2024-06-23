import React,{ useEffect, useState }  from "react";
import "./app.less";
import Main from './main';
import Loading from './loading';
import { emitMessage, onMessage } from '../connect';
import { ComMessage, Frame } from "../common";

export default function (){
  const [{loaded, status, items}, setLoad] = useState<any>({loaded: false, status: 'init', items: []});
  useEffect(() => {
    onMessage('message', (e: any) => {
      const { type, body, requestId } = e.data;
      switch (type) {
        case 'init':
          {
            if (body) {
              const { status } = body;
              setLoad({loaded: true, status});
              return;
            } else {
              return;
            }
          }
        case 'framelist':
          {
            const items = body as Frame[];
            setLoad({loaded: true, status: 'done', items});
          }
          break;
      }
    });
    emitMessage(new ComMessage('ready', 'demo'));
  }, []);
  if(loaded){
    return (<Main status={status} items={items||[]}/>)
  }
  return (<Loading/>)
  // return (<Main data={data}/>)
}