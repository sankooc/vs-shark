
import ReactDOM from "react-dom/client";
import Application from "../fluent/index";
import { StoreProvider } from "../context";
import { useStore } from './store';
import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

const App = () => {
  useEffect(() => {
    invoke("frontend_ready");
  }, []);
  return <Application />;
}

ReactDOM.createRoot(document.getElementById('app')!).render(
  <StoreProvider store={useStore}>
    <App />
  </StoreProvider>
);