import React, { useEffect, useState } from "react";
import { emitMessage, trace } from "../../connect";
import { ColumnItem, ComMessage, Frame, TCPCol } from "../../common";
import DTable from '../dataTable';
import { MainProto } from "../../wasm";
import { TCPConversation } from "rshark";

const TCPList = (props: MainProto) => {
    const getData = (): TCPCol[] => {
        return props.instance.getConversations().map((d, inx) => {
            const item = new TCPCol(d);
            item.no = inx + 1;
            return item;
        });
    };
    const items = getData();
    const columes = [
        { field: 'no', header: 'index', style: { width: '5%' } },
        { field: 'item.source', header: 'source' },
        { field: 'item.dest', header: 'dest' },
        { field: 'item.count', header: 'count', style: { width: '7%' } },
        { field: 'item.throughput', header: 'throughput', style: { width: '7%' }  }
    ];
    return (<div className="flex flex-nowrap h-full w-full" id="frame-page">
        <DTable cols={columes} items={items} onSelect={() => {}} scrollHeight="95vh"/>
    </div>
    );
}

export default TCPList;