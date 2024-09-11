
import React, { useState, useEffect } from 'react';
import { Tree, TreeEventNodeEvent, TreeNodeClickEvent } from 'primereact/tree';
import { TreeNode } from 'primereact/treenode';
import './app.css';
import { Field } from 'rshark';
import { CField } from '../../common';
const className = 'vector';
class StackProps {
    items: CField[];
    frame: number;
    onSelect: (frame: number, key: string, field:any) => void;
}
export default function Stack(props:StackProps) {
    const [store, setStore] = useState<any>({ items: [], key: '', data: null });
    useEffect(() => {
        setStore({key: ''})
    }, [props.frame]);
    const mapper = (it: CField, key: string): TreeNode => {
        const rs = {
            key,
            label: it.summary,
            data: it,
            className: store.key === key ? className +' active' : className,
            children: [],
            selectable: true,
        };
        let _inx = 0;
        for(const f of (it.children || [])){
            rs.children.push(mapper(f, `${key}_${_inx}`));
            _inx+=1;
        }
        return rs;
    }
    // console.log(props.items);
    
    const stacks: TreeNode[] = props.items.map((item, inx) => {
        return mapper(item, inx + '');
    });
    const onSelect = (e: TreeNodeClickEvent) => {
        const { node } = e;
        props.onSelect(props.frame, node.key + '', node.data);
        setStore({...store, key: node.key + ''})
    }
    return (
        <div className="flex-grow-1 justify-content-center" style={{ height: '100%', border: 0, padding: 0}}>
            <Tree className="tree-view" value={stacks} style={{ height: '30vh', border: 0, padding: 0 }} contentStyle={{ padding: 0, height: "100%" }} onNodeClick={onSelect} />
        </div>
    )
}