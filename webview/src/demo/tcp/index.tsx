import React, { useEffect, useState } from "react";
import { emitMessage, trace } from "../../connect";
import { ColumnItem, ComMessage, Frame, TCPCol } from "../../common";
import { Splitter, SplitterPanel } from 'primereact/splitter';
import DTable from '../dataTable';
import Stack from '../tree';
import HexView from '../detail';

class ListProps {
    items: TCPCol[];
}

const TCPList = (props: ListProps) => {
    const getData = (): TCPCol[] => {
        return props.items;
    };
    const items = getData();
    const columes = [
        { field: 'no', header: 'index', style: { width: '2%' } },
        { field: 'ep1', header: 'ep1' },
        { field: 'ep2', header: 'ep2' },
        { field: 'total', header: 'total', style: { width: '7%' } },
        { field: 'tcp', header: 'tcp', style: { width: '7%' } },
        { field: 'tcpUse', header: 'tcpUse', style: { width: '7%' } },
        { field: 'count', header: 'count', style: { width: '7%' } },
        { field: 'countUse', header: 'countUse', style: { width: '7%' }  }
    ];
    return (<div className="flex flex-nowrap h-full w-full" id="frame-page">
        <DTable cols={columes} items={items} onSelect={() => {}} scrollHeight="95vh"/>
    </div>
    );
}

export default TCPList;