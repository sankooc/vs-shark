import { useEffect, useState } from "react";
import DTable from '../dataTable';
import { IListResult, IVConnection, IVConversation } from "../../../share/gen";
import { useStore } from "../../store";
import { compute, ComRequest, format_bytes_single_unit } from "../../../share/common";
import { ColumnProps } from "primereact/column";
import { DataTableStateEvent } from "primereact/datatable";


class ConnectProper {
  conversationIndex!: number;
}
const ConnectionList = (props: ConnectProper) => {
  const size = 20;
  const connections = useStore((state) => state.connections);
  const [page, setPage] = useState<number>(1);
  const [loading, setLoading] = useState<boolean>(false);
  const [result, setResult] = useState<IListResult<IVConnection>>({
    start: 0,
    total: 0,
    items: [],
  });

  const mountHook = () => {
    setLoading(true);
    const data: ComRequest = {
      catelog: "connection",
      type: "list",
      param: { ...compute(page, size), conversionIndex: props.conversationIndex },
    };
    console.log('request', data);
    connections(data).then((rs: IListResult<IVConnection>) => {
      setResult(rs);
      setLoading(false);
    })

  };
  useEffect(mountHook, [page]);

  const columes: ColumnProps[] = [
    { field: "protocol", header: "Protocol" },
    { field: "primary.port", header: "sender" },
    { field: "second.port", header: "receiver" },
    { field: "primary.statistic.throughput", header: "TX-Bytes", body: (item: IVConnection) => format_bytes_single_unit(item.primary.statistic.throughput) },
    { field: "primary.statistic.count", header: "TX-Packets" },
    { field: "primary.statistic.clean_throughput", header: "TX-Used", body: (item: IVConnection) => format_bytes_single_unit(item.primary.statistic.clean_throughput) },
    { field: "second.statistic.throughput", header: "RX-Bytes", body: (item: IVConnection) => format_bytes_single_unit(item.second.statistic.throughput) },
    { field: "second.statistic.count", header: "RX-Packets" },
    { field: "second.statistic.clean_throughput", header: "RX-Used", body: (item: IVConnection) => format_bytes_single_unit(item.second.statistic.clean_throughput) },

  ];
  const onPage = (event: DataTableStateEvent) => {
    console.log('page', event);
    setPage(page!);
  }
  return (<div className="flex flex-nowrap h-full w-full">
    <DTable loading={loading} showGridlines cols={columes} result={{ items: result.items, page, size, total: result.total }} onPage={onPage} />
  </div>
  );
}
const ConversationList = () => {
  const size = 20;
  const conversations = useStore((state) => state.conversations);
  const [page, setPage] = useState<number>(1);
  const [loading, setLoading] = useState<boolean>(false);
  const [expandedRows, setExpandedRows] = useState<any[]>([]);
  const [result, setResult] = useState<IListResult<IVConversation>>({
    start: 0,
    total: 0,
    items: [],
  });
  const rowExpansionTemplate = (data: IVConversation) => {
    const index = data.key;
    return <ConnectionList conversationIndex={index}/>
  };
  const onRowToggle = (e: any) => {
    const d: any[] = e.data;
    setExpandedRows(d)
  }
  const mountHook = () => {
    setLoading(true);
    const data: ComRequest = {
      catelog: "conversation",
      type: "list",
      param: compute(page, size),
    };
    console.log('request', data);
    conversations(data).then((rs: IListResult<IVConversation>) => {
      setResult(rs);
      setLoading(false);
    })

  };
  useEffect(mountHook, [page]);

  const columes: ColumnProps[] = [
    { expander: true, field: "_inx", header: "", style: { width: "25px" } },
    { field: "sender", header: "sender" },
    { field: "receiver", header: "receiver" },
    { field: "connects", header: "connections" },
    { field: "packets", header: "packets", body: (item: IVConversation) => item.sender_packets + item.receiver_packets },
    { field: "bytes", header: "bytes", body: (item: IVConversation) => format_bytes_single_unit(item.sender_bytes + item.receiver_bytes) },
    { field: "sender_packets", header: "RX Packets" },
    { field: "receiver_packets", header: "TX Packets" },
    { field: "sender_bytes", header: "RX Bytes", body: (item: IVConversation) => format_bytes_single_unit(item.sender_bytes) },
    { field: "receiver_bytes", header: "TX Bytes", body: (item: IVConversation) => format_bytes_single_unit(item.receiver_bytes) },
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

export default ConversationList;