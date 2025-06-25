import * as React from "react";
import {
  makeStyles,
  NavDrawer,
  NavDrawerBody,
  NavItem,
  NavSectionHeader,
} from "@fluentui/react-components";

import {
  bundleIcon,
  TextboxRotate9020Regular,
  TextboxRotate9020Filled,
  FormSparkle20Regular,
  FormSparkle20Filled,
  PlugConnected20Filled,
  PlugConnected20Regular,
} from "@fluentui/react-icons";

import FrameComponent from "./frame";
import ConversationComponent from "./conversation";
import HttpComponent from "./http";
import { useStore } from "../store";
import LoadingComponent from './loading';

const FrameIcon = bundleIcon(TextboxRotate9020Filled, TextboxRotate9020Regular);
const ConversationIcon = bundleIcon(FormSparkle20Filled, FormSparkle20Regular);
const HttpIcon = bundleIcon(PlugConnected20Filled, PlugConnected20Regular);

const useCSS = makeStyles({
  nav: {
    "& button[aria-current=page]": {
      color: 'rgb(71, 158, 245)'
    }
  }
});

const Basic = () => {
  const [select, setSelect] = React.useState<string>('Frames');
  const info = useStore((state) => state.fileinfo);
  const progress = useStore((state) => state.progress);
  const styles = useCSS();

  // console.log('----');
  // console.log(info);
  // console.log(progress);
  // console.log('----');
  if (!progress) {
    return <LoadingComponent info={info} progress={progress} />
  }
  const renderComponent = (): React.ReactElement => {
    switch (select) {
      case 'Conversations':
        return <ConversationComponent />
      case 'Frames':
        return <FrameComponent />
      case 'HTTPs':
        return <HttpComponent />
      default:
        return <FrameComponent />
    }
  }
  const components = [{
    name: 'Conversations',
    icon: ConversationIcon,
  }, {
    name: 'HTTPs',
    icon: HttpIcon,
  }];

  return (
    <div className="flex flex-row h-full w-full">
      <div className="flex-1" style={{ width: '85vw' }}>
        {renderComponent()}
      </div>
      <NavDrawer
        defaultSelectedValue={select}
        defaultSelectedCategoryValue=""
        open={true}
        type="inline"
        multiple={false}
        className={styles.nav}
        style={{ width: '15vw' }}
      >

        <NavDrawerBody>
          <NavItem
            onClick={() => {
              setSelect('Frames');
            }}
            icon={<FrameIcon />}
            value={'Frames'}
            key={'Frames'}
          >
            Frames
          </NavItem>
          <NavSectionHeader>Statistics</NavSectionHeader>
          {
            components.map((item) => (
              <NavItem
                onClick={() => {
                  setSelect(item.name);
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
    </div>
  );
};

export default Basic;