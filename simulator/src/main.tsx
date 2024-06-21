import React,{ ReactElement, useEffect, useState }  from "react";
import ProgressBar from 'react-bootstrap/ProgressBar';
import FrameList from './frame';
import {IPPacket, Protocol, readBuffers, IPv4 } from "protocols"
import "bootstrap/dist/css/bootstrap.css"

class MainProps {
    data!: Uint8Array;
}
// class Store {
//   loaded: number = 0;
//   page: string =  'frame';
// }
function Main(props: MainProps) {
  const [ store, setStore ] = useState<any>({loaded: false, page: 'frame'});
  const [frames, setFrame] = useState<IPPacket[]>([]);
  useEffect(() => {
    const root = readBuffers(props.data, 20);
    root.addEventListener('init', () => {
      setStore({...store, loaded: false});
    })
    root.addEventListener('finish', () => {
      setStore({...store, loaded: true});
    })
    root.addEventListener('frame', (evt: CustomEvent<IPPacket[]>) => {
      const items: IPPacket[] = evt.detail;
      frames.push(...items);
      setFrame([...root.packets]);
    });
  }, []);
  const buildNav = (key: string, txt: string): ReactElement => {
    if(store.page === key) {
      return (<li><a href="#" className="nav-link text-white active">{txt}</a></li>);
    }
    return (<li><a href="#" className="nav-link text-white" onClick={() => {
      setStore({...store, page: key});
    }}>{txt}</a></li>);
  }
  return (
    <>
    {store.loaded? null : <ProgressBar now={70}/>}
    <main className="d-flex flex-nowrap">
    <div className="d-flex flex-column flex-shrink-0 text-bg-dark" style={{width: '200px'}}>
        <ul id="navc" className="nav nav-pills flex-column mb-auto">
            {buildNav('frame', 'Frame')}
            {buildNav('tcp', 'TCP')}
            {buildNav('dns', 'DNS')}
        </ul>
    </div>
    <div className="flex-grow-1" id="content">
      <FrameList items={frames} key={frames.length}></FrameList>
    </div>
    </main>
    </>
  );
}

export default Main;





