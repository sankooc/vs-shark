import React, { useEffect } from "react";
import { DataTable, DataTableSelectionSingleChangeEvent } from 'primereact/datatable';
import DTable from '../dataTable';
import { IVConversation } from "../../../share/gen";

// const tableBuild = () => {
  
// };

const TCPList = () => {
  
  const columes = [
    { header: 'source', body: (data: IVConversation) => <span>{append(data.source.ip, data.source.port, data.source.host)}</span>,style: { width: '30rem' }},
    { header: 'target', body: (data: IVConversation) => <span>{append(data.target.ip, data.target.port, data.target.host)}</span> ,style: { width: '30rem' }},
    { header: 's-accuracy', body: (data: IVConversation) => <span>{inv(data.source)}</span>  },
    { sortable, field: 'source.throughput', header: 's-throughput'  },
    { header: 't-accuracy', body: (data: IVConversation) => <span>{inv(data.target)}</span>  },
    { sortable, field: 'target.throughput', header: 't-throughput'  },
    { sortable, header: 'count', body: (data: IVConversation) => <span>{data.source.count + data.target.count}</span>  },
    { sortable, header: 'throughput', body: (data: IVConversation) => <span>{data.source.throughput + data.target.throughput}</span>  },
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