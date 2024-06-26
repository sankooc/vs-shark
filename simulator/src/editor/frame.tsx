import React, { useEffect, useState } from "react";
import Table from 'react-bootstrap/Table';
import { emitMessage, trace } from "../connect";
import { ComMessage, Frame } from "../common";





class FrameListProps {
  items: Frame[];
}
const getStyle = (item: Frame,sel: number): string => {
  if(item.no === sel){
      return 'active';
  }
  return item.style;
}

function FrameList(props: FrameListProps) {
  const getData = (): Frame[] => {
    return props.items;
  };
  const [store, setStore] = useState<any>({ current: 1, pageSize: 500 });
  const [select, setSelect] = useState(0);
  const items = getData();
  const total = items.length;
  const startFrom = (store.current - 1) * store.pageSize;
  const _items = items.slice(startFrom, Math.min(startFrom + store.pageSize, total));
  const maxPage = Math.floor(total / store.pageSize) + 1;
  const columes = ['no', 'time', 'source', 'dest', 'protocol', 'length', 'info'];
  const cols = _items;

  const parseTime = (time: number): string => {
    const date = new Date(time);
    const [hour, minutes, seconds, ms ] = [
      date.getHours(),
      date.getMinutes(),
      date.getSeconds(),
      date.getMilliseconds()
    ];
    return `${minutes}:${seconds} ${ms}`;
  }
  const pg = [];
  for (let i = Math.max(1, store.current - 2); i <= Math.min(maxPage, store.current + 2); i += 1) {
    pg.push(i);
  }
  return (<div className="d-flex flex-nowrap" id="frame-page">
    <div className="input-group flex-row filter">
      <span className="input-group-text">&gt;</span>
      <input type="text" className="form-control" />
    </div>
    <div className="main-content">
      <Table bordered hover size={'sm'}>
        <thead>
          <tr>
            {columes.map(c => <th>{c}</th>)}
          </tr>
        </thead>
        <tbody>
          {cols.map(item => 
          (<tr className={getStyle(item, select)} onClick={() => {
            setSelect(item.no);
            emitMessage(new ComMessage('frame-select', { index: item.no }));
            trace('select ' + item.no);
          }}>
            <td className="no">{item.no}</td>
            <td className="time">{parseTime(item.time)}</td>
            <td className="ipadd">{item.source}</td>
            <td className="ipadd">{item.dest}</td>
            <td className="time">{item.protocol}</td>
            <td className="time">{item.len}</td>
            <td className="info">{item.info}</td></tr>))}
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

        {/* {pg.map( n => <li className="page-item"><a className={n === store.current ? "page-link active" :"page-link"} href="#" onClick={() => {
      setStore({...store, current: n});
    }}>{n}</a></li>)}
    {store.current < maxPage ? <li className="page-item"><a className="page-link" href="#" onClick={() => {
      setStore({...store, current: maxPage});
    }}>Last</a></li>: null} */}

      </ul>
    </nav>
  </div>
  );
}

export default FrameList;