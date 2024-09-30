import React, { ReactElement, useEffect } from "react";
import { ComMessage, IDNSRecord } from "../../common";
import { emitMessage } from '../../connect';
import { TreeTable } from 'primereact/treetable';
import { Column, ColumnProps } from 'primereact/column';
class Proto {
  items: IDNSRecord[]
}
const DNSList = (props: Proto) => {
  const mountHook = () => {
    emitMessage(new ComMessage('dns', null));
  };
  useEffect(mountHook, []);
  const columes = [
    { field: 'name', header: 'name' },
    { field: '_type', header: 'type' },
    { field: 'class', header: 'clz' },
    { field: 'ttl', header: 'ttl' },
    { field: 'content', header: 'address' }
  ];
  const mapper = {};
  for(const item of props.items){
    const { name, _type } = item;
    const key = `${name}-${_type}`;
    mapper[key] = mapper[key] || [];
    mapper[key].push(item);
  }
  const values = [];
  let count = 1;
  for(const _its of Object.values(mapper)) {
    const its = _its as IDNSRecord[];
    const first = its.shift();
    const it: any = { key: count + "", data: first };
    if(its.length){
      it.children = its.map((f, inx) => ({ data: f, key: `${count}_${inx}` }));
    }
    values.push(it);
    count += 1;
  }
  return (<div className="flex flex-nowrap h-full w-full" id="dns-page">
    <TreeTable value={values} >
        {columes.map((c: ColumnProps, inx: number): ReactElement => {
          return (<Column {...c} key={'col' + inx} expander={inx===0}></Column>)
        })}
    </TreeTable>
  </div>
  );
};

export default DNSList;
