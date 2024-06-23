import React,{ useEffect, useState } from "react";
import { Tree } from 'antd';
import type { TreeDataNode, TreeProps } from 'antd';
import { onMessage } from '../connect';
import { CTreeItem } from "../common";

const convert = (item: CTreeItem, key: string): TreeDataNode => {
  const p = {
    title: item.label,
    key: key,
    children: []
  };
  for(let i = 0; i< item.children.length; i+=1){
    const ch = item.children[i];
    p.children.push(convert(ch, `${key}-${i}`));
  }
  return p;
}
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
  const root = new CTreeItem('root');
  root.children.push(...items);
  const _item = convert(root, Date.now() + '');
  const onSelect: TreeProps['onSelect'] = (selectedKeys, info) => {
    console.log('selected', selectedKeys, info);
  };

  return (
    <Tree
      showLine
      onSelect={onSelect}
      treeData={_item.children || []}
    />
  );
};

export default App;