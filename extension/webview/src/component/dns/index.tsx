import React, { useEffect } from "react";
import { ComMessage, IDNSRecord } from "../../common";
import DTable from '../dataTable2';
import { emitMessage } from '../../connect';
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
  return (<div className="flex flex-nowrap h-full w-full" id="frame-page">
    <DTable cols={columes} result={{ items: props.items, page: 1, size: props.items.length, total: props.items.length }} />
  </div>
  );
};

export default DNSList;
