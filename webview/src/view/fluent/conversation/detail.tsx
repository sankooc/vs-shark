
import { usePcapStore } from "../../../share/context";
import { IVConnection} from "../../../share/gen";
import { createTableColumn, TableCellLayout, TableColumnDefinition } from "@fluentui/react-components";
import { compute, ComRequest, format_bytes_single_unit } from "../../../share/common";
import Grid from "../table";
import { connect_size } from "../../conf";
import { useParams, useLocation } from "react-router";
import { ConversationIcon, protocolText } from "../common";
import { DesktopMacRegular, TextBulletListSquareRegular, UsbPlugFilled } from "@fluentui/react-icons";


const SIZE: "small" | "medium" = 'small';

export default function Component() {
    const { conversationIndex } = useParams();
    const location = useLocation();
    const title = location.state?.title || "detail";
    const pageSize = connect_size;
    const connections = usePcapStore((state) => state.connectionList);
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
            columnId: "host",
            renderHeaderCell: () => <><DesktopMacRegular /> Address</>,
            renderCell: (item) => {
                return (
                    <TableCellLayout>
                        {item.primary.host + "/" + item.second.host}
                    </TableCellLayout>
                );
            },
        }),
        createTableColumn<IVConnection>({
            columnId: "primary_port",
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
            columnId: "second_port",
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
            renderHeaderCell: () => <>Bytes(TX/RX)</>,
            renderCell: (item) => {
                return (
                    <TableCellLayout>
                        {format_bytes_single_unit(item.primary.statistic.throughput) + "/" + format_bytes_single_unit(item.second.statistic.throughput)}
                    </TableCellLayout>
                );
            },
        }),
        createTableColumn<IVConnection>({
            columnId: "primary.statistic.count",
            renderHeaderCell: () => <>Packets(TX/RX)</>,
            renderCell: (item) => {
                return (
                    <TableCellLayout>
                        {item.primary.statistic.count + "/" + item.second.statistic.count}
                    </TableCellLayout>
                );
            },
        }),
        createTableColumn<IVConnection>({
            columnId: "primary.statistic.clean_throughput",
            renderHeaderCell: () => <>TCP(TX/RX)</>,
            renderCell: (item) => {
                return (
                    <TableCellLayout>
                        {format_bytes_single_unit(item.primary.statistic.clean_throughput) + '/' + format_bytes_single_unit(item.second.statistic.clean_throughput)}
                    </TableCellLayout>
                );
            },
        }),
    ];
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

    const gridProps = {
        size: SIZE,
        columns, pageSize, load, breads
    };
    return <Grid {...gridProps} />;
    // return <div className="flex flex-column h-full" style={{ overflowX: "hidden", overflowY: "auto" }}>
    //     <Grid size="small" columns={columns} onClick={onClick} pageSize={pageSize} load={load} breads={breads} />
    // </div>
}