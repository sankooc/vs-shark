import React,{ useEffect, useState } from "react";
import Table from 'react-bootstrap/Table';
import {IPPacket, Protocol, IPv4 } from "protocols"




class FrameListProps {
  items!: IPPacket[];
}
class Frame {
  no!: number;
  time: string = '';
  source: string = 'n/a';
  dest: string = 'n/a';
  protocol: string;
  iRtt: number = 0;
  len: number = 0;
  info: string;
}
const _map: string[] = [
  'ETHER',
  'MAC',
  'IPV4',
  'IPV6',
  'ARP',
  'TCP',
  'UDP',
  'ICMP',
  'IGMP',
  'DNS',
  'NBNS',
  'DHCP',
];
const convert = (packet: IPPacket): Frame => {
  const rs = new Frame();
  rs.no = packet.getIndex();
  const ip = (packet.getProtocal(Protocol.IPV4) || packet.getProtocal(Protocol.IPV6)) as IPv4;
  rs.protocol = _map[packet.protocol];
  if(ip){
    rs.source = ip.source;
    rs.dest = ip.target;
  }
  rs.len = packet.getProtocal(Protocol.ETHER).packet.length;
  rs.info = packet.toString();
  return rs;
}

function FrameList(props: FrameListProps) {

  const getData = ():IPPacket[]  => {
    return props.items;
  };
  const [store, setStore] = useState<any>({current: 1, total: props.items.length, pageSize: 500});
  const items = getData();
  const startFrom = (store.current - 1) * store.pageSize;
  const _items = items.slice(startFrom, Math.min(startFrom + store.pageSize, store.total));
  const maxPage = Math.floor(store.total / store.pageSize) + 1;
  const slint = maxPage > 4;
  const columes = ['no', 'time', 'source', 'dest', 'protocol', 'length', 'info'];
  const cols = _items.map(convert);
  const pg = [];
  for(let i = Math.max(1, store.current - 2); i <= Math.min(maxPage, store.current + 2) ; i += 1){
    pg.push(i);
  }
  return (<div className="d-flex flex-nowrap" id="frame-page">
    <div className="input-group mb-3 flex-row filter">
      <span className="input-group-text">&gt;</span>
      <input type="text" className="form-control"/>
    </div>
    <div className="main-content">
    <Table bordered hover size={'sm'} style={{fontSize: '.9em'}}>
      <thead>
        <tr>
          {columes.map(c => <th>{c}</th>)}
        </tr>
      </thead>
      <tbody>
        {cols.map(item => <tr>
        <td className="no">{item.no}</td>
        <td className="time">{item.time}</td>
        <td className="ipadd">{item.source}</td>
        <td className="ipadd">{item.dest}</td>
        <td className="time">{item.protocol}</td>
        <td className="time">{item.len}</td>
        <td className="info">{item.info}</td></tr>)}
      </tbody>
    </Table>

    </div>
    <nav aria-label="Page navigation example" className="main-pagination">
  <ul className="pagination">
    {store.current > 1 ? <li className="page-item"><a className="page-link" href="#" onClick={() => {
      const { current } = store;
      setStore({...store, current: current - 1});
    }}>Previous</a></li> : null}
    {pg.map( n => <li className="page-item"><a className={n === store.current ? "page-link active" :"page-link"} href="#" onClick={() => {
      setStore({...store, current: n});
    }}>{n}</a></li>)}
    {store.current < maxPage ? <li className="page-item"><a className="page-link" href="#" onClick={() => {
      setStore({...store, current: maxPage});
    }}>Last</a></li>: null}
    
  </ul>
</nav>
  </div>
  );
}

export default FrameList;