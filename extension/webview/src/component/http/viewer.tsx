import React, { useState } from "react";
import { Tree } from 'primereact/tree';
import { TreeNode } from 'primereact/treenode';
import Content from './view';
import { IConnect, IHttpMessage } from "../../gen";
class Proto {
    item: IHttpMessage;
    content?: Uint8Array;
}
const Viewer = (props: Proto) => {
    const [store, setStore] = useState<any>({ items: [], key: '', data: null });
    const convert = (item: IHttpMessage): TreeNode[] => {
        if (!item) {
            return [];
        }
        const items = [];
        items.push({ label: item.head});
        item.headers.map((label) => {items.push({label})})
        return items;
    }
    const value = convert(props.item);
    const onSelect = (e) => {
        const { node } = e;
        if(store.key != node.key){
            setStore({ ...store, key: node.key + '' });
        }
        // setMessage(node.data);
    };
    return (<div className="http-viewer flex flex-row align-items-center justify-content-between">
        <div className="http-header-view">
            <Tree className="tree-view" value={value} style={{ height: '30vh', border: 0, padding: 0 }} contentStyle={{ padding: 0, height: "100%" }} onNodeClick={onSelect} />
        </div>
        <div className="http-body-view">
            <Content message={props.item} content={props.content}/>
        </div>
    </div>);
};

export default Viewer;
