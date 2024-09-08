import React, { useEffect, useState } from "react";
import { ColumnItem, HexV } from "../../common";
import { MainProto } from '../../wasm';
import { Splitter, SplitterPanel, SplitterResizeEndEvent } from 'primereact/splitter';
import DTable from '../dataTable';
import Stack from '../tree';
import HexView from '../detail';
import { IconField } from 'primereact/iconfield';
import { MultiSelect } from 'primereact/multiselect';
import { WContext, FrameInfo, Field } from 'rshark';

class FrameListProps {
  items: FrameInfo[];
  ctx: WContext;
}
export class FrameItem implements ColumnItem {
  no!: number;
  time!: number;
  time_str?: string;
  source: string = 'n/a';
  dest: string = 'n/a';
  protocol!: string;
  iRtt: number = 0;
  len: number = 0;
  style: string='';
  info!: string;
  constructor(info: FrameInfo){
    this.no = info.index;
    this.time = info.time;
    this.source = info.source;
    this.dest = info.dest;
    this.protocol = info.protocol;
    this.len = info.len;
    this.iRtt = info.irtt;
    if(info.status == 'info'){
      this.style = info.protocol?.toLowerCase();
    } else {
      this.style = info.status;
    }
    this.info = info.info;
  }
  public getIndex(): number {
      return this.no;
  }
  public getStyle(inx: number): string {
      if(this.no === inx){
          return 'active';
      }
      return this.style;
  }

}
function FrameList(props: MainProto) {
  const [filters, setFilter] = useState(null);
  const [stacks, setStack] = useState<Field[]>([]);
  const [index, setIndex] = useState(0);
  const [hex, setHex] = useState<HexV>(null);
  const [[up, down], setUp] = useState([70, 30]);
  const frames = props.instance.getFrames().map((item) => new FrameItem(item));
  const getData = (): ColumnItem[] => {
    if(!filters || !filters.length) return frames;
    const maps = {};
    for(const f of filters){
      maps[f.code] = 1;
    }
    return frames.filter((it: FrameItem) => { return !!maps[it.protocol]});
  };
  const items = getData();
  const columes = [
    { field: 'no', header: 'index', style: { width: '4%' } },
    { field: 'time', header: 'time', style: { width: '8%' } },
    { field: 'source', header: 'source', style: { width: '15%' }},
    { field: 'dest', header: 'dest', style: { width: '15%' }},
    { field: 'protocol', header: 'protocol', style: { width: '5%' } },
    { field: 'len', header: 'len', style: { width: '5%' } },
    { field: 'info', header: 'info', style: { width: '20vw' }  }
  ];
  const protos = [
  ];
  const _map = {};
  for(const f of frames){
    _map[f.protocol] = _map[f.protocol] || 0;
    _map[f.protocol] = _map[f.protocol] += 1;
  }
  for (const code in _map) {
    protos.push({ name: `${code.toUpperCase()} (${_map[code]})`, code});
  }
  const onSelect = (item: ColumnItem): void => {
    setStack(props.instance.ctx.get_fields(item.no - 1));
    setIndex(item.no);
    setHex(null);
  };
  const onStackSelect = (f) => {
    const { data, start, size } = f;
    const h = new HexV(data);
    h.index = [start, size];
    setHex(h);
  };
  const scrollHeight = `calc(${up}vh - 40px)`;
  const upSplice = (evt: SplitterResizeEndEvent) => {
    const { sizes } = evt; 
    const [ up, down ] = sizes;
    setUp([up, down]);
  }
  return (<div className="flex flex-nowrap h-full w-full" id="frame-page">
    <Splitter layout="vertical" className="h-full w-full" onResizeEnd={upSplice}>
      <SplitterPanel className="flex flex-column align-items-center justify-content-center" size={up} minSize={50} style={{overflow: "auto"}}>
        <IconField iconPosition="left" className="w-full">
          <MultiSelect value={filters} onChange={(e) => { setFilter(e.value) }} options={protos} optionLabel="name"
            placeholder="Select Protocols" maxSelectedLabels={10} className="p-inputtext-sm w-2" />
        </IconField>
        <DTable cols={columes} scrollHeight={scrollHeight} items={items} onSelect={onSelect} />
      </SplitterPanel>
      <SplitterPanel className="flex align-items-center justify-content-center" minSize={30} style={{maxHeight: `${down}vh`}}>
        <Splitter className="w-full" style={{height: `100%`}}>
          <SplitterPanel className="flex align-items-center" size={50} minSize={50} style={{overflow: 'auto' }}>
            <Stack frame={index} items={stacks} onSelect={onStackSelect}/>
          </SplitterPanel>
          <SplitterPanel className="flex align-items-center" size={50} minSize={50} style={{overflow: 'auto' }}>
            <HexView data={hex}/>
          </SplitterPanel>
        </Splitter>
      </SplitterPanel>
    </Splitter>
  </div>
  );
}

export default FrameList;