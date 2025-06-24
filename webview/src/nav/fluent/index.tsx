
import { FluentProvider, webDarkTheme } from '@fluentui/react-components';
import '../index.scss';
import Application from './app';


const UI = () => {
  return <FluentProvider theme={webDarkTheme} className="flex flex-column h-full">
    <Application/>
  </FluentProvider>
}

export default UI;
