import { useStore } from "../../store";
import { IVConnection} from "../../../share/gen";
import { createTableColumn, TableCellLayout, TableColumnDefinition } from "@fluentui/react-components";
import { compute, ComRequest, format_bytes_single_unit } from "../../../share/common";
import Grid from "../grid";
import { connect_size } from "../../conf";
import { useParams, useLocation } from "react-router";
import { ConversationIcon, protocolText } from "../common";
import { ArrowDownloadRegular, ArrowUploadRegular, DesktopMacRegular, TextBulletListSquareRegular, UsbPlugFilled } from "@fluentui/react-icons";

export default function Component() {
    const { conversationIndex } = useParams();
    const location = useLocation();
    const title = location.state?.title || "detail";
    const pageSize = connect_size;
    const connections = useStore((state) => state.connections);
    const columns: TableColumnDefinition<IVConnection>[] = [
        createTableColumn<IVConnection>({
            columnId: "protocol",
            renderHeaderCell: () => <><TextBulletListSquareRegular />Protocol</>,
            renderCell: (item) => {
                return (
                    <TableCellLayout>
                        {protocolText(item.protocol)}
                    </TableCellLayout>
                );
            },
        }),
        createTableColumn<IVConnection>({
            columnId: "primary.host",
            renderHeaderCell: () => <><DesktopMacRegular /> Sender</>,
            renderCell: (item) => {
                return (
                    <TableCellLayout>
                        {item.primary.host}
                    </TableCellLayout>
                );
            },
        }),
        createTableColumn<IVConnection>({
            columnId: "primary.port",
            renderHeaderCell: () => <><UsbPlugFilled /> S-port</>,
            renderCell: (item) => {
                return (
                    <TableCellLayout>
                        {item.primary.port}
                    </TableCellLayout>
                );
            },
        }),
        createTableColumn<IVConnection>({
            columnId: "second.host",
            renderHeaderCell: () => <><DesktopMacRegular /> Receiver</>,
            renderCell: (item) => {
                return (
                    <TableCellLayout>
                        {item.second.host}
                    </TableCellLayout>
                );
            },
        }),
        createTableColumn<IVConnection>({
            columnId: "second.port",
            renderHeaderCell: () => <><UsbPlugFilled /> R-port</>,
            renderCell: (item) => {
                return (
                    <TableCellLayout>
                        {item.second.port}
                    </TableCellLayout>
                );
            },
        }),
        createTableColumn<IVConnection>({
            columnId: "primary.statistic.throughput",
            renderHeaderCell: () => <><ArrowUploadRegular />Bytes</>,
            renderCell: (item) => {
                return (
                    <TableCellLayout>
                        {format_bytes_single_unit(item.primary.statistic.throughput)}
                    </TableCellLayout>
                );
            },
        }),
        createTableColumn<IVConnection>({
            columnId: "primary.statistic.count",
            renderHeaderCell: () => <><ArrowUploadRegular />TX-Packets</>,
            renderCell: (item) => {
                return (
                    <TableCellLayout>
                        {item.primary.statistic.count}
                    </TableCellLayout>
                );
            },
        }),
        createTableColumn<IVConnection>({
            columnId: "primary.statistic.clean_throughput",
            renderHeaderCell: () => <><ArrowUploadRegular />TX-Used</>,
            renderCell: (item) => {
                return (
                    <TableCellLayout>
                        {format_bytes_single_unit(item.primary.statistic.clean_throughput)}
                    </TableCellLayout>
                );
            },
        }),
        createTableColumn<IVConnection>({
            columnId: "second.statistic.throughput",
            renderHeaderCell: () => <><ArrowDownloadRegular />RX-Bytes</>,
            renderCell: (item) => {
                return (
                    <TableCellLayout>
                        {format_bytes_single_unit(item.second.statistic.throughput)}
                    </TableCellLayout>
                );
            },
        }),
        createTableColumn<IVConnection>({
            columnId: "second.statistic.count",
            renderHeaderCell: () => <><ArrowDownloadRegular />RX-Packet</>,
            renderCell: (item) => {
                return (
                    <TableCellLayout>
                        {item.second.statistic.count}
                    </TableCellLayout>
                );
            },
        }),
        createTableColumn<IVConnection>({
            columnId: "second.statistic.clean_throughput",
            renderHeaderCell: () => <><ArrowDownloadRegular />RX-Used</>,
            renderCell: (item) => {
                return (
                    <TableCellLayout>
                        {format_bytes_single_unit(item.second.statistic.clean_throughput)}
                    </TableCellLayout>
                );
            },
        }),
    ];
    const onClick = (_item: IVConnection) => {
    };
    const load = async (page: number) => {
        const data: ComRequest = {
            catelog: "connection",
            type: "list",
            param: { ...compute(page, pageSize), conversionIndex: conversationIndex },
        };
        return connections(data)
    }

    const breads = [
        { name: "Conversations",icon: <ConversationIcon/>, path: "/conversations" },
        { name: title },
    ]
    return <div className="flex flex-column h-full" style={{ overflowX: "hidden", overflowY: "auto" }}>
        <Grid columns={columns} onClick={onClick} pageSize={pageSize} load={load} breads={breads} />
    </div>
}