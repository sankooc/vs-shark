import React from "react";
import './index.css';
// import { ReactSVG } from 'react-svg'
import Docs from "./doc.svg";

// import ReactLogo from './doc.svg';



const ErrPage = () => {
  const content = "Failed to parse file";
  return (<div className="error-page">
    <main>
      <div>
       <Docs/>
      </div>
      <div>
        <p>{content}</p>
      </div>
    </main>
  </div>)
}
export default ErrPage;