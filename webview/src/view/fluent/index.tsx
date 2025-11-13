
import { FluentProvider } from '@fluentui/react-components';
import './index.scss';
import Application from './app';
// import { useStore } from '../store';

import { teamsDarkTheme } from "@fluentui/react-components";
const UI = () => {
  return <FluentProvider theme={teamsDarkTheme} className="h-full">
    <Application/>
  </FluentProvider>
}

export default UI;
