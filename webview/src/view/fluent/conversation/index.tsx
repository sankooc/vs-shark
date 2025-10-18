import { useStore } from "../../store";
import { IVConversation } from "../../../share/gen";
import { createTableColumn, TableCellLayout, TableColumnDefinition, Toolbar, ToolbarButton } from "@fluentui/react-components";
import { compute, ComRequest, format_bytes_single_unit } from "../../../share/common";

import Grid from "../table";
import { conversation_size } from "../../conf";

import { useNavigate } from "react-router";
import { ConversationIcon } from "../common";
import { BoxRegular, DesktopMacRegular, DocumentTextRegular, FolderListRegular, MoreHorizontalFilled, TextBulletListSquareColor } from "@fluentui/react-icons";

// import { PageFrame } from '../table';

const SIZE: "small" | "medium" = 'small';

function Component() {
    const conversations = useStore((state) => state.conversations);
    const navigate = useNavigate();
    const columns: TableColumnDefinition<IVConversation>[] = [
        createTableColumn<IVConversation>({
            columnId: "sender",
            renderHeaderCell: () => <><DesktopMacRegular /> Sender</>,
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
            renderHeaderCell: () => <><DesktopMacRegular /> Receiver</>,
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
            renderHeaderCell: () => <><DocumentTextRegular /> Connections</>,
            renderCell: (item) => {
                return item.connects;
            },
        }),
        createTableColumn<IVConversation>({
            columnId: "sender_packets",
            renderHeaderCell: () => <><FolderListRegular /> RX Packets</>,
            renderCell: (item) => {
                return item.sender_packets;
            },
        }),
        createTableColumn<IVConversation>({
            columnId: "receiver_packets",
            renderHeaderCell: () => <><FolderListRegular /> TX Packets</>,
            renderCell: (item) => {
                return item.receiver_packets;
            },
        }),
        createTableColumn<IVConversation>({
            columnId: "sender_bytes",
            renderHeaderCell: () => <><BoxRegular /> RX Bytes</>,
            renderCell: (item) => {
                return format_bytes_single_unit(item.sender_bytes);
            },
        }),
        createTableColumn<IVConversation>({
            columnId: "receiver_bytes",
            renderHeaderCell: () => <><BoxRegular /> TX Bytes</>,
            renderCell: (item) => {
                return format_bytes_single_unit(item.receiver_bytes);
            },
        }),
        createTableColumn<IVConversation>({
            columnId: "ops",
            renderHeaderCell: () => "ext",
            renderCell: (item) => {
                return <Toolbar aria-label="Default" size="small">
                    <ToolbarButton icon={<TextBulletListSquareColor />} onClick={() => { onClick(item) }} />
                    <ToolbarButton icon={<MoreHorizontalFilled />} />
                </Toolbar>
                // return <TableCellLayout media={<TextBulletListSquareColor />} style={{cursor: 'pointer'}}></TableCellLayout>
            },
        }),
    ];
    const onClick = (item: IVConversation) => {
        const title = `${item.sender} / ${item.receiver}`;
        navigate('/conversation/' + item.key, { state: { title } });
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

    const breads = [
        { name: "Conversations", icon: <ConversationIcon />, path: "/conversations" }
    ]
    const columnSizingOptions = {
        sender: {
            minWidth: 200,
            idealWidth: 250,
            autoFitColumns: true,
        },
        receiver: {
            minWidth: 200,
            idealWidth: 250,
            autoFitColumns: true,
        }

    };
    const gridProps = {
        size: SIZE,
        columns, pageSize, load, columnSizingOptions, breads
    };
    return <Grid {...gridProps} />;
}

export default Component;