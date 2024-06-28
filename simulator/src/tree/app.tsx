import React,{ useEffect, useState } from "react";
import { onMessage, emitMessage } from '../connect';
import { CTreeItem, ComMessage, HexV } from "../common";
import "./app.css";

// const convert = (item: CTreeItem, key: string): TreeDataNode => {
//   const p = {
//     title: item.label,
//     key: key,
//     children: []
//   };
//   for(let i = 0; i< item.children.length; i+=1){
//     const ch = item.children[i];
//     p.children.push(convert(ch, `${key}-${i}`));
//   }
//   return p;
// }
const App: React.FC = () => {
  const [{items, data}, setItem] = useState<any>({items: []});
  useEffect(() => {
    onMessage('message', (e: any) => {
      const { type, body, requestId } = e.data;
      switch (type) {
        case 'frame':
          setItem(body);
      }
    });
  }, []);
  const build = (item: CTreeItem) => {
    const len = item.children.length;
    if(len){
      return <details className="tree-nav__item is-expandable">
        <summary className="tree-nav__item-title" onClick={() => {
        if(item.index && item.index.length){
          const h = new HexV(data);
          h.index = item.index;
          emitMessage(new ComMessage<HexV>('hex-data', h));
        }
      }}>{item.label}</summary>
        <div className="tree-nav__item">
          {item.children.map(it => build(it))}
        </div>
      </details>;
    } else {
      return <a className="tree-nav__item-title" onClick={() => {
        if(item.index && item.index.length){
          const h = new HexV(data);
          h.index = item.index;
          emitMessage(new ComMessage<HexV>('hex-data', h));
        }
      }}>{item.label}</a>
    }
  };
  return (
    <nav className="tree-nav">
      {items.map((it) => build(it))}
    </nav>
  );
};

export default App;