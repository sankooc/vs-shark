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
} from "@fluentui/react-icons";

import FrameComponent from "./frame";
import { useStore } from "../store";
import LoadingComponent from './loading';

const FrameIcon = bundleIcon(TextboxRotate9020Filled, TextboxRotate9020Regular);

const Basic = () => {
  const [select, setSelect] = React.useState<string>('Frames');


  const info = useStore((state) => state.fileinfo);
  const progress = useStore((state) => state.progress);

  console.log('----');
  console.log(info);
  console.log(progress);
  console.log('----');
  const renderComponent = (): React.ReactElement => {
    if (!info || !progress) {
      return <LoadingComponent info={info} progress={progress}/>
    }
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
        className="w-15rem"
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