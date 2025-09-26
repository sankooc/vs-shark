import { useStore } from "../../store";
import { IVConversation } from "../../../share/gen";
import { createTableColumn, TableCellLayout, TableColumnDefinition } from "@fluentui/react-components";
import { compute, ComRequest, format_bytes_single_unit } from "../../../share/common";

import Grid from "../grid";
import { conversation_size } from "../../conf";

import { useNavigate } from "react-router";
import { ConversationIcon } from "../common";
import { BoxRegular, DesktopMacRegular, DocumentTextRegular, FolderListRegular } from "@fluentui/react-icons";

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
            renderHeaderCell: () =>  <><FolderListRegular /> TX Packets</>,
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
        { name: "Conversations", icon: <ConversationIcon/>, path: "/conversations" }
    ]
    return <div className="flex flex-column h-full" style={{ overflowX: "hidden", overflowY: "auto" }}>
        <Grid columns={columns} onClick={onClick} pageSize={pageSize} load={load} breads={breads} />
    </div>
}

export default Component;