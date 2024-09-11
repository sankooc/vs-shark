import React, { useEffect, useState } from "react";
import { CField, ComMessage, HexV, IResult } from "../../common";
import {emitMessage, onMessage} from '../../connect';
import DTable from '../dataTable2';
import Stack from '../tree';
import HexView from '../detail';

function FrameList() {
  const [filters, setFilter] = useState(null);
  const [{items, page, size, total}, setItems] = useState<IResult>({items: [], page: 1, size: 0, total: 0});
  const [stacks, setStack] = useState<CField[]>([]);
  const [index, setIndex] = useState(0);
  const [hex, setHex] = useState<HexV>(null);
  // const [[up, down], setUp] = useState([70, 30]);
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
      }
    });
    emitMessage(new ComMessage('frame', {page:1, size: 500}));
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
  };
  const getStyle = (item) => {
    return (item.protocol || '').toLowerCase();
  }
  const onStackSelect = (index, key, _f) => {
    emitMessage(new ComMessage('hex',{index: index - 1, key}));
  };
  const request = (event) => {
    const {rows, page} = event;
    emitMessage(new ComMessage('frame', {page: page + 1, size: rows}));
  }
  return (<div className="flex flex-nowrap flex-column h-full w-full" id="frame-page">
    <div className="editor flex-shrink-0" style={{height: '70vh', overflow: "hidden", "borderBottom":"1px solid var(--vscode-list-focusBackground)"}}>
      <DTable key={page} cols={columes} result={{items, page, size, total}} getStyle={getStyle} onSelect={onSelect} request={request} scrollHeight={70}/>
    </div>
    <div className="viewer flex-grow-1 flex flex-row">
      <div className="treemap h-full flex-shrink-0">
      <Stack frame={index} items={stacks} onSelect={onStackSelect}/>
      </div>
      <div className="hexvewer">
        <HexView data={hex}/>
      </div>
    </div>
  </div>
  );
}

export default FrameList;