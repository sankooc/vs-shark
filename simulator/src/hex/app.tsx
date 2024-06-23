import React,{ useEffect, useState } from "react";
import { onMessage } from '../connect';
import { HexV } from "../common";

function HexView() {
  const [data, setData] = useState<HexV>();
  useEffect(() => {
    onMessage('message', (e: any) => {
      const { type, body, requestId } = e.data;
      switch (type) {
        case 'hex-data':
          const data = body as HexV;
          setData(data);
      }
    });
  }, []);
  const indexes = [];
  const codes = [];
  let text = '';
  if(data){
    const lent = data.data.length;
    for(let i = 0 ; i < lent; i += 16){
      const inx = `0x${i.toString(16).padStart(8, '0')}`;
      indexes.push(inx);
    }
    for(let i=0;i<lent;i += 1){
      codes.push(data.data[i].toString(16).padStart(2, '0'));
    }
    try {
      for(let i=0;i<lent;i += 1){
        text += String.fromCharCode(data.data[i])
      }
    }catch(e){}
  }
  return (<>
    <div className="index">
      {indexes.map(inx => <pre>{inx}</pre>)}
    </div>
    <div className="hex">
      {codes.map((code, inx) => <code>{code}</code>)}
    </div>
    <div className="text">
      <pre>{text}</pre>
    </div>
    </>
  );
}

export default HexView;