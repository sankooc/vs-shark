import React, { useEffect, useState } from "react";
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
          console.log(data)
          setData(data);
      }
    });
  }, []);
  const indexes = [];
  const codes = [];
  let text = '';
  let start = 0;
  let end = 0;
  const getActive = (inx: number): string => {
    if(end > 0 && inx >= start && inx < end){
      return 'active';
    }
    return '';
  }
  if (data) {
    const lent = data.data.length;
    if(data.index && data.index[1]){
      start = data.index[0];
      end = start + data.index[1]
    }
    for (let i = 0; i < lent; i += 16) {
      const inx = `0x${i.toString(16).padStart(8, '0')}`;
      indexes.push(inx);
    }
    for (let i = 0; i < lent; i += 1) {
      codes.push(data.data[i].toString(16).padStart(2, '0'));
    }
    try {
      for (let i = 0; i < lent; i += 1) {
        const code = data.data[i];
        if (code > 33 && code !== 129 && code !== 141 && code !== 143 && code !== 144 && code !== 157) {
          text += String.fromCharCode(code)
        } else {
          text += '?'
        }
      }
    } catch (e) { }
  }
  return (<>
    <div className="index">
      {indexes.map(inx => <pre>{inx}</pre>)}
    </div>
    <div className="hex">
      {codes.map((code, inx) => <code className={getActive(inx)}>{code}</code>)}
    </div>
    <div className="text">
      <pre>{text}</pre>
    </div>
  </>
  );
}

export default HexView;