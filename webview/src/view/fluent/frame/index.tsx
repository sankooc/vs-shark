import { useEffect, useState } from "react";
import { usePcapStore } from "../../context";
import { compute, ComRequest, PcapState } from "../../../share/common";
import { IFrameInfo, IListResult } from "../../../share/gen";
// import { makeStyles } from "@fluentui/react-components";
import AutoSizer from "react-virtualized-auto-sizer";


import { VirtualizedDataGrid } from './data';
import Stack from "./stack";
import { frame_size } from "../../conf";
import Paging from '../pagination2';

function Empty() {
  return <></>
}


function Component() {
  const _request = usePcapStore((state) => state.request);
  const progress = usePcapStore((state: PcapState) => state.progress);

  const [page, setPage] = useState<number>(1);
  const [result, setResult] = useState<IListResult<IFrameInfo>>({
    start: 0,
    total: 0,
    items: [],
  });
  const [select, setSelect] = useState<IFrameInfo | undefined>(undefined);
  const size = frame_size;
  const persist = `${page} ${JSON.stringify(progress || {})}`
  useEffect(() => {
    const data: ComRequest = {
      catelog: "frame",
      type: "list",
      param: compute(page, size),
    };
    _request<IListResult<IFrameInfo>>(data).then((rs) => {
      setResult(rs);
    });
  }, [persist]);
  return <AutoSizer>
    {({ height, width }) => {
      if(height < 370){
        return <span>need more space</span>
      }
      const bodyHeight = Math.ceil(height * 0.65);
      return <div className="flex flex-column frame-content" style={{ height: height + "px", width: width + "px" }}>
        <VirtualizedDataGrid bodyHeight={bodyHeight} items={result.items} onSelect={setSelect} />
        <Paging page={page} total={result.total} pageSize={size} onPageChange={(page: number) => {
          setPage(page);
          setSelect(undefined);
        }} />
        <div className="flex-grow-1" style={{ borderTop: "var(--strokeWidthThin) solid var(--colorNeutralStroke2)" }}>
          {select ? <Stack select={select.index} /> : <Empty />}
        </div>
      </div>
    }}
  </AutoSizer>
}

export default Component;

