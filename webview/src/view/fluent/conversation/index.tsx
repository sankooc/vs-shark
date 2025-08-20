import { useState } from "react";
import { useStore } from "../../store";
import { IVConnection, IVConversation } from "../../../share/gen";
import { Button, createTableColumn, Drawer, DrawerBody, DrawerHeader, DrawerHeaderTitle, TableCellLayout, TableColumnDefinition } from "@fluentui/react-components";
import { compute, ComRequest, format_bytes_single_unit } from "../../../share/common";
import { Dismiss24Regular } from "@fluentui/react-icons";
import Grid from "../grid";
import { connect_size, conversation_size } from "../../conf";

class ConnectProp {
    conversationIndex!: number;
}
const ConnectionList = (props: ConnectProp) => {
    const pageSize = connect_size;
    const connections = useStore((state) => state.connections);
    const columns: TableColumnDefinition<IVConnection>[] = [
        createTableColumn<IVConnection>({
            columnId: "protocol",
            renderHeaderCell: () => {
                return "Protocol";
            },
            renderCell: (item) => {
                return (
                    <TableCellLayout>
                        {item.protocol}
                    </TableCellLayout>
                );
            },
        }),
        createTableColumn<IVConnection>({
            columnId: "primary.port",
            renderHeaderCell: () => {
                return "Sender";
            },
            renderCell: (item) => {
                return (
                    <TableCellLayout>
                        {item.primary.port}
                    </TableCellLayout>
                );
            },
        }),
        createTableColumn<IVConnection>({
            columnId: "second.port",
            renderHeaderCell: () => {
                return "Receiver";
            },
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
            renderHeaderCell: () => {
                return "TX-Bytes";
            },
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
            renderHeaderCell: () => {
                return "TX-Packets";
            },
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
            renderHeaderCell: () => {
                return "TX-Used";
            },
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
            renderHeaderCell: () => {
                return "RX-Bytes";
            },
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
            renderHeaderCell: () => {
                return "RX-Packets";
            },
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
            renderHeaderCell: () => {
                return "RX-Used";
            },
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
            param: { ...compute(page, pageSize), conversionIndex: props.conversationIndex },
          };
          return connections(data)
    }

    return <div className="flex flex-column h-full" style={{ overflowX: "hidden", overflowY: "auto" }}>
        <Grid columns={columns} onClick={onClick} pageSize={pageSize} load={load} />
    </div>
}

class ConversationProp {
    onClick?: (item: IVConversation) => void;
}

function ConversationList (props: ConversationProp) {
    const conversations = useStore((state) => state.conversations);
    const pageSize = conversation_size;
    const columns: TableColumnDefinition<IVConversation>[] = [
        createTableColumn<IVConversation>({
            columnId: "sender",
            renderHeaderCell: () => {
                return "Sender";
            },
            renderCell: (item) => {
                return (
                    <TableCellLayout>
                        {item.sender}
                    </TableCellLayout>
                );
            },
        }),
        createTableColumn<IVConversation>({
            columnId: "receiver",
            renderHeaderCell: () => {
                return "Receiver";
            },
            renderCell: (item) => {
                return (
                    <TableCellLayout>
                        {item.receiver}
                    </TableCellLayout>
                );
            },
        }),
        createTableColumn<IVConversation>({
            columnId: "connects",
            renderHeaderCell: () => {
                return "Connections";
            },

            renderCell: (item) => {
                return item.connects;
            },
        }),
        createTableColumn<IVConversation>({
            columnId: "sender_packets",
            renderHeaderCell: () => {
                return "RX Packets";
            },

            renderCell: (item) => {
                return item.sender_packets;
            },
        }),
        createTableColumn<IVConversation>({
            columnId: "receiver_packets",
            renderHeaderCell: () => {
                return "TX Packets";
            },

            renderCell: (item) => {
                return item.receiver_packets;
            },
        }),
        createTableColumn<IVConversation>({
            columnId: "sender_bytes",
            renderHeaderCell: () => {
                return "RX Bytes";
            },

            renderCell: (item) => {
                return format_bytes_single_unit(item.sender_bytes);
            },
        }),
        createTableColumn<IVConversation>({
            columnId: "receiver_bytes",
            renderHeaderCell: () => {
                return "TX Bytes";
            },

            renderCell: (item) => {
                return format_bytes_single_unit(item.receiver_bytes);
            },
        }),
    ];
    const onClick = (item: IVConversation) => {
        // setOpen(true);
        // setSelect(item.key);
    };
    const load = async (page: number) => {
        const data: ComRequest = {
            catelog: "conversation",
            type: "list",
            param: compute(page, pageSize),
        };
        return conversations(data);
    }
    <Grid columns={columns} onClick={onClick} pageSize={pageSize} load={load} />
}

function Component() {
    const conversations = useStore((state) => state.conversations);
    const [select, setSelect] = useState<number | undefined>(undefined);
    const [open, setOpen] = useState<boolean>(false);
    const columns: TableColumnDefinition<IVConversation>[] = [
        createTableColumn<IVConversation>({
            columnId: "sender",
            renderHeaderCell: () => {
                return "Sender";
            },
            renderCell: (item) => {
                return (
                    <TableCellLayout>
                        {item.sender}
                    </TableCellLayout>
                );
            },
        }),
        createTableColumn<IVConversation>({
            columnId: "receiver",
            renderHeaderCell: () => {
                return "Receiver";
            },
            renderCell: (item) => {
                return (
                    <TableCellLayout>
                        {item.receiver}
                    </TableCellLayout>
                );
            },
        }),
        createTableColumn<IVConversation>({
            columnId: "connects",
            renderHeaderCell: () => {
                return "Connections";
            },

            renderCell: (item) => {
                return item.connects;
            },
        }),
        // createTableColumn<IVConversation>({
        //     columnId: "packets",
        //     renderHeaderCell: () => {
        //         return "Packets";
        //     },

        //     renderCell: (item) => {
        //         return item.sender_packets + item.receiver_packets;
        //     },
        // }),
        // createTableColumn<IVConversation>({
        //     columnId: "bytes",
        //     renderHeaderCell: () => {
        //         return "Bytes";
        //     },

        //     renderCell: (item) => {
        //         return format_bytes_single_unit(item.sender_bytes + item.receiver_bytes);
        //     },
        // }),
        createTableColumn<IVConversation>({
            columnId: "sender_packets",
            renderHeaderCell: () => {
                return "RX Packets";
            },

            renderCell: (item) => {
                return item.sender_packets;
            },
        }),
        createTableColumn<IVConversation>({
            columnId: "receiver_packets",
            renderHeaderCell: () => {
                return "TX Packets";
            },

            renderCell: (item) => {
                return item.receiver_packets;
            },
        }),
        createTableColumn<IVConversation>({
            columnId: "sender_bytes",
            renderHeaderCell: () => {
                return "RX Bytes";
            },

            renderCell: (item) => {
                return format_bytes_single_unit(item.sender_bytes);
            },
        }),
        createTableColumn<IVConversation>({
            columnId: "receiver_bytes",
            renderHeaderCell: () => {
                return "TX Bytes";
            },

            renderCell: (item) => {
                return format_bytes_single_unit(item.receiver_bytes);
            },
        }),
    ];
    const onClick = (item: IVConversation) => {
        setOpen(true);
        setSelect(item.key);
    };
    const pageSize = conversation_size;
    const load = async (page: number) => {
        const data: ComRequest = {
            catelog: "conversation",
            type: "list",
            param: compute(page, pageSize),
        };
        return conversations(data);
    }

    return <div className="flex flex-column h-full" style={{ overflowX: "hidden", overflowY: "auto" }}>
        <Grid columns={columns} onClick={onClick} pageSize={pageSize} load={load} />
        <Drawer
            type="overlay"
            separator
            open={open}
            position="bottom"
            size="full"
            modalType="non-modal"
        >
            <DrawerHeader>
                <DrawerHeaderTitle
                    action={
                        <Button
                            appearance="subtle"
                            aria-label="Close"
                            icon={<Dismiss24Regular />}
                            onClick={() => setOpen(false)}
                        />
                    }
                >
                </DrawerHeaderTitle>
            </DrawerHeader>

            <DrawerBody>
                {select !== undefined && <ConnectionList conversationIndex={select} />}
            </DrawerBody>
        </Drawer>
    </div>
}

export default Component;