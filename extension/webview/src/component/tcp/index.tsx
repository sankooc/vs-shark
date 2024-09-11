import React, { useEffect } from "react";
import { emitMessage } from "../../connect";
import { ComMessage, IConversation } from "../../common";
import DTable from '../dataTable2';
class Proto {
  items: IConversation[]
}
const TCPList = (props: Proto) => {
  const mountHook = () => {
    emitMessage(new ComMessage('conversation', null));
  };
  useEffect(mountHook, []);
  const items = props.items;
  const columes = [
    { field: 'source', header: 'source' },
    { field: 'dest', header: 'dest' },
    { field: 'count', header: 'count', style: { width: '7%' } },
    { field: 'throughput', header: 'throughput', style: { width: '7%' } }
  ];
  return (<div className="flex flex-nowrap h-full w-full">
    <DTable cols={columes} result={{ items: items, page: 1, size: items.length, total: items.length }} />
  </div>
  );
}

export default TCPList;