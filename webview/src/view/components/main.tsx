import { ReactElement, useEffect, useState } from "react";
import { MenuItem } from "primereact/menuitem";
import { Badge } from "primereact/badge";
import { Menu } from "primereact/menu";
import {
  ComMessage,
  ComType,
  IContextInfo,
  ILines,
  IStatistic,
  deserialize,
} from "../../core/common";
// import { onMessage, log, emitMessage } from "../../core/connect";

import FrameList from "./frame";
import Empty from "./loading/empty";
import Loading from "./loading";
import { _log } from "../util";
import { useStore } from "../store";
// import { create } from 'zustand'

// import _httpRecords from '../mock/http.json';

const itemRenderer = (item: any, options: any) => {
  return (
    <a
      className="flex align-items-center px-3 py-2 cursor-pointer"
      onClick={options.onClick}
    >
      {item.icon && <span className={item.icon} />}
      <span className={`mx-2 ${item.items && "font-semibold"}`}>
        {item.label}
      </span>
      {item.data && <Badge className="ml-auto" value={item.data} />}
    </a>
  );
};

// let _start = 0;
const Main = () => {
  const [select, setSelect] = useState("frame");
  const [status, setStatus] = useState<number>(-1);
  const sendReady = useStore((state) => state.sendReady);
  useEffect(() => {
    sendReady();
    //   return onMessage("message", (e: any) => {
    //     const { type, body, id } = e.data;
    //     _log(type, body, id);
    //     switch (type) {
    //       case ComType.SERVER_REDAY: {
    //         emitMessage(ComMessage.new(ComType.CLIENT_REDAY, Date.now()));
    //         setStatus(1);
    //         break;
    //       }
    //     }
    //   });
    //   // _start = Date.now();
    //   // emitMessage(new ComMessage('ready', 'demo'));
  }, []);
  const convert = (): MenuItem[] => {
    const mitems: MenuItem[] = [];
    const addPanel = (
      id: string,
      label: string,
      extra: string,
      icon: string = "",
    ): void => {
      mitems.push({
        id,
        data: extra,
        template: itemRenderer,
        label,
        icon,
        className: select === id ? "active" : "",
        command: (env) => {
          // setSelect(env.item.id);
        },
      });
    };
    addPanel("frame", "Frame", "", "pi pi-list");
    return mitems;
  };
  const buildPage = (): ReactElement => {
    return <FrameList />;
  };
  if (status == -1) {
    return <Empty />;
  }
  if (status == 0) {
    return <Loading />;
    // return <ErrPage/>
    // return <TLSComponent items={tlsRecords}/>
    // return <HttpComponnet items={_httpRecords} />
    // return <DNSList items={_dnsRecords}/>
    // return <Overview framedata={overview_json} metadata={meta_json} httpdata={http_json.statistic} />
  }
  const navItems = convert();
  return (
    <>
      <div className="card h-full">
        {/* <BreadCrumb model={items} home={home} /> */}
        <div className="flex flex-row h-full">
          <div className="w-full flex flex-grow-1">{buildPage()}</div>
          <div
            className="flex flex-column flex-grow-0 flex-shrink-0"
            style={{ width: "10vw" }}
          >
            <Menu model={navItems} className="w-full h-full" />
          </div>
        </div>
      </div>
    </>
  );
};

export default Main;
