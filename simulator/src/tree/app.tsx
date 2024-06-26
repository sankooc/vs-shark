import React,{ useEffect, useState } from "react";
import { onMessage } from '../connect';
import { CTreeItem } from "../common";
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
  const [items, setItem] = useState<CTreeItem[]>([]);
  useEffect(() => {
    onMessage('message', (e: any) => {
      const { type, body, requestId } = e.data;
      switch (type) {
        case 'frame':
          const data = body as CTreeItem[];
          setItem(data);
      }
    });
  }, []);
  // const root = new CTreeItem('root');
  // root.children.push(...items);
  // const _item = convert(root, Date.now() + '');
  // const onSelect: TreeProps['onSelect'] = (selectedKeys, info) => {
  //   console.log('selected', selectedKeys, info);
  // };
  const build = (item: CTreeItem) => {
    const len = item.children.length;
    if(len){
      return <details className="tree-nav__item is-expandable">
        <summary className="tree-nav__item-title">{item.label}</summary>
        <div className="tree-nav__item">
          {item.children.map(it => build(it))}
        </div>
      </details>;
    } else {
      return <a className="tree-nav__item-title"><i className="icon ion-ios-bookmarks"></i> {item.label}</a>
    }
  };
  return (
    <nav className="tree-nav">
      {items.map((it) => build(it))}
    </nav>
  );
};

export default App;