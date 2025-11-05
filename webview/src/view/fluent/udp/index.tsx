import { useStore } from "../../store";
import { IUDPConversation } from "../../../share/gen";
import { createTableColumn, TableCellLayout, TableColumnDefinition } from "@fluentui/react-components";
import { compute, ComRequest, format_bytes_single_unit, formatMicroseconds } from "../../../share/common";
import { useState } from "react";
import Grid from "../table";
import { conversation_size } from "../../conf";

// import { useNavigate } from "react-router";
import { IPSelector, UDPTabIcon } from "../common";
import { BoxRegular, DesktopMacRegular, DocumentMultipleRegular, DocumentRegular, DocumentTextRegular } from "@fluentui/react-icons";
import Spark from "../overview/spark";

// import { PageFrame } from '../table';

const SIZE: "small" | "medium" = 'small';

const headIcon = (item: IUDPConversation) => {
    if (item && item.packets > 1) {
        return <DocumentMultipleRegular />
    }
    return <DocumentRegular />
}

function Component() {
    const conversations = useStore((state) => state.udpList);
    // const navigate = useNavigate();
    const [ip, setIp] = useState<string>('');
    const columns: TableColumnDefinition<IUDPConversation>[] = [
        createTableColumn<IUDPConversation>({
            columnId: "sender",
            renderHeaderCell: () => <><DesktopMacRegular /> Address A</>,
            renderCell: (item) => {
                return (
                    <TableCellLayout media={headIcon(item)}>
                        {item.sender + ':' + item.sender_port}
                    </TableCellLayout>
                );
            },
        }),
        createTableColumn<IUDPConversation>({
            columnId: "receiver",
            renderHeaderCell: () => <><DesktopMacRegular /> Address B</>,
            renderCell: (item) => {
                return (
                    <TableCellLayout>
                        {item.receiver + ':' + item.receiver_port}
                    </TableCellLayout>
                );
            },
        }),
        createTableColumn<IUDPConversation>({
            columnId: "count",
            renderHeaderCell: () => <><DocumentTextRegular /> Packets</>,
            renderCell: (item) => <TableCellLayout>{item.packets}</TableCellLayout>,
        }),
        createTableColumn<IUDPConversation>({
            columnId: "bytes",
            renderHeaderCell: () => <><BoxRegular /> Bytes</>,
            renderCell: (item) => <TableCellLayout>{format_bytes_single_unit(item.bytes)}</TableCellLayout>,
        }),
        createTableColumn<IUDPConversation>({
            columnId: "last",
            renderHeaderCell: () => <><BoxRegular /> Last Time</>,
            renderCell: (item) => {
                try {
                    const records = item.records;
                    if (records && records.length > 1) {
                        const end = records[records.length - 1][0];
                        const start = records[0][0];
                        return <TableCellLayout>{formatMicroseconds(start, end - start)}</TableCellLayout>
                    }
                } catch (e) {
                    console.error(e);
                }
                return <TableCellLayout>N/A</TableCellLayout>;
            },
        }),
        createTableColumn<IUDPConversation>({
            columnId: "grap",
            renderHeaderCell: () => <></>,
            renderCell: (item) => {
                try {
                    const records = item.records;
                    if (records && records.length > 2) {
                        return <Spark data={records} />
                    }
                } catch (e) {
                    console.error(e);
                }
                return <TableCellLayout>N/A</TableCellLayout>;
                
            },
        }),
    ];
    const pageSize = conversation_size;
    const load = async (page: number) => {
        const _ip = ip === 'ANY' ? '' : ip;
        const data: ComRequest = {
            catelog: "udp",
            type: "list",
            param: { ...compute(page, pageSize), ip: _ip },
        };
        return conversations(data);;
    }

    const breads = [
        { name: "UDP", icon: <UDPTabIcon />, path: "/udp" }
    ]
    const columnSizingOptions = {
        sender: {
            minWidth: 300,
            idealWidth: 300,
            autoFitColumns: true,
        },
        receiver: {
            minWidth: 300,
            idealWidth: 300,
            autoFitColumns: true,
        }

    };
    const gridProps = {
        filterComponent: (<>
            <IPSelector onSelect={setIp} />
        </>),
        size: SIZE,
        columns, pageSize, load, columnSizingOptions, breads
    };
    return <Grid {...gridProps} />;
}

export default Component;