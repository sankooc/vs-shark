import React,{ ReactElement, useEffect, useState }  from "react";
import ProgressBar from 'react-bootstrap/ProgressBar';
import FrameList from './frame';
import FakeProgress from "fake-progress";
import "bootstrap/dist/css/bootstrap.css"
import { Frame } from "../common";

class MainProps {
    status: string;
    items: Frame[];
}

let interval;
let p: FakeProgress;
const timeConstant = 3000;
const inTime = 400;
function Main(props: MainProps) {
  const [ store, setStore ] = useState<any>({loaded: false, page: 'frame'});
  const [progress, setProgress] = useState<number>(0);
  useEffect(() => {
    switch(props.status){
      case 'init':{
        if(!interval){
          p = new FakeProgress({ timeConstant, autoStart : true });
          interval = setInterval(() => {
            setProgress(p.progress*100)
          }, inTime);
        } 
      }
      break;
      case 'done':
        setStore({...store, loaded: true});
        if(interval) {
          clearInterval(interval);
          interval = null;
        }
        if(p){
          p.end();
        }
        break;
      
    }
    // const root = readBuffers(props.data, 20);
    // root.addEventListener('init', () => {
    //   setStore({...store, loaded: false});
    // })
    // root.addEventListener('finish', () => {
    //   setStore({...store, loaded: true});
    // })
    // root.addEventListener('frame', (evt: CustomEvent<IPPacket[]>) => {
    //   const items: IPPacket[] = evt.detail;
    //   frames.push(...items);
    //   setFrame([...root.packets]);
    // });
  }, [props.status]);
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
    {store.loaded? null : <ProgressBar striped variant="success" animated now={progress}/>}
    <main className="d-flex flex-nowrap">
    <div className="d-flex flex-column flex-shrink-0 text-bg-dark navigation">
        <ul id="navc" className="nav nav-pills flex-column mb-auto">
            {buildNav('frame', 'Frame')}
            {buildNav('tcp', 'TCP')}
            {buildNav('dns', 'DNS')}
        </ul>
    </div>
    <div className="flex-grow-1" id="content">
      <FrameList items={props.items}></FrameList>
    </div>
    </main>
    </>
  );
}

export default Main;





