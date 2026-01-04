
import { FluentProvider } from '@fluentui/react-components';
import '../../scss/flex.scss'
import './index.scss';
import Application from './app';

// import { webDarkTheme } from "@fluentui/react-components";
import { customLightTheme } from './theme';

const UI = () => {
  return <FluentProvider theme={customLightTheme} style={{contain: 'content', height: '100%'}}>
    <Application/>
  </FluentProvider>
}

export default UI;
