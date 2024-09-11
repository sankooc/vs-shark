import React, { useEffect, useState } from "react";
import { emitMessage,onMessage } from "../../connect";
import { ComMessage, IConversation } from "../../common";
import DTable from '../dataTable2';

const TCPList = () => {
    const [items, setItems] = useState<IConversation[]>([]);
    const mountHook = () => {
        const remv = onMessage('message', (e: any) => {
          const { type, body, requestId } = e.data;
          switch (type) {
            case '_conversation': {
                setItems(body);
              break;
            }
          }
        });
        emitMessage(new ComMessage('conversation', null));
        return remv;
      };
      useEffect(mountHook, []);
    const columes = [
        { field: 'source', header: 'source' },
        { field: 'dest', header: 'dest' },
        { field: 'count', header: 'count', style: { width: '7%' } },
        { field: 'throughput', header: 'throughput', style: { width: '7%' }  }
    ];
    return (<div className="flex flex-nowrap h-full w-full">
      <DTable cols={columes} result={{items: items, page: 1, size: items.length, total: items.length}} />
    </div>
    );
}

export default TCPList;