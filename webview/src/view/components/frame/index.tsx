import { useEffect } from "react";
import DTable from "../dataTable2.tsx";
import { useStore } from "../../store";
import { PaginatorPageChangeEvent } from "primereact/paginator";
import { ComRequest, Pagination } from "../../../core/common.ts";
import { IFrameInfo, IListResult } from "../../../core/gen.ts";
import { ColumnProps } from "primereact/column";
import { Tooltip } from "primereact/tooltip";
import dayjs from "dayjs";

const PAGE_SIZE = 500;
function FrameList() {
  // const [filter, setFilter] = useState<any[]>([]);
  // const [options, setOptions] = useState<string[]>([]);
  const _request = useStore((state) => state.request);
  const frameResult: IListResult<IFrameInfo> = useStore(
    (state) => state.frameResult,
  ) || { start: 0, total: 0, items: [] };
  const { items, start, total } = frameResult;
  const page = Math.floor(start / PAGE_SIZE) + 1;
  const size = PAGE_SIZE;
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
    _request(data);
  };
  useEffect(mountHook, []);
  const columes: ColumnProps[] = [
    {
      field: "index",
      header: "index",
      style: { width: "4rem" },
      body: (item: any) => item.index + 1,
    },
    {
      field: "time",
      header: "time",
      style: { width: "7rem" },
      body: (item: any) => {
        const ts = Math.round(item.time / 1000);
        const trimts = item.time % 1000000000;
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
              {trimts}
            </span>
          </>
        );
      },
      // headerTooltip: 'headerTooltip',
    },
    { field: "source", header: "source", style: { width: "17.5rem" } },
    { field: "dest", header: "dest", style: { width: "17.5rem" } },
    { field: "protocol", header: "protocol", style: { width: "6rem" } },
    { field: "len", header: "len", style: { width: "5.5rem" } },
    { field: "info", header: "info" },
  ];
  const onSelect = (_item: any): void => {
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
    const { rows, page } = event;
    const data: ComRequest = {
      catelog: "frame",
      type: "list",
      param: compute(page + 1, rows),
    };
    _request(data);
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
          result={{ items, page, size, total }}
          getStyle={getStyle}
          onSelect={onSelect}
          request={request}
          scrollHeight={70}
        />
      </div>
    </div>
  );
}

export default FrameList;
