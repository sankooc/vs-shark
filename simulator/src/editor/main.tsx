import React, { ReactElement, useEffect, useState } from "react";
import ProgressBar from 'react-bootstrap/ProgressBar';
import Overview from './overview';
import FrameList from './frame';
import ConnectList from './tcpConnection';
import DNSList from './dns';
import ARPReplies from './arp';
import FakeProgress from "fake-progress";
import "bootstrap/dist/css/bootstrap.css"
import { Frame, Grap, TCPCol, MainProps } from "../common";

let interval;
let p: FakeProgress;
const timeConstant = 3000;
const inTime = 400;
function Main(props: MainProps) {
  const [store, setStore] = useState<any>({ loaded: false, page: 'overview' });
  const [progress, setProgress] = useState<number>(0);
  useEffect(() => {
    switch (props.status) {
      case 'init': {
        if (!interval) {
          p = new FakeProgress({ timeConstant, autoStart: true });
          interval = setInterval(() => {
            setProgress(p.progress * 100)
          }, inTime);
        }
      }
        break;
      case 'done':
        setStore({ ...store, loaded: true });
        if (interval) {
          clearInterval(interval);
          interval = null;
        }
        if (p) {
          p.end();
        }
        break;

    }
  }, [props.status]);
  const buildNav = (key: string, txt: string, num: number = 0): ReactElement => {
    if (store.page === key) {
      return (<li key={key}>
        <a href="#" className="nav-link text-white active">{txt}</a>
      </li>);
    }
    return (<li key={key}><a href="#" className="nav-link text-white" onClick={() => {
      setStore({ ...store, page: key });
    }}>{txt}&nbsp;&nbsp;&nbsp;{num ? <span className="badge rounded-pill text-bg-warning">{num}</span> : null}</a></li>);
  }
  const getTable = (): ReactElement => {
    switch (store.page) {
      case 'overview':
        return <Overview data={props.overview} />
      case 'tcp':
        return <ConnectList items={props.tcps} />
      case 'arp':
        return <ARPReplies graph={props.arpGraph} legends={['sender', 'target']} />
      case 'dns':
        return <DNSList items={props.dnsRecords} />
    }
    return <FrameList items={props.items}></FrameList>;
  }
  const getNav = (): ReactElement[] => {
    const navs = [];
    navs.push(buildNav('overview', 'Overview'));
    if (props.items?.length) {
      navs.push(buildNav('frame', 'Frame', props.items?.length));
    }
    if (props.tcps?.length) {
      navs.push(buildNav('tcp', 'TCP', props.tcps?.length));
    }
    if (props.arpGraph?.nodes.length) {
      navs.push(buildNav('arp', 'ARP', props.arpGraph?.nodes.length));
    }
    if (props.dnsRecords?.length) {
      navs.push(buildNav('dns', 'DNS', props.dnsRecords.length));
    }
    return navs
  }
  return (
    <>
      {store.loaded ? null : <ProgressBar striped variant="success" animated now={progress} />}
      <main className="d-flex flex-nowrap">
        <div className="d-flex flex-column flex-shrink-0 text-bg-dark navigation">
          <ul id="navc" className="nav nav-pills flex-column mb-auto">
            {getNav()}
          </ul>
        </div>
        <div className="flex-grow-1" id="content">
          {getTable()}
        </div>
      </main>
    </>
  );
}

export default Main;





