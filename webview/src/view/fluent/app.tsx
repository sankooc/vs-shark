import { BrowserRouter, Route, Routes, Navigate } from "react-router";
import Header from './menu';

import FrameComponent from "./frame";
import ConversationComponent from "./conversation";
import ConversationDetailComponent from './conversation/detail';
import HttpComponent from "./http";
import HttpDetailComponent from "./http/detail";
import OverviewComponent from "./overview";
import UDPComponent from './udp';
import DNSComponent from './dns';
import DNSRecordComponent from './dns/sub';
import TLSHostList from './tls';
import TLSConvList from './tls/sub';
import DebugComponent from './debug';

import { usePcapStore } from "../context";
import LoadingComponent from './loading';
import { PcapState } from "../../share/common";

const Basic = () => {
  const info = usePcapStore((state: PcapState) => state.fileinfo);
  const progress = usePcapStore((state: PcapState) => state.progress);
  if (!progress) {
    return <BrowserRouter>
      <Header />
      <LoadingComponent info={info} progress={progress} />
    </BrowserRouter>
  }

  return (
    <BrowserRouter>
      <div className="flex flex-column h-full w-full">
        <Header />
        <div className="flex-1 flex flex-column  main-content">
          <Routes>
            <Route path="/" index element={<FrameComponent />} />
            <Route path="/overview" element={<OverviewComponent />} />
            <Route path="/conversation" element={<ConversationComponent />} />
            <Route path="/conversation/:conversationIndex" element={<ConversationDetailComponent />} />
            <Route path="/https" element={<HttpComponent />} />
            <Route path="/http/detail/:httpIndex" element={<HttpDetailComponent />} />
            <Route path="/tlslist" element={<TLSHostList />} />
            <Route path="/tls/:index" element={<TLSConvList />} />
            <Route path="/udp" element={<UDPComponent />} />
            <Route path="/dns" element={<DNSComponent />} />
            <Route path="/dns/:index" element={<DNSRecordComponent />} />
            <Route path="/debug" element={<DebugComponent />} />
            <Route path="*" element={<Navigate to="/" replace />} />
          </Routes>
        </div>
        {/* <StatusBar /> */}
        {/* </div> */}
      </div>
    </BrowserRouter>
  );
};

export default Basic;