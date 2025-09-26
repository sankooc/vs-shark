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

// const useCSS = makeStyles({
//   pagnation: {
//     textAlign: "right",
//     overflow: "hidden",
//     padding: "5px 10px",
//     fontSize: "0.5em",
//     flexShrink: 0,
//   },
//   icon: {
//     minWidth: "1.7em",
//     padding: "0",
//     border: "none",
//   },
//   iconSelect: {
//     minWidth: "1.7em",
//     backgroundColor: "#344",
//     padding: "0",
//   }
// });

// const NextIcon = bundleIcon(TriangleRight20Filled, TriangleRight20Regular);
// const PrevIcon = bundleIcon(TriangleLeft20Filled, TriangleLeft20Regular);
// interface PageProps {
//   page: number;
//   total: number;
//   pageSize: number;
//   onPageChange: (page: number) => void
// }
// function Paging(props: PageProps) {
//   const styles = useCSS();
//   const hasTotal = props.total >= 0;

//   if (hasTotal) {
//     const max = Math.ceil(props.total / props.pageSize);
//     const start = Math.max(1, props.page - 2);
//     const end = Math.min(max, props.page + 2);

//     const pages = [];
//     for (let i = start; i <= end; i++) {
//       pages.push(i);
//     }
//     return <div className={styles.pagnation}>
//       {props.page > 1 && <Button appearance="transparent" onClick={() => { props.onPageChange(props.page - 1) }} className={styles.icon} icon={<PrevIcon />}> </Button>}
//       {pages.map((page) => (<Button key={page} shape="circular" onClick={() => { props.onPageChange(page) }} className={page == props.page ? styles.iconSelect : styles.icon}>{page}</Button>))}
//       {props.page < max && <Button appearance="transparent" onClick={() => { props.onPageChange(props.page + 1) }}className={styles.icon} icon={<NextIcon />}> </Button>}
//     </div>

//   } else {
//     return <div className={styles.pagnation}>
//        {props.page > 1 && <Button appearance="transparent" className={styles.icon} icon={<PrevIcon />}> </Button>}
//       <Button appearance="transparent" className={styles.icon} icon={<NextIcon />}> </Button>
//     </div>
//   }
// }

function Empty() {
  return <div className="w-full" style={{ padding: "10px" }}>no content</div>
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
      return <div className="flex flex-column" style={{ height: height + "px", width: width + "px" }}>
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

