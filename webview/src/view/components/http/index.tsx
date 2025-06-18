import { useEffect, useState } from "react";
import DTable from '../dataTable';
import { IListResult, IVHttpConnection } from "../../../share/gen";
import { useStore } from "../../store";
import { compute, ComRequest, HttpMessageWrap, MessageCompress } from "../../../share/common";
import { ColumnProps } from "primereact/column";
import { DataTableStateEvent } from "primereact/datatable";


interface HttpContentProps {
  conn: IVHttpConnection
}

const HttpContent = (props: HttpContentProps) => {
  const httpDetail = useStore((state) => state.httpDetail);
  const [list, setList] = useState<HttpMessageWrap[]>([]);
  useEffect(() => {
    httpDetail(props.conn).then((rs: MessageCompress[]) => {
      const list: HttpMessageWrap[] = rs.map((r: MessageCompress) => {
        const rt = JSON.parse(r.json);
        if (r.data.length > 0) {
          rt.raw = r.data;
        }
        return rt;
      });
      setList(list);
    });
  }, []);

  return (<div className="flex flex-nowrap h-full w-full">
  </div>
  );
}


const HttpConnectionList = () => {
  const size = 20;
  const httpConnections = useStore((state) => state.httpConnections);
  const [page, setPage] = useState<number>(1);
  const [loading, setLoading] = useState<boolean>(false);
  const [expandedRows, setExpandedRows] = useState<any[]>([]);
  const [result, setResult] = useState<IListResult<IVHttpConnection>>({
    start: 0,
    total: 0,
    items: [],
  });

  const mountHook = () => {
    setLoading(true);
    const data: ComRequest = {
      catelog: "http_connection",
      type: "list",
      param: { ...compute(page, size) },
    };
    console.log('request', data);
    httpConnections(data).then((rs: IListResult<IVHttpConnection>) => {
      setResult(rs);
      setLoading(false);
    })

  };
  useEffect(mountHook, [page]);

  const rowExpansionTemplate = (data: IVHttpConnection) => {
    return <HttpContent conn={data} />
  };
  const onRowToggle = (e: any) => {
    const d: any[] = e.data;
    setExpandedRows(d)
  }
  const _status = (item: IVHttpConnection): string => {
    if (item.response) {
      let ss = item.response.split(' ');
      if (ss && ss.length > 1) {
        return ss[1];
      }
    }
    return 'N/A';
  }
  const _method = (item: IVHttpConnection): string => {
    if (item.request) {
      let ss = item.request.split(' ');
      if (ss && ss.length > 1) {
        return ss[1];
      }
    }
    return 'N/A';
  }
  const _host = (item: IVHttpConnection): string => {
    if (item.request) {
      let ss = item.request.split(' ');
      if (ss && ss.length > 1) {
        return ss[1];
      }
    }
    return 'N/A';
  }

  const columes: ColumnProps[] = [
    { expander: true, field: "_inx", header: "", style: { width: "25px" } },
    { field: "response", header: "Status", body: _status },
    { field: "request", header: "Method", body: _method },
    { field: "request", header: "Host", body: _host },
    { field: "length", header: "Length" },
    { field: "content_type", header: "ContentType" },
    { field: "rt", header: "time" },
  ];
  const onPage = (event: DataTableStateEvent) => {
    const { page } = event;
    setPage(page!);
  }
  return (<div className="flex flex-nowrap h-full w-full">
    <DTable loading={loading} showGridlines={false} rowExpansionTemplate={rowExpansionTemplate} onRowToggle={onRowToggle} expandedRows={expandedRows} cols={columes} result={{ items: result.items, page, size, total: result.total }} onPage={onPage} />
  </div>
  );
}


export default HttpConnectionList;