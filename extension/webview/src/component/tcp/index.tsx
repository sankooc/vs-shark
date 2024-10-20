import React, { useEffect } from "react";
import { emitMessage } from "../../connect";
import { ComMessage } from "../../common";
import DTable from '../dataTable';
import { ITCPConversation, IWEndpoint } from "../../gen";
class Proto {
  items: ITCPConversation[]
}

const append = (ip, port, host) => {
  if(host){
    return `${ip}:${port} (${host})`;
  }
  return `${ip}:${port}`; 
}
const inv = (ep: IWEndpoint) => {
  // return `${ep.invalid}/${ep.retransmission}/${ep.count}`;
  if (!ep){
    return '0%'
  }
  if (ep.count === 0) {
    return '100%';
  }
  return (100 * (ep.count - ep.invalid - ep.retransmission) / ep.count).toFixed(2) + '%'
}
const TCPList = (props: Proto) => {
  const mountHook = () => {
    emitMessage(new ComMessage('conversation', null));
  };
  useEffect(mountHook, []);
  const items = props.items;
  const sortable = items.length < 3000;
  const columes = [
    { header: 'source', body: (data: ITCPConversation) => <span>{append(data.source.ip, data.source.port, data.source.host)}</span>,style: { width: '30rem' }},
    { header: 'target', body: (data: ITCPConversation) => <span>{append(data.target.ip, data.target.port, data.target.host)}</span> ,style: { width: '30rem' }},
    { header: 's-accuracy', body: (data: ITCPConversation) => <span>{inv(data.source)}</span>  },
    { sortable, field: 'source.throughput', header: 's-throughput'  },
    { header: 't-accuracy', body: (data: ITCPConversation) => <span>{inv(data.target)}</span>  },
    { sortable, field: 'target.throughput', header: 't-throughput'  },
    { sortable, header: 'count', body: (data: ITCPConversation) => <span>{data.source.count + data.target.count}</span>  },
    { sortable, header: 'throughput', body: (data: ITCPConversation) => <span>{data.source.throughput + data.target.throughput}</span>  },
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