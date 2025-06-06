import { useState, useEffect } from "react";
import { Tree, TreeNodeClickEvent } from "primereact/tree";
import { TreeNode } from "primereact/treenode";
import "./app.scss";
import { Cursor, IField, IFrameSelect, VRange } from "../../../../share/common";
import { useStore } from "../../../store";

const className = "vector";

interface StackProps {
  select: number;
  onSelect: (cursor: Cursor) => void;
}
export default function Stack(props: StackProps) {
  const [data, setData] = useState<IFrameSelect>({ fields: [], start: 0, end: 0, data: new Uint8Array() });
  // const [fields, setField] = useState<IField[]>([]);
  const [select, setSelect] = useState<string>("");
  // const [scope, setScope] = useState<VRange>(new VRange(0,0));
  const _request = useStore((state) => state.request);
  useEffect(() => {
    if (props.select < 0) {
      return;
    }
    _request<IFrameSelect>({
      catelog: "frame",
      type: "select",
      param: { index: props.select },
    }).then((rs) => {
      setData(rs);
      // let fs: IFrameSelect = rs;
      // console.log(rs);
      // setField(rs.fields);
      // setScope(new VRange(rs.start, rs.end));
      // setField(rs);
    });
  }, [props.select]);
  const mapper = (it: IField, key: string): TreeNode => {
    const rs: TreeNode = {
      key,
      label: it.summary,
      data: it,
      // className,
      className: select === key ? className + " active" : className,
      selectable: true,
    };
    if (it.children && it.children.length) {
      let _inx = 0;
      let children: TreeNode[] = [];
      for (const f of it.children) {
        const ch: TreeNode = mapper(f, `${key}_${_inx}`);
        children.push(ch);
        _inx += 1;
      }
      rs.children = children;
    }
    return rs;
  };
  const items = data.fields || [];
  const stacks: TreeNode[] = items.map((item, inx) => {
    return mapper(item, inx + "");
  });
  const onSelect = (e: TreeNodeClickEvent) => {
    const { node } = e;
    const selected: IField = node.data;
    setSelect(node.key + "");
    // console.log(selected);
    if(selected.source){
      let extra = data.extra;
      const cursor = {
        scope: new VRange(0, extra?.length || 0),
        data: extra,
        selected: {
          start: selected.start || 0,
          size: selected.size || 0,
        }
      };
      props.onSelect(cursor);
    } else {
      const cursor = {
        scope: new VRange(data.start, data.end),
        data: data.data,
        selected: {
          start: selected.start || 0,
          size: selected.size || 0,
        },
      };
      props.onSelect(cursor);
    }
  };
  return (
    <div
      className="flex-grow-1 justify-content-center"
      style={{ height: "100%", border: 0, padding: 0 }}
    >
      <Tree
        className="tree-view"
        value={stacks}
        style={{ height: "30vh", border: 0, padding: 0 }}
        contentStyle={{ padding: 0, height: "100%" }}
        onNodeClick={onSelect}
      />
    </div>
  );
}
