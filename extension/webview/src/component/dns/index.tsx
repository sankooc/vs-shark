import React, { useEffect, useState } from "react";
import { ColumnItem } from "../../common";
import DTable from '../dataTable';
import { DNSRecord, MainProto } from "../../wasm";

class IDNSRecord implements ColumnItem {
    style?: string;
    no?: number;
    record: DNSRecord
    getIndex(): number {
        return this.no;
    }
    getStyle(inx: number): string {
        return this.style
    }
    constructor(record: DNSRecord) {
        this.record = record;
        this.style = 'info';
    }
}

const DNSList = (props: MainProto) => {
    const getData = (): IDNSRecord[] => {
        const _items = props.instance.getDNSRecord();
        return _items.map((item: DNSRecord, index: number) => {
            const rs = new IDNSRecord(item);
            rs.no = index + 1;
            return rs;
        });
    };
    const items = getData();
    const columes = [
        // { field: 'record.source', header: 'source'},
        { field: 'record.name', header: 'name' },
        { field: 'record._type', header: 'type' },
        { field: 'record.class', header: 'clz'},
        { field: 'record.ttl', header: 'ttl'},
        { field: 'record.content', header: 'address' }
    ];
    return (<div className="flex flex-nowrap h-full w-full" id="frame-page">
        <DTable cols={columes} items={items} onSelect={() => {}} scrollHeight="95vh"/>
    </div>
    );
};

export default DNSList;
