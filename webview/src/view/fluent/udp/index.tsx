import { useStore } from "../../store";
import { IUDPConversation } from "../../../share/gen";
import { createTableColumn, TableCellLayout, TableColumnDefinition } from "@fluentui/react-components";
import { compute, ComRequest, format_bytes_single_unit, formatMicroseconds } from "../../../share/common";
import { useState } from "react";
import Grid, { SortState } from "../table";

// import { useNavigate } from "react-router";
import { IPSelector, TimeIcon, UDPTabIcon } from "../common";
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
    // const tableFeature = useTableSort({});

    // const [sortState, setSortState] = useState<TableSortState<IUDPConversation>>({
    //     sortColumn: "time",
    //     sortDirection: "ascending",
    // });
    // const navigate = useNavigate();
    const [sortState, setSortState] = useState<SortState>({
        sortColumn: 'time',
        sortDirection: 'ascending'
    })
    const [ip, setIp] = useState<string>('');
    const columns: TableColumnDefinition<IUDPConversation>[] = [
        createTableColumn<IUDPConversation>({
            columnId: "time",
            renderHeaderCell: TimeIcon,
            compare: (_a, _b) => {
                return 0;
            },
            renderCell: (item) => {
                let content = item.ts_str;
                if (item.offset_str) {
                    content = content + ` (+${item.offset_str[0]}${item.offset_str[1]})`
                }
                return <TableCellLayout>{content}</TableCellLayout>
            },
        }),
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
    const pageSize = 30;
    const load = async (page: number) => {
        const _ip = ip === 'ANY' ? '' : ip;
        const asc = sortState.sortDirection === 'ascending';
        const data: ComRequest = {
            catelog: "udp",
            type: "list",
            param: { ...compute(page, pageSize), ip: _ip, asc },
        };
        return conversations(data);;
    }

    const breads = [
        { name: "UDP", icon: <UDPTabIcon />, path: "/udp" }
    ]
    const columnSizingOptions = {
        time: {
            minWidth: 300,
            idealWidth: 300,
        },
        sender: {
            minWidth: 200,
            idealWidth: 200,
        },
        receiver: {
            minWidth: 200,
            idealWidth: 200,
        }
    };
    const gridProps = {
        filterComponent: (<>
            <IPSelector onSelect={setIp} />
        </>),
        size: SIZE,
        sortState,
        onSortChange: (_e: Event, state: SortState) => {
            setSortState(state);
        },
        columns, pageSize, load, columnSizingOptions, breads
    };
    return <Grid {...gridProps} />;
}

export default Component;