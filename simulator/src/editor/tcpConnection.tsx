import React, { useState } from "react";
import Table from 'react-bootstrap/Table';
import { emitMessage, trace } from "../connect";
import {  TCPCol } from "../common";





class FrameListProps {
  items: TCPCol[];
}


const parseDataSize = (num: number): string => {
  return num + 'bytes';
  // let n = num;
  // let str = '';
  // str += (n % 1024);
  // n = Math.floor(n / 1024)
  // if(n > 0){
  //   str = `${n % 1024}Kb${str}`;
  //   n = ~~(n / 1024)
  // }
  // if(n > 0){
  //   str = `${n % 1024}Mb${str}`;
  //   n = ~~(n / 1024)
  // }
  // if(n > 0){
  //   str = `${n}Gb${str}`;
  // }
  // return str;
};
function ConnectList(props: FrameListProps) {
  const getData = (): TCPCol[] => {
    return props.items;
  };
  const [store, setStore] = useState<any>({ current: 1, pageSize: 50 });
  const [select, setSelect] = useState(0);
  const items = getData();
  const total = items.length;
  const startFrom = (store.current - 1) * store.pageSize;
  const _items = items.slice(startFrom, Math.min(startFrom + store.pageSize, total));
  const maxPage = Math.floor(total / store.pageSize) + 1;
  const columes = ['no', 'endpoint', 'endpoint', 'total', 'tcpSize', 'tcp_', 'count', 'count_'];
  const cols = _items;
  const pg = [];
  for (let i = Math.max(1, store.current - 2); i <= Math.min(maxPage, store.current + 2); i += 1) {
    pg.push(i);
  }
  return (<div className="d-flex flex-nowrap" id="frame-page">
    <div className="main-content">
      <Table bordered hover size={'sm'}>
        <thead>
          <tr>
            {columes.map(c => <th>{c}</th>)}
          </tr>
        </thead>
        <tbody>
          {cols.map(item => <tr onClick={() => {
            setSelect(item.no);
          }}>
            <td className="no">{item.no}</td>
            <td className="ipadd">{item.ep1}</td>
            <td className="ipadd">{item.ep2}</td>
            <td className="time">{parseDataSize(item.total)}</td>
            <td className="time">{parseDataSize(item.tcp)}</td>
            <td className="time">{parseDataSize(item.tcpUse)}</td>
            <td className="time">{parseDataSize(item.count)}</td>
            <td className="time">{parseDataSize(item.countUse)}</td>
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

export default ConnectList;