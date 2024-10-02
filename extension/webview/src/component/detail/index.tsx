import React, { useEffect, useState, useRef } from "react";
import { HexV } from "../../common";
import { TabView, TabPanel } from 'primereact/tabview';
import Hex from './hex';
import './app.css';

function HexView(props: { data?: HexV }) {
  let hasSelected = false;
  let select = new Uint8Array();
  if(props.data && props.data.index) {
    const {index, data} = props.data;
    const [start, size] = index;
    if (data.length && size > 0 && start >= 0 ) {
      hasSelected = true;
      select = data.slice(start, start + size);
    }
  } else {
    return <div style={{padding: '10px'}}> No Data </div>
  }
  return <TabView className="w-full detail-tab" style={{padding: 0}}>
    <TabPanel header="Frame" style={{padding: 0}}>
      <Hex data={props.data}/>
    </TabPanel>
    {hasSelected && <TabPanel header="Selected">
    <Hex data={{index: [0, 0], data: select}}/>
    </TabPanel>}
  </TabView>
}

export default HexView;