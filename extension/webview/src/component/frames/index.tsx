import React, { useEffect, useState, useRef } from "react";
import { CField, ComMessage, HexV, IResult, deserialize } from "../../common";
import {emitMessage, onMessage} from '../../connect';
import DTable from '../dataTable2';
import Stack from '../tree';
import HexView from '../detail';
import { IField, IFrameInfo, IListResult } from "../../gen";


const PAGE_SIZE = 500;
function FrameList() {
  const [filter, setFilter] = useState<any[]>([]);
  const [options, setOptions] = useState<string[]>([]);
  const [{items, start, total}, setItems] = useState<IListResult<IFrameInfo>>({items: [], total: 1, start: 0 });
  const [stacks, setStack] = useState<IField[]>([]);
  const [index, setIndex] = useState(0);
  const [hex, setHex] = useState<HexV>(null);
  const ref = useRef(null);

  const page = Math.floor(start / PAGE_SIZE) + 1;
  const size = PAGE_SIZE;
  const mountHook = () => {
    const remv = onMessage('message', (e: any) => {
      const { type, body, requestId } = e.data;
      switch (type) {
        case '_frame': {
          setItems(deserialize(body));
          break;
        }
        case '_fields': {
          setStack(deserialize(body));
          break
        }
        case '_hex': {
          setHex(body);
          break
        }
        case '_protocols': {
          setOptions(body);
          break
        }
      }
    });
    emitMessage(new ComMessage('protocols', null));
    emitMessage(new ComMessage('frame', {page:1, size: PAGE_SIZE, filter}));
    return remv;
  };
  useEffect(mountHook, []);
  const columes = [
    { field: 'index', header: 'index', style: { width: '4rem' } },
    { field: 'time', header: 'micro sec', style: { width: '7rem' } },
    { field: 'source', header: 'source', style: { width: '17.5rem' }},
    { field: 'dest', header: 'dest', style: { width: '17.5rem' }},
    { field: 'protocol', header: 'protocol', style: { width: '5.5rem' } },
    { field: 'len', header: 'len', style: { width: '5.5rem' } },
    { field: 'info', header: 'info' }
  ];
  const onSelect = (item: any): void => {
    setIndex(item.index);
    emitMessage(new ComMessage('fields', item.index - 1));
    setHex(new HexV(new Uint8Array()));
  };
  const getStyle = (item) => {
    switch(item.status){
      case 'deactive':
        return item.status
      case 'errordata':
          return 'errdata';
      default: {
        return (item.protocol || '').toLowerCase();

      }
    }
  }
  const onStackSelect = (index, key, _f) => {
    emitMessage(new ComMessage('hex',{index: index - 1, key}));
  };
  const request = (event) => {
    const {rows, page} = event;
    const _filter = filter.map(f => f.code).join('&');
    emitMessage(new ComMessage('frame', {page: page + 1, size: rows, filter: _filter}));
  }
  const extraFilter = {
    options,
    value: filter,
    onChange: (e) => {
      setFilter(e.value);
      const _filter =  e.value.map(f => f.code).join('&');
      emitMessage(new ComMessage('frame', {page:1, size: PAGE_SIZE, filter: _filter}));
    }
  }
  return (<div className="flex flex-nowrap flex-column h-full w-full" id="frame-page">
    <div className="editor flex-shrink-0" style={{height: '70vh', overflow: "hidden", "borderBottom":"1px solid var(--vscode-list-focusBackground)"}}>
      <DTable key={page} filter={extraFilter} cols={columes} result={{items, page, size, total}} getStyle={getStyle} onSelect={onSelect} request={request} scrollHeight={70}/>
    </div>
    <div className="viewer flex-grow-1 flex flex-row">
      <div className="treemap h-full flex-shrink-0">
      <Stack key={'stack'+index} frame={index} items={stacks} onSelect={onStackSelect}/>
      </div>
      <div ref={ref} className="hexvewer">
        <HexView data={hex}/>
      </div>
    </div>
  </div>
  );
}

export default FrameList;