import React,{ useEffect, useState }  from "react";
import "./app.css";
import Main from './main';
import Loading from './loading';
import { emitMessage, onMessage } from '../connect';
import { ComMessage, Frame, Grap, TCPCol, MainProps } from "../common";

export default function (){

  const [{loaded, status, items}, setLoad] = useState<any>({loaded: false, status: 'init'});
  const [ mProps, setProps] = useState<MainProps>();
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
        case 'data':
          {
            setProps(body as MainProps);
            setLoad({loaded: true, status: 'done'});

          }
          break;
      }
    });
    emitMessage(new ComMessage('ready', 'demo'));
  }, []);
  if(mProps){
    return (<Main {...mProps}/>)
  }
  return (<Loading/>)
  // return (<Main data={data}/>)
}