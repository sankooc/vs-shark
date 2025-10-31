import * as React from "react";
import {
  Hamburger,
  makeStyles,
  NavCategory,
  NavCategoryItem,
  NavDrawer,
  NavDrawerBody,
  NavDrawerHeader,
  NavItem,
  NavSubItem,
  NavSubItemGroup,
} from "@fluentui/react-components";
import { BrowserRouter, useNavigate, Route, Routes, Navigate } from "react-router";

import FrameComponent from "./frame";
import ConversationComponent from "./conversation";
import ConversationDetailComponent from './conversation/detail';
import HttpComponent from "./http";
import HttpDetailComponent from "./http/detail";
import OverviewComponent from "./overview";
import UDPComponent from './udp';
import TLSHostList from './tls/hosts';
import { useStore } from "../store";
import LoadingComponent from './loading';
import { ConversationIcon, FrameIcon, HttpIcon, OverviewIcon, StatisticTabIcon, TLSIcon, UDPTabIcon } from "./common";

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

  //OverviewIcon
  const components = [{
    name: 'TCP',
    path: 'conversations',
    icon: ConversationIcon,
  }, {
    name: 'UDP',
    path: 'udp',
    icon: UDPTabIcon
  }, {
    name: 'HTTP',
    path: 'https',
    icon: HttpIcon,
  }, {
    name: 'TLS',
    path: 'tls/hosts',
    icon: TLSIcon,
  }];
  const styles = useCSS();
  if (!isOpen) {
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
    style={{ width: '12em' }}
  >
    <NavDrawerHeader>
      <Hamburger onClick={() => setIsOpen(false)} />
    </NavDrawerHeader>

    <NavDrawerBody>
      <NavItem
        onClick={() => {
          setSelect('Overview');
          navigate('/overview');
        }}
        icon={<OverviewIcon />}
        value={'Overview'}
        key={'Overview'}
      >
        Overview
      </NavItem>
      {/* <NavSectionHeader>Statistic</NavSectionHeader> */}
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

      <NavCategory value="static">
        <NavCategoryItem icon={<StatisticTabIcon />}>
          Stat
        </NavCategoryItem>
        <NavSubItemGroup>
          {
            components.map((item) => (
              <NavSubItem
                onClick={() => {
                  setSelect(item.name);
                  navigate('/' + item.path)
                }}
                value={item.name}
                key={item.name}
              >
                {<item.icon />} {item.name}
              </NavSubItem>
            ))
          }
        </NavSubItemGroup>
      </NavCategory>
      {/* {
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
      } */}
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
        <div className="flex-1 flex flex-column main-content">
          <Routes>
            <Route path="/" index element={<FrameComponent />} />
            {/* <Route path="/frames" element={<FrameComponent />} /> */}
            <Route path="/overview" element={<OverviewComponent />} />
            <Route path="/conversations" element={<ConversationComponent />} />
            <Route path="/conversation/:conversationIndex" element={<ConversationDetailComponent />} />
            <Route path="/https" element={<HttpComponent />} />
            <Route path="/http/detail/:httpIndex" element={<HttpDetailComponent />} />
            <Route path="/tls/hosts" element={<TLSHostList />} />
            <Route path="/udp" element={<UDPComponent />} />
            <Route path="*" element={<Navigate to="/" replace />} />
          </Routes>
        </div>
      </div>
    </BrowserRouter>
  );
};

export default Basic;