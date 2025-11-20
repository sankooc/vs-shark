
import { FluentProvider } from '@fluentui/react-components';
// import "primeflex/primeflex.css";
import '../../scss/flex.scss'
import './index.scss';
import Application from './app';

import { webDarkTheme } from "@fluentui/react-components";
const UI = () => {
  return <FluentProvider theme={webDarkTheme} className="h-full">
    <Application/>
  </FluentProvider>
}

export default UI;
