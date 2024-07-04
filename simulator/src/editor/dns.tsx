import React, { useState } from "react";
import Table from 'react-bootstrap/Table';
import { emitMessage, trace } from "../connect";
import {  DNSRecord } from "../client";

class FrameListProps {
  items: DNSRecord[];
}


const parseDataSize = (num: number): string => {
  return num + 'bytes';
};
function DNSList(props: FrameListProps) {
  const getData = (): DNSRecord[] => {
    return props.items;
  };
  const [store, setStore] = useState<any>({ current: 1, pageSize: 100 });
  const [select, setSelect] = useState(0);
  const items = getData();
  const total = items.length;
  const startFrom = (store.current - 1) * store.pageSize;
  const _items = items.slice(startFrom, Math.min(startFrom + store.pageSize, total));
  const maxPage = Math.floor(total / store.pageSize) + 1;
  const columes = ['source', 'name', 'type', 'class', 'ttl', 'address'];
  const cols = _items;
  return (<div className="d-flex flex-nowrap" id="frame-page">
    <div className="main-content">
      <Table bordered hover size={'sm'}>
        <thead>
          <tr>
            {columes.map(c => <th>{c}</th>)}
          </tr>
        </thead>
        <tbody>
          {cols.map(item => <tr>
            <td className="time">{item.source}</td>
            <td className="time">{item.name}</td>
            <td className="time">{item.type}</td>
            <td className="time">{item.clz}</td>
            <td className="time">{item.ttl}</td>
            <td className="time">{item.address}</td>
            </tr>)}
        </tbody>
      </Table>

    </div>
    <nav className="main-pagination">
      <ul className="pagination">
        <li className="page-item"><a className={store.current > 1 ? 'page-link' : 'page-link disabled'} href="#" onClick={() => {
          const { current } = store;
          setStore({ ...store, current: current - 1 });
        }}>&lt;</a></li>
        <li className="page-item"><a className={store.current < maxPage ? 'page-link' : 'page-link disabled'} href="#" onClick={() => {
          setStore({ ...store, current: store.current + 1 });
        }}>&gt;</a></li>
      </ul>
    </nav>
  </div>
  );
}

export default DNSList;