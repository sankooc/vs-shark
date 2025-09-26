import * as React from "react";
import {
  Hamburger,
  makeStyles,
  NavDrawer,
  NavDrawerBody,
  NavDrawerHeader,
  NavItem
} from "@fluentui/react-components";
import { BrowserRouter, useNavigate, Route, Routes, Navigate } from "react-router";

import FrameComponent from "./frame";
import ConversationComponent from "./conversation";
import ConversationDetailComponent from './conversation/detail';
import HttpComponent from "./http";
import HttpDetailComponent from "./http/detail";
import { useStore } from "../store";
import LoadingComponent from './loading';
import { ConversationIcon, FrameIcon, HttpIcon } from "./common";

// import '../colors';

// const FrameIcon = bundleIcon(TextboxRotate9020Filled, TextboxRotate9020Regular);
// const ConversationIcon = bundleIcon(FormSparkle20Filled, FormSparkle20Regular);
// const HttpIcon = bundleIcon(PlugConnected20Filled, PlugConnected20Regular);

const useCSS = makeStyles({
  nav: {
    "& button[aria-current=page]": {
      color: 'rgb(71, 158, 245)'
    }
  }
});



const Nav = () => {
  const navigate = useNavigate();
  const [isOpen, setIsOpen] = React.useState(true);
  const [select, setSelect] = React.useState<string>('Frames');

  const components = [{
    name: 'Conversations',
    path: 'conversations',
    icon: ConversationIcon,
  }, {
    name: 'HTTPs',
    path: 'https',
    icon: HttpIcon,
  }];
  const styles = useCSS();
  if(!isOpen){
    return (<div style={{ borderRight: '1px solid #ddd', padding: '4px' }}>
            <Hamburger onClick={() => setIsOpen(true)} />
            </div>)
  }
  return <NavDrawer
    defaultSelectedValue={select}
    defaultSelectedCategoryValue=""
    open={isOpen}
    type="inline"
    multiple={false}
    className={styles.nav}
    style={{ width: '12em', borderRight: '1px solid #ddd' }}
  >
        <NavDrawerHeader>
            <Hamburger onClick={() => setIsOpen(false)} />
        </NavDrawerHeader>

    <NavDrawerBody>
      <NavItem
        onClick={() => {
          setSelect('Frames');
          navigate('/');
        }}
        icon={<FrameIcon />}
        value={'Frames'}
        key={'Frames'}
      >
        Frames
      </NavItem>
      {/* <NavSectionHeader>Stat</NavSectionHeader> */}
      {
        components.map((item) => (
          <NavItem
            onClick={() => {
              setSelect(item.name);
              navigate('/' + item.path)
            }}
            icon={<item.icon />}
            value={item.name}
            key={item.name}
          >
            {item.name}
          </NavItem>
        ))
      }
    </NavDrawerBody>
  </NavDrawer>
}


const Basic = () => {
  const info = useStore((state) => state.fileinfo);
  const progress = useStore((state) => state.progress);
  if (!progress) {
    return <LoadingComponent info={info} progress={progress} />
  }
  return (
    <BrowserRouter>
    <div className="flex flex-row h-full w-full">
        <Nav />
        <div className="flex-1" style={{ width: 'calc(100% - 12em)' }}>
          <Routes>
            <Route path="/" index element={<FrameComponent />} />
            {/* <Route path="/frames" element={<FrameComponent />} /> */}
            <Route path="/conversations" element={<ConversationComponent />} />
            <Route path="/conversation/:conversationIndex" element={<ConversationDetailComponent />} />
            <Route path="/https" element={<HttpComponent />} />
            <Route path="/http/detail" element={<HttpDetailComponent />} />
            <Route path="*" element={<Navigate to="/" replace />} />
          </Routes>
        </div>
    </div>
    </BrowserRouter>
  );
};

export default Basic;