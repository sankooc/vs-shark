import React from "react";
import './index.css';
import icon from "./icon256.png";
function Empty() {
  const image_src = icon;
  const text = "Select pcap file from menu";
  return (<div className="web-main flex align-items-center justify-content-center flex-column">
    <img className="animated bounce" src={image_src} />
    <div>{text}</div>
  </div>
  );
}

export default Empty;