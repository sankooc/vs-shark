import { useState, useEffect } from "react";
import { Tree, TreeNodeClickEvent } from "primereact/tree";
import { TreeNode } from "primereact/treenode";
import "./app.scss";
import { Cursor, IField } from "../../../../share/common";
import { useStore } from "../../../store";

const className = "vector";

interface StackProps {
  select: number;
  onSelect: (cursor: Cursor) => void;
}
export default function Stack(props: StackProps) {
  const [field, setField] = useState<IField>();
  const [select, setSelect] = useState<string>("");
  const _request = useStore((state) => state.request);
  useEffect(() => {
    if (props.select < 0) {
      return;
    }
    _request<IField>({
      catelog: "frame",
      type: "select",
      param: { index: props.select },
    }).then((rs) => {
      setField(rs);
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

  const items = field?.children || [];
  const stacks: TreeNode[] = items.map((item, inx) => {
    return mapper(item, inx + "");
  });
  const onSelect = (e: TreeNodeClickEvent) => {
    const { node } = e;
    setSelect(node.key + "");
    const cursor = {
      scope: {
        start: field?.start || 0,
        size: field?.size || 0,
      },
      selected: {
        start: node.data.start || 0,
        size: node.data.size || 0,
      },
    };
    props.onSelect(cursor);
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
