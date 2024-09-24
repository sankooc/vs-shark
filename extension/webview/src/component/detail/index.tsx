import React, { useEffect, useState, useRef } from "react";
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
  const ref = useRef(null);
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
      let _indx = 0;
      while(_indx <= raw.length){
        const _fin = Math.min(_indx + 16, raw.length);
        if(end > start){
          const _start = Math.max(start, _indx);
          const _end = Math.min(end, _fin);
          if(start > _fin || end < _indx) {
            texts.push(<div className="asc" key={"pre-" + _indx}>{to_string(raw.slice(_indx, _fin))}</div>);
          } else if (_start < _fin) {
            texts.push(<div className="asc" key={"pre-" + _indx}>
              {to_string(raw.slice(_indx, _start))}
              <pre>{to_string(raw.slice(_start, _end))}</pre>
              {to_string(raw.slice(_end, _fin))}
              </div>);
          }
        } else {
          texts.push(<div className="asc" key={"pre-" + _indx}>{to_string(raw.slice(_indx, _fin))}</div>);
        }
        _indx = _fin;
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
    <div className="text">
      {texts}
    </div>
  </div>
  );
}

export default HexView;