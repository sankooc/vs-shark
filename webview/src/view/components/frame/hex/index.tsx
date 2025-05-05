import { TabView, TabPanel } from "primereact/tabview";
import Hex from "./hex";
import "./app.scss";
import { Cursor } from "../../../../share/common";
import { useStore } from "../../../store";
import { useEffect, useState } from "react";

interface Props {
  cursor: Cursor;
}

function HexView(props: Props) {
  let hasSelected = false;
  let select = new Uint8Array();
  const [bin, setBin] = useState<Uint8Array>();
  const request = useStore((state) => state.requestData);

  let selected: [number, number] | undefined = undefined;
  const scope = props.cursor.scope;
  let fetch = "";
  if (scope) {
    fetch = `${scope.start}-${scope.size}`;
    const inx = props.cursor.selected;
    if (inx) {
      const start = Math.max(inx.start - scope.start, 0);
      selected = [start, inx.size];
    }
  }

  useEffect(() => {
    const scope = props.cursor.scope;
    if (!scope) {
      return;
    }
    console.log("fetch data");
    request(scope).then((rs: { data: Uint8Array }) => {
      const _bin = rs.data;
      if (_bin.length == 0) {
        return;
      }
      setBin(rs.data);
    });
  }, [fetch]);
  if (!bin) {
    return <div style={{ padding: "10px" }}> No Data </div>;
  }

  if (selected) {
    hasSelected = true;
    select = bin.slice(selected[0], selected[0] + selected[1]);
  }
  return (
    <TabView
      className="w-full h-full flex flex-column detail-tab"
      style={{ padding: 0 }}
    >
      <TabPanel header="Frame" style={{ padding: 0 }}>
        <Hex bin={bin} highlight={selected} />
      </TabPanel>
      {hasSelected && (
        <TabPanel header="Selected">
          <Hex bin={select} highlight={[0, 0]} />
        </TabPanel>
      )}
    </TabView>
  );
}

export default HexView;
