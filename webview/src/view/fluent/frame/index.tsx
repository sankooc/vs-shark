/* eslint-disable react-hooks/exhaustive-deps */
import { useEffect, useState } from "react";
import { useStore } from "../../store";
import { compute, ComRequest } from "../../../share/common";
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
  const _request = useStore((state) => state.request);
  const [page, setPage] = useState<number>(1);
  const [result, setResult] = useState<IListResult<IFrameInfo>>({
    start: 0,
    total: 0,
    items: [],
  });
  const [select, setSelect] = useState<IFrameInfo | undefined>(undefined);

  const size = frame_size;
  useEffect(() => {
    const data: ComRequest = {
      catelog: "frame",
      type: "list",
      param: compute(page, size),
    };
    _request<IListResult<IFrameInfo>>(data).then((rs) => {
      setResult(rs);
    });
  }, [page]);
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

