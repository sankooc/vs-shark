
import { FluentProvider } from '@fluentui/react-components';
import '../../scss/flex.scss'
import './index.scss';
import Application from './app';

import { webDarkTheme } from "@fluentui/react-components";
const UI = () => {
  return <FluentProvider theme={webDarkTheme} style={{contain: 'content', height: '100%'}}>
    <Application/>
  </FluentProvider>
}

export default UI;
