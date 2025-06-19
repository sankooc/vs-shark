import { useState, useEffect } from "react";
import { Cursor, IField, IFrameSelect, VRange } from "../../../share/common";
import { useStore } from "../../store";
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
    backgroundColor: 'var(--colorBrandBackground2)',
  },
});


interface StackProps {
  select: number;
  // onSelect: (cursor: Cursor) => void;
}

export default function Stack(props: StackProps) {
  const styles = useStyles();
  const [data, setData] = useState<IFrameSelect>({ fields: [], start: 0, end: 0, data: new Uint8Array() });
  const [select, setSelect] = useState<string>("");
  const [cursor, setCursor] = useState<Cursor | undefined>();
  const _request = useStore((state) => state.request);
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
    // console.log(selected);
    if (selected.source) {
      let extra = data.extra;
      const cursor = {
        scope: new VRange(0, extra?.length || 0),
        data: extra,
        selected: {
          start: selected.start || 0,
          size: selected.size || 0,
        }
      };
      setCursor(cursor);
    } else {
      const cursor = {
        scope: new VRange(data.start, data.end),
        data: data.data,
        selected: {
          start: selected.start || 0,
          size: selected.size || 0,
        },
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
        <Tree>
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
    {({ height, width }) => {
      console.log('hex', height, width);
      return <div className="w-full flex">
        <div className="flex-1" style={{ height: (height - 2) + "px", overflow: "auto", borderRight: "var(--strokeWidthThin) solid var(--colorNeutralStroke2)" }}>
          <Tree aria-label="Default" size="small" className={styles.customTree}>
            {data.fields.map(build)}
          </Tree>
        </div>
        <div className="flex-1" style={{ height: (height - 2) + "px", overflow: "auto" }}>
          <HexView cursor={cursor} />
        </div>
      </div>
    }}
  </AutoSizer>
  );
}
