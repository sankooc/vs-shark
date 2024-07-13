import React, { useEffect, useState } from "react";
import { emitMessage, trace } from "../../connect";
import { ColumnItem, ComMessage, Frame, HexV } from "../../common";
import { Splitter, SplitterPanel } from 'primereact/splitter';
import DTable from '../dataTable';
import Stack from '../tree';
import HexView from '../detail';
import { IconField } from 'primereact/iconfield';
import { InputIcon } from 'primereact/inputicon';
import { InputText } from 'primereact/inputtext';
import { MultiSelect } from 'primereact/multiselect';
import { Context, Packet } from "nshark";
import { IField } from "nshark/built/src/common";

class FrameListProps {
  items: Frame[];
  ctx: Context;
}

function FrameList(props: FrameListProps) {
  const [filters, setFilter] = useState(null);
  const [stacks, setStack] = useState<Packet[]>([]);
  const [hex, setHex] = useState<HexV>(null);
  const getData = (): Frame[] => {
    if(!filters?.length) return props.items;
    const maps = {};
    for(const f of filters){
      maps[f.code] = 1;
    }
    return props.items.filter((it: Frame) => { return !!maps[it.protocol]});
  };
  const items = getData();
  const columes = [
    { field: 'no', header: 'index', style: { width: '4%' } },
    { field: 'time_str', header: 'time', style: { width: '8%' } },
    { field: 'source', header: 'source', style: { width: '80px' }},
    { field: 'dest', header: 'dest', style: { width: '80px' }},
    { field: 'protocol', header: 'protocol', style: { width: '30px' } },
    { field: 'len', header: 'length', style: { width: '5%' } },
    { field: 'info', header: 'info', style: { width: '20vw' }  }
  ];
  const protos = [
  ];
  const _map = {};
  for(const f of props.items){
    _map[f.protocol] = _map[f.protocol] || 0;
    _map[f.protocol] = _map[f.protocol] += 1;
  }
  for (const code in _map) {
    protos.push({ name: `${code.toUpperCase()} (${_map[code]})`, code});
  }
  const onSelect = (item: ColumnItem): void => {
    const f = props.ctx.getFrames()[item.no - 1];
    const fs: Packet[] = [];
    let tmp = f;
    do{
      fs.unshift(tmp);
      tmp = tmp.parent;
    } while(tmp);
    setStack(fs);
  };
  return (<div className="flex flex-nowrap h-full w-full" id="frame-page">
    <Splitter layout="vertical" className="h-full w-full">
      <SplitterPanel className="flex flex-column align-items-center justify-content-center" size={70}>
        <IconField iconPosition="left" className="w-full">
          <MultiSelect value={filters} onChange={(e) => { setFilter(e.value) }} options={protos} optionLabel="name"
            placeholder="Select Protocols" maxSelectedLabels={10} className="p-inputtext-sm w-2" />
        </IconField>
        <DTable cols={columes} items={items} onSelect={onSelect} />
      </SplitterPanel>
      <SplitterPanel className="flex align-items-center justify-content-center" size={30} minSize={20}>
        <Splitter className="w-full">
          <SplitterPanel className="flex align-items-center" size={50} minSize={50} style={{ height: '28vh', overflow: 'auto' }}>
            <Stack items={stacks} ctx={props.ctx} onSelect={(f: IField) => {
              const data = f.getSource();
              const start = f.getStartIndex();
              const size = f.getSize();
              const h = new HexV(data);
              h.index = [start, size];
              setHex(h);
            }}/>
          </SplitterPanel>
          <SplitterPanel className="flex align-items-center" size={50} minSize={50} style={{ height: '28vh', overflow: 'auto' }}>
            <HexView data={hex}/>
          </SplitterPanel>
        </Splitter>
      </SplitterPanel>
    </Splitter>
  </div>
  );
}

export default FrameList;