
import React, { useState, useEffect } from 'react';
import { Tree, TreeEventNodeEvent, TreeNodeClickEvent } from 'primereact/tree';
import { TreeNode } from 'primereact/treenode';
import './app.css';
import { Field } from 'rshark';
const className = 'vector';
class StackProps {
    items: Field[];
    frame: number;
    // data: Uint8Array;
    onSelect: (field) => void;
}
export default function Stack(props:StackProps) {
    const [store, setStore] = useState<any>({ items: [], key: '', data: null });
    let counter = 0;
    useEffect(() => {
        setStore({key: ''})
    }, [props.frame]);
    const mapper = (it: Field): TreeNode => {
        const key = 'item' + (counter += 1);
        const rs = {
            key,
            label: it.summary,
            data: it,
            className: store.key === key ? className +' active' : className,
            children: [],
            selectable: true,
        };
        for(const f of (it.children || [])){
            if(f.summary) {
                rs.children.push(mapper(f));
            }
        }
        return rs;
    }
    const stacks: TreeNode[] = props.items.map(mapper);
    const onSelect = (e: TreeNodeClickEvent) => {
        const { node } = e;
        props.onSelect(node.data);
        setStore({...store, key: node.key + ''})
    }
    return (
        <div className="flex-grow-1 justify-content-center" style={{ height: '100%', border: 0, padding: 0}}>
            <Tree className="tree-view" value={stacks} style={{ border: 0, padding: 0 }} contentStyle={{ padding: 0 }} onNodeClick={onSelect} />
        </div>
    )
}