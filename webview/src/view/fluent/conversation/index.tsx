import { useStore } from "../../store";
import { IVConversation } from "../../../share/gen";
import { createTableColumn, TableCellLayout, TableColumnDefinition, Toolbar, ToolbarButton } from "@fluentui/react-components";
import { compute, ComRequest, format_bytes_single_unit } from "../../../share/common";
import { useState } from "react";
import Grid from "../table";
import { conversation_size } from "../../conf";

import { useNavigate } from "react-router";
import { ConversationIcon, IPSelector } from "../common";
import { BoxRegular, DesktopMacRegular, DocumentMultipleRegular, DocumentRegular, DocumentTextRegular, FolderListRegular, MoreHorizontalFilled, TextBulletListSquareColor } from "@fluentui/react-icons";

// import { PageFrame } from '../table';

const SIZE: "small" | "medium" = 'small';

const headIcon = (item: IVConversation) => {
    if(item && item.connects > 1){
        return <DocumentMultipleRegular/>
    }
    return <DocumentRegular />
}

function Component() {
    const conversations = useStore((state) => state.conversations);
    const navigate = useNavigate();
    const [ ip, setIp ] = useState<string>('');
    const columns: TableColumnDefinition<IVConversation>[] = [
        createTableColumn<IVConversation>({
            columnId: "sender",
            renderHeaderCell: () => <><DesktopMacRegular /> Sender</>,
            renderCell: (item) => {
                return (
                    <TableCellLayout media={headIcon(item)}>
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
            renderCell: (item) => <TableCellLayout>{item.connects}</TableCellLayout>,
        }),
        createTableColumn<IVConversation>({
            columnId: "sender_packets",
            renderHeaderCell: () => <><FolderListRegular /> RX Packets</>,
            renderCell: (item) => <TableCellLayout>{item.sender_packets}</TableCellLayout>,
        }),
        createTableColumn<IVConversation>({
            columnId: "receiver_packets",
            renderHeaderCell: () => <><FolderListRegular /> TX Packets</>,
            renderCell: (item) => <TableCellLayout>{item.receiver_packets}</TableCellLayout>,
        }),
        createTableColumn<IVConversation>({
            columnId: "sender_bytes",
            renderHeaderCell: () => <><BoxRegular /> RX Bytes</>,
            renderCell: (item) => <TableCellLayout>{format_bytes_single_unit(item.sender_bytes)}</TableCellLayout>,
        }),
        createTableColumn<IVConversation>({
            columnId: "receiver_bytes",
            renderHeaderCell: () => <><BoxRegular /> TX Bytes</>,
            renderCell: (item) => <TableCellLayout>{format_bytes_single_unit(item.receiver_bytes)}</TableCellLayout>,
        }),
        createTableColumn<IVConversation>({
            columnId: "ops",
            renderHeaderCell: () => "ext",
            renderCell: (item) => {
                return <TableCellLayout><Toolbar aria-label="Default" size="small">
                    <ToolbarButton icon={<TextBulletListSquareColor />} onClick={() => { onClick(item) }} />
                    <ToolbarButton icon={<MoreHorizontalFilled />} />
                </Toolbar></TableCellLayout>
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
        const _ip = ip === 'ANY' ? '' : ip;
        const data: ComRequest = {
            catelog: "conversation",
            type: "list",
            param:{ ...compute(page, pageSize), ip: _ip },
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
        filterComponent: (<>
            <IPSelector onSelect={setIp} />
        </>),
        size: SIZE,
        columns, pageSize, load, columnSizingOptions, breads
    };
    return <Grid {...gridProps} />;
}

export default Component;