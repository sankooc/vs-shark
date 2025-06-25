
import { FluentProvider } from '@fluentui/react-components';
import './index.scss';
import Application from './app';
import { useStore } from '../store';


const UI = () => {
  const theme = useStore((state) => state.theme);
  console.log('apply theme', theme);
  return <FluentProvider theme={theme} className="h-full">
    <Application/>
  </FluentProvider>
}

export default UI;
