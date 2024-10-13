import React, { ReactElement, useEffect, useState } from "react";
import Viewer from './viewer';
import { TreeTable, TreeTableEvent } from "primereact/treetable";
import { Column, ColumnProps } from "primereact/column";
import { TreeNode } from "primereact/treenode";
import { emitMessage, onMessage } from "../../connect";
import { IHttpMessage } from "../../gen";
import { ComMessage } from "../../common";

class Props {
  cols: ColumnProps[];
  items: TreeNode[];
}

const SubComponnet = (props: Props) => {
  const mountHook = () => {
    const remv = onMessage('message', (e: any) => {
      const { type, body, requestId } = e.data;
      switch (type) {
        case '_http-content': {
          setCotent(body);
          break;
        }
      }
    });
    return remv;
  };
  useEffect(mountHook, []);
  const [content, setCotent] = useState<Uint8Array>(null);
  const [selection, setSelect] = useState<IHttpMessage>(null);
  const onSelect = (evt: TreeTableEvent) => {
    if(evt.node?.data?.data) {
      const msg: IHttpMessage = evt.node.data.data.msg;

      setSelect(msg);
      const param = evt.node.data.data.param;
      if(param && msg.len > 0){
        emitMessage(new ComMessage('http-content', param));
      }
    }
  };
  const scrollHeight = 70;
  const rowClass = (data: TreeNode): object => {
    const rs = {};
    if(!data.children) {
      rs[`type-${data.data.mes}`] = true;
      rs[`method-${data.data.target}`] = true;
    }
    return rs;
  }
  return (<>
    <TreeTable className="flex-grow-1 flex-shink-1" onRowClick={onSelect} rowClassName={rowClass} value={props.items} style={{height: `${scrollHeight}vh`, overflow: 'auto'}} size={0}>
        {props.cols.map((c: ColumnProps, inx: number): ReactElement => {
          return (<Column {...c} key={'col' + inx} ></Column>)
        })}
    </TreeTable>
    <Viewer key={Date.now()} content={content}  item = {selection}/>
  </>
  );
};

export default SubComponnet;
