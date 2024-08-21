import React, { useEffect, useState } from "react";
import { emitMessage, trace } from "../../connect";
import { ColumnItem, ComMessage, Frame, TCPCol } from "../../common";
import { Splitter, SplitterPanel } from 'primereact/splitter';
import DTable from '../dataTable';
import Stack from '../tree';
import HexView from '../detail';
import { IDNSRecord, DNSProps } from "../../common";

const DNSList = (props: DNSProps) => {
    const getData = (): IDNSRecord[] => {
        return props.dnsRecords;
    };
    const items = getData();
    
    const columes = [
        { field: 'record.source', header: 'source'},
        { field: 'record.name', header: 'name' },
        { field: 'record.type', header: 'type' },
        { field: 'record.clz', header: 'clz'},
        { field: 'record.ttl', header: 'ttl'},
        { field: 'record.address', header: 'address' }
    ];
    return (<div className="flex flex-nowrap h-full w-full" id="frame-page">
        <DTable cols={columes} items={items} onSelect={() => {}} scrollHeight="95vh"/>
    </div>
    );
};

export default DNSList;
