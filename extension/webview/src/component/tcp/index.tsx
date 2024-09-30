import React, { useEffect } from "react";
import { emitMessage } from "../../connect";
import { ComMessage, IConversation } from "../../common";
import DTable from '../dataTable';
class Proto {
  items: IConversation[]
}

const append = (ip, port, host) => {
  if(host){
    return `${ip}:${port} (${host})`;
  }
  return `${ip}:${port}`; 
}
const TCPList = (props: Proto) => {
  const mountHook = () => {
    emitMessage(new ComMessage('conversation', null));
  };
  useEffect(mountHook, []);
  const items = props.items;
  const columes = [
    { header: 'source', body: (data) => <span>{append(data.source_ip, data.source_port, data.source_host)}</span> },
    { header: 'target', body: (data) => <span>{append(data.target_ip, data.target_port, data.target_host)}</span> },
    { sortable: true, field: 'count', header: 'count', style: { width: '7%' } },
    { sortable: true, field: 'throughput', header: 'throughput', style: { width: '7%' } }
  ];
  const result = {
    items: items.map((it, inx) => ({ index: (inx + 1), ...it })), 
    page: 1, 
    size: items.length, 
    total: items.length
  };
  return (<div className="flex flex-nowrap h-full w-full">
    <DTable cols={columes} result={result} />
  </div>
  );
}

export default TCPList;