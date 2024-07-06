import React, { useEffect, useState } from "react";
import { emitMessage, trace } from "../../connect";
import { ColumnItem, ComMessage, Frame } from "../../common";
import { Splitter, SplitterPanel } from 'primereact/splitter';
import DTable from '../dataTable';
import Stack from '../tree';
import HexView from '../detail';
import { IconField } from 'primereact/iconfield';
import { InputIcon } from 'primereact/inputicon';
import { InputText } from 'primereact/inputtext';
import { MultiSelect } from 'primereact/multiselect';

class FrameListProps {
  items: Frame[];
}

function FrameList(props: FrameListProps) {
  const [filters, setFilter] = useState(null);
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
    { field: 'no', header: 'index', style: { width: '5%' } },
    { field: 'time_str', header: 'time' },
    { field: 'source', header: 'source' },
    { field: 'dest', header: 'dest' },
    { field: 'protocol', header: 'protocol', style: { width: '5%' } },
    { field: 'len', header: 'length', style: { width: '5%' } },
    { field: 'info', header: 'info' }
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
    emitMessage(new ComMessage('frame-select', { index: item.no }));
  };
  return (<div className="flex flex-nowrap h-full w-full" id="frame-page">
    <Splitter layout="vertical" className="h-full w-full">
      <SplitterPanel className="flex flex-column align-items-center justify-content-center" size={70}>
        <IconField iconPosition="left" className="w-full">

          <MultiSelect value={filters} onChange={(e) => { setFilter(e.value) }} options={protos} optionLabel="name"
            placeholder="Select Protocols" maxSelectedLabels={10} className="p-inputtext-sm w-2" />
          {/* <InputText placeholder="Search" className="p-inputtext-sm w-10"/> */}
        </IconField>
        <DTable cols={columes} items={items} onSelect={onSelect} />
      </SplitterPanel>
      <SplitterPanel className="flex align-items-center justify-content-center" size={30} minSize={20}>
        <Splitter className="w-full">
          <SplitterPanel className="flex align-items-center" size={50} minSize={50} style={{ height: '28vh', overflow: 'auto' }}>
            <Stack />
          </SplitterPanel>
          <SplitterPanel className="flex align-items-center" size={50} minSize={50} style={{ height: '28vh', overflow: 'auto' }}>
            <HexView />
          </SplitterPanel>
        </Splitter>

      </SplitterPanel>
    </Splitter>
  </div>
  );
}

export default FrameList;