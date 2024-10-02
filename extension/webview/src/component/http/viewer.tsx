import React, { useState } from "react";
import { Tree } from 'primereact/tree';
import { TreeNode } from 'primereact/treenode';
import { IHttp, IHttpEnity } from '../../common';
import Content from './view';
class Proto {
    item: IHttp
}
const Viewer = (props: Proto) => {
    console.log(props.item);
    const [store, setStore] = useState<any>({ items: [], key: '', data: null });
    const [message, setMessage] = useState<IHttpEnity>(null);
    const convert = (item: IHttp): TreeNode[] => {
        if (!item) {
            return [];
        }
        const items = [];
        let msg = item.req;
        let counter = 1;
        items.push({
            label: 'HTTP Resquest',
            key: 'request',
            style: { padding: 0 },
            selectable: true,
            data: msg,
            className: store.key === "request" ? 'http-msg-head active' : 'http-msg-head',
            children: [{ label: msg.head }, ...msg.header.map(h => ({ key: counter++, label: h }))]
        });

        msg = item.res;
        items.push({
            label: 'HTTP Response',
            key: 'respone',
            data: msg,
            style: { padding: 0 },
            selectable: true,
            className: store.key === "respone" ? 'http-msg-head active' : 'http-msg-head',
            children: [{ label: msg.head }, ...msg.header.map(h => ({ selectable: false, key: counter++, label: h }))]
        });
        return items;
    }
    const value = convert(props.item);
    const onSelect = (e) => {
        const { node } = e;
        if(store.key != node.key){
            setStore({ ...store, key: node.key + '' });
        }
        setMessage(node.data);
    };
    return (<div className="http-viewer flex-grow-1 flex flex-row align-items-center justify-content-between">
        <div className="http-header-view">
            <Tree className="tree-view" value={value} style={{ height: '30vh', border: 0, padding: 0 }} contentStyle={{ padding: 0, height: "100%" }} onNodeClick={onSelect} />
        </div>
        <div className="http-body-view">
            <Content message={message}/>
        </div>
    </div>);
};

export default Viewer;
