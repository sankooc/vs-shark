import React,{ useEffect, useState }  from "react";
import "./app.less";
import Main from './main';
import Loading from './loading';
import * as vs from "vscode-webview"

export default function (){
  const [data, setData] = useState<Uint8Array>();
  useEffect(() => {
    console.log('init');
    // if(!!acquireVsCodeApi){
      if(window['acquireVsCodeApi']){
      const vscode = acquireVsCodeApi();
      vscode.postMessage({ type: 'ready' });
    }
    window.addEventListener('message', (e: any) => {
      const { type, body, requestId } = e.data;
      switch (type) {
        case 'init':
          {
            if (body) {
              setData(body as Uint8Array);
              return;
            } else {
              return;
            }
          }
      }
    });
  }, []);
  if(data){
    return (<Main data={data}/>)
  }
  return (<Loading/>)
  // return (<Main data={data}/>)
}