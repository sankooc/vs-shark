
import ReactDOM from "react-dom/client";
import Application from "../fluent/index";
import { StoreProvider } from "../../share/context";
import { useStore } from './store';

ReactDOM.createRoot(document.getElementById('app')!).render(
  <StoreProvider store={useStore}>
    <Application />
  </StoreProvider>
);