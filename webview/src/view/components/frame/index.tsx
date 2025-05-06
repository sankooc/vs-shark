import { useEffect, useState } from "react";
import DTable from "../dataTable2.tsx";
import { useStore } from "../../store";
import { PaginatorPageChangeEvent } from "primereact/paginator";
import { ComRequest, Cursor, Pagination } from "../../../share/common.ts";
import { IFrameInfo, IListResult } from "../../../share/gen.ts";
import { ColumnProps } from "primereact/column";
import { Tooltip } from "primereact/tooltip";
import Stack from "./tree";
import HexView from "./hex";
import dayjs from "dayjs";

const PAGE_SIZE = 500;
function FrameList() {
  // const [filter, setFilter] = useState<any[]>([]);
  // const [options, setOptions] = useState<string[]>([]);
  const size = PAGE_SIZE;
  const _request = useStore((state) => state.request);
  const [page, setPage] = useState<number>(1);
  const [select, setSelect] = useState<number>(-1);
  const [cursor, setCursor] = useState<Cursor>({});
  const [result, setResult] = useState<IListResult<IFrameInfo>>({
    start: 0,
    total: 0,
    items: [],
  });
  // const frameResult = useStore(
  //   (state) => state.frameResult,
  // ) || { start: 0, total: 0, items: [] };
  // const frameSelect = useStore(
  //   (state) => state.frameSelect,
  // )
  // console.log('frameSelect:', frameSelect);
  // const { items, start, total } = frameResult;
  // const page = Math.floor(start / PAGE_SIZE) + 1;
  const compute = (page: number, size: number): Pagination => {
    if (page < 1) {
      return { start: 0, size: size };
    }
    const start = (page - 1) * size;
    return { start, size };
  };
  const mountHook = () => {
    const data: ComRequest = {
      catelog: "frame",
      type: "list",
      param: compute(page, size),
    };
    _request<IListResult<IFrameInfo>>(data).then((rs) => {
      // console.log('rs', rs);
      setResult(rs);
    });
  };
  useEffect(mountHook, [page]);
  const columes: ColumnProps[] = [
    {
      field: "index",
      header: "index",
      style: { width: "5rem" },
      body: (item: any) => item.index + 1,
    },
    {
      field: "time",
      header: "time",
      style: { width: "7rem" },
      body: (item: any) => {
        const time = item.time;
        if (time > 1000) {
          const lis = item.time % 1000;
          const ts = Math.round(item.time / 1000);
          // const trimts = item.time % 1000000000;
          const date = dayjs(ts).format("YYYY-MM-DD HH:mm:ss");
          return (
            <>
              <Tooltip target=".time-target" />
              <span
                className="time-target"
                data-pr-tooltip={date}
                data-pr-position="right"
                data-pr-at="right+5 top"
                data-pr-my="left center-2"
              >
                {date.substring(date.length - 5) + ":" + lis}
              </span>
            </>
          );
        }
        return "n/a";
      },
      // headerTooltip: 'headerTooltip',
    },
    { field: "source", header: "source", style: { width: "17.5rem" } },
    { field: "dest", header: "dest", style: { width: "17.5rem" } },
    { field: "protocol", header: "protocol", style: { width: "7rem" } },
    { field: "len", header: "len", style: { width: "5.5rem" } },
    { field: "info", header: "info" },
  ];
  const onSelect = (_item: IFrameInfo): void => {
    setSelect(_item.index);
    // _request<IField[]>({catelog: "frame", type: "select", param: _item}).then((rs) => {
    //   console.log(rs);
    // })
    // setIndex(item.index);
    // emitMessage(new ComMessage('fields', item.index - 1));
    // setHex(new HexV(new Uint8Array()));
  };
  const getStyle = (item: any) => {
    switch (item.status) {
      case "deactive":
        return item.status;
      case "errordata":
        return "errdata";
      default: {
        return (item.protocol || "").toLowerCase();
      }
    }
  };
  // const onStackSelect = (index: number, key: string, _f: any) => {
  //   emitMessage(new ComMessage('hex',{index: index - 1, key}));
  // };
  const request = (event: PaginatorPageChangeEvent) => {
    setPage(event.page + 1);
    // const { rows, page } = event;
    // const data: ComRequest = {
    //   catelog: "frame",
    //   type: "list",
    //   param: compute(page + 1, rows),
    // };
    // _request(data);
  };
  // const extraFilter = {
  //   // options,
  //   value: filter,
  //   onChange: (e: any) => {
  //     setFilter(e.value);
  //     // const _filter = e.value.map((f: any) => f.code).join("&");
  //     // emitMessage(new ComMessage('frame', {page:1, size: PAGE_SIZE, filter: _filter}));
  //   },
  // };
  return (
    <div className="flex flex-nowrap flex-column h-full w-full" id="frame-page">
      <div
        className="editor flex-shrink-0"
        style={{
          height: "70vh",
          overflow: "hidden",
          borderBottom: "1px solid var(--vscode-list-focusBackground)",
        }}
      >
        <DTable
          key={page}
          // filter={extraFilter}
          cols={columes}
          result={{ items: result.items, page, size, total: result.total }}
          getStyle={getStyle}
          onSelect={onSelect}
          request={request}
          scrollHeight={70}
        />
      </div>

      <div className="viewer flex-grow-1 flex flex-row">
        <div className="treemap h-full flex-shrink-0">
          <Stack select={select} onSelect={setCursor} />
        </div>
        <div className="hexvewer">
          <HexView cursor={cursor} />
        </div>
      </div>
    </div>
  );
}

export default FrameList;
