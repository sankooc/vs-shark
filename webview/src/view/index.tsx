import React from "react";
import ReactDOM from "react-dom/client";

import Application from "./components/main";
import "primeflex/primeflex.css";
import "primeicons/primeicons.css";
import "./scss/index.scss";

ReactDOM.createRoot(document.getElementById("app")!).render(<Application />);
