import React, { useEffect, useState } from "react";
import { HexV } from "../../common";
import { HexViewer } from 'react-hexviewer-ts';
import './app.css';


const ALL_EXCEPT_PRINTABLE_LATIN = /[^\x20-\x7f]/g
const CONTROL_CHARACTERS_ONLY = /[\x00-\x1f]/g
const ascii_escape = function (str) {
  return str.replace(ALL_EXCEPT_PRINTABLE_LATIN, ".")
}
const to_string = (data: Uint8Array): string => {
  let text = '';
  data.forEach(ch => text += String.fromCharCode(ch))
  return ascii_escape(text)
}

function HexView(props: { data?: HexV }) {
  const indexes = [];
  const codes = [];
  let start = 0;
  let end = 0;
  const getActive = (inx: number): string => {
    if (end > 0 && inx >= start && inx < end) {
      return 'active';
    }
    return '';
  }
  const data = props.data;
  let hasData = !!data?.data;
  const texts = [];
  if (data) {
    const lent = data.data.length;
    if (data.index && data.index[1]) {
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
    if (data.data) {
      const raw = data.data;
      if(end > start){
        texts.push(<pre key={"pre-1"}>{to_string(raw.slice(0, start))}</pre>);
        texts.push(<pre key={"pre-2"} className="active">{to_string(raw.slice(start, end))}</pre>);
        if(end < raw.length){
          texts.push(<pre key={"pre-3"}>{to_string(raw.slice(end))}</pre>);
        }
      } else {
        texts.push(<pre key="out">{to_string(data.data)}</pre>);
      }
    }
  }
  
  if (!hasData || !indexes.length) {
    return <div id="detail"></div>
  }
  return (<div id="detail">
    <div className="index">
      {indexes.map(inx => <pre key={'line' + inx}>{inx}</pre>)}
    </div>
    <div className="hex">
      {codes.map((code, inx) => <code key={'code' + inx} className={getActive(inx)}>{code}</code>)}
    </div>
    <div className="text flex-grow-1">
      {texts}
    </div>
  </div>
  );
}

export default HexView;