
import React, { useState, useEffect } from 'react';
import { Tree, TreeEventNodeEvent, TreeNodeClickEvent } from 'primereact/tree';
import { TreeNode } from 'primereact/treenode';
import { emitMessage, onMessage } from '../../connect';
import { CTreeItem, ComMessage } from '../../common';
import './app.css';

const className = 'vector';

class StackData {
    data: Uint8Array;
    items: CTreeItem[];
    key?: string;
}
export default function Stack() {
    const [store, setStore] = useState<StackData>({ items: [], key: '', data: null });
    useEffect(() => {
        onMessage('message', (e: any) => {
            const { type, body, requestId } = e.data;
            switch (type) {
                case 'frame':
                    setStore(body as StackData);
            }
        });
    }, []);
    let counter = 0;
    const mapper = (it: CTreeItem): TreeNode => {
        const key = 'item' + (counter += 1);
        const rs = {
            key,
            label: it.label,
            data: {data: store.data, index: it.index},
            className: store.key === key ? className+ ' active' : className,
            children: (it.children || []).map(mapper),
            selectable: true,
        };
        return rs;
    }
    const stacks: TreeNode[] = store.items.map(mapper);
    const onSelect = (e: TreeNodeClickEvent) => {
        const { node } = e;
        emitMessage(new ComMessage('hex-data', node.data));
        setStore({...store, key: node.key + ''})
    }
    return (
        <div className="flex-grow-1 justify-content-center" style={{ height: '100%', border: 0, padding: 0}}>
            <Tree className="tree-view" value={stacks} style={{ border: 0, padding: 0 }} contentStyle={{ padding: 0 }} onNodeClick={onSelect} />
        </div>
    )
}