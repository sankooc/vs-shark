import { useState, useEffect } from "react";
import { Cursor, IField, IFrameSelect } from "../../../share/common";
import { usePcapStore } from "../../context";
import { makeStyles, Tree, TreeItem, TreeItemLayout } from "@fluentui/react-components";
import AutoSizer from "react-virtualized-auto-sizer";
import HexView from './hex';



const useStyles = makeStyles({
  customTree: {
    '--spacingHorizontalXXL': '12px',
    // '&& .fui-Radio': {
    //   display: 'none',
    // },
    '--fontWeightRegular': 'bold',
  },
  itemSelect: {
    backgroundColor: '#524c42',
  },
});


interface StackProps {
  select: number;
  // onSelect: (cursor: Cursor) => void;
}

export default function Stack(props: StackProps) {
  const styles = useStyles();
  const [data, setData] = useState<IFrameSelect>({ fields: [], datasource: [] });
  const [select, setSelect] = useState<string>("");
  const [cursor, setCursor] = useState<Cursor | undefined>();
  const _request = usePcapStore((state) => state.request);
  useEffect(() => {
    if (props.select < 0) {
      return;
    }
    setSelect("");
    setCursor(undefined);
    _request<IFrameSelect>({
      catelog: "frame",
      type: "select",
      param: { index: props.select },
    }).then((rs) => {
      setData(rs);
    });
  }, [props.select]);

  const send = (selected: IField) => {
    const ds = data.datasource[selected.source || 0];
    if (ds) {
      const scope = ds.range;
      const tab = selected.source == 0 ? 'Frame' : 'Segment';
      const data = ds.data;
      const cursor = {
        scope,
        data,
        tab,
        selected: {
          start: selected.start || 0,
          size: selected.size || 0,
        }
      };
      setCursor(cursor);
    }
  }

  let counter = 0;
  const build = (it: IField) => {
    counter += 1;
    const key = `${props.select}_${counter}`;
    if (it.children && it.children.length) {
      return <TreeItem itemType="branch" key={key}>
        <TreeItemLayout onClick={() => {
          setSelect(key);
          send(it);
        }} className={select === key ? styles.itemSelect : ""} >{it.summary}</TreeItemLayout>
        <Tree size="small">
          {it.children.map((item, _inx) => {
            return build(item);
          })}
        </Tree>
      </TreeItem>
    } else {
      return <TreeItem itemType="leaf" key={key}>
        <TreeItemLayout onClick={() => {
          setSelect(key);
          send(it);
        }} className={select === key ? styles.itemSelect : ""} >{it.summary}</TreeItemLayout>
      </TreeItem>
    }
  }
  return (<AutoSizer className="h-full w-full">
    {({ height }) => {
      const _height = (height - 5) + "px";
      return <div className="w-full flex">
        <div className="flex-1" style={{ height: _height, overflow: "auto", borderRight: "var(--strokeWidthThin) solid var(--vscode-menu-border)" }}>
          <Tree aria-label="Default" size="small" className={styles.customTree}>
            {data.fields.map(build)}
          </Tree>
        </div>
        <div className="flex-1" style={{ height: _height, overflow: "auto" }}>
          <HexView cursor={cursor} />
        </div>
      </div>
    }}
  </AutoSizer>
  );
}
