import React, { useEffect, useState, useRef } from "react";
import { CField, ComMessage, HexV, IResult } from "../../common";
import {emitMessage, onMessage} from '../../connect';
import DTable from '../dataTable2';
import Stack from '../tree';
import HexView from '../detail';


const PAGE_SIZE = 500;
function FrameList() {
  const [filter, setFilter] = useState<any[]>([]);
  const [options, setOptions] = useState<string[]>([]);
  const [{items, page, size, total}, setItems] = useState<IResult>({items: [], page: 1, size: 0, total: 0});
  const [stacks, setStack] = useState<CField[]>([]);
  const [index, setIndex] = useState(0);
  const [hex, setHex] = useState<HexV>(null);
  const ref = useRef(null);
  const mountHook = () => {
    const remv = onMessage('message', (e: any) => {
      const { type, body, requestId } = e.data;
      switch (type) {
        case '_frame': {
          setItems(body);
          break;
        }
        case '_fields': {
          setStack(body);
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
    { field: 'index', header: 'index', style: { width: '4%' } },
    { field: 'time', header: 'time', style: { width: '8%' } },
    { field: 'source', header: 'source', style: { width: '15%' }},
    { field: 'dest', header: 'dest', style: { width: '15%' }},
    { field: 'protocol', header: 'protocol', style: { width: '5%' } },
    { field: 'len', header: 'len', style: { width: '5%' } },
    { field: 'info', header: 'info', style: { width: '20vw' }  }
  ];
  const onSelect = (item: any): void => {
    setIndex(item.index);
    emitMessage(new ComMessage('fields', item.index - 1));
    setHex(new HexV(new Uint8Array()));
  };
  // if(ref?.current) {
  //   ref.current.scrollIntoView({
  //     behavior: "smooth",
  //     block: "start"
  //   })
  // }
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
    const _filter = filter.map(f => f.code);
    emitMessage(new ComMessage('frame', {page: page + 1, size: rows, filter: _filter}));
  }
  const extraFilter = {
    options,
    value: filter,
    onChange: (e) => {
      setFilter(e.value);
      const _filter =  e.value.map(f => f.code);
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