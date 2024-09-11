import React, { useEffect, useState } from "react";
import { ComMessage, IConversation } from "../../common";
// import DTable from '../vsTable';
import DTable from '../dataTable2';
import {emitMessage, onMessage} from '../../connect';

const DNSList = () => {
    const [items, setItems] = useState<IConversation[]>([]);
    const mountHook = () => {
        const remv = onMessage('message', (e: any) => {
          const { type, body, requestId } = e.data;
          switch (type) {
            case '_dns': {
                setItems(body);
              break;
            }
          }
        });
        emitMessage(new ComMessage('dns', null));
        return remv;
      };
      useEffect(mountHook, []);
    const columes = [
        { field: 'name', header: 'name' },
        { field: '_type', header: 'type' },
        { field: 'class', header: 'clz'},
        { field: 'ttl', header: 'ttl'},
        { field: 'content', header: 'address' }
    ];
    return (<div className="flex flex-nowrap h-full w-full" id="frame-page">
        <DTable cols={columes} result={{items: items, page: 1, size: items.length, total: items.length}} />
    </div>
    );
};

export default DNSList;
