import * as React from "react";
import {
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
import { useStore } from "../store";
import LoadingComponent from './loading';

const FrameIcon = bundleIcon(TextboxRotate9020Filled, TextboxRotate9020Regular);
const ConversationIcon = bundleIcon(FormSparkle20Filled, FormSparkle20Regular);
const HttpIcon = bundleIcon(PlugConnected20Filled, PlugConnected20Regular);

const Basic = () => {
  const [select, setSelect] = React.useState<string>('Frames');
  const info = useStore((state) => state.fileinfo);
  const progress = useStore((state) => state.progress);

  console.log('----');
  console.log(info);
  console.log(progress);
  console.log('----');
  if (!info || !progress) {
    return <LoadingComponent info={info} progress={progress}/>
  }
  const renderComponent = (): React.ReactElement => {
    return <FrameComponent />
  }
  const components = [{
    name: 'Frames',
    icon: FrameIcon,
    component: FrameComponent,
  }];

  return (
    <div className="flex flex-row h-full">
      <div className="flex-1">
        {renderComponent()}
      </div>
      <NavDrawer
        defaultSelectedValue={select}
        defaultSelectedCategoryValue=""
        open={true}
        type="inline"
        multiple={false}
        className="w-10rem"
      >

        <NavDrawerBody>
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
          <NavSectionHeader>Statistics</NavSectionHeader>
        </NavDrawerBody>
      </NavDrawer>
    </div>
  );
};

export default Basic;