import { useStore } from "../../store";
import { IUDPConversation } from "../../../share/gen";
import { createTableColumn, TableCellLayout, TableColumnDefinition } from "@fluentui/react-components";
import { compute, ComRequest, format_bytes_single_unit } from "../../../share/common";
import { useState } from "react";
import Grid from "../table";
import { conversation_size } from "../../conf";

// import { useNavigate } from "react-router";
import { ConversationIcon, IPSelector } from "../common";
import { BoxRegular, DesktopMacRegular, DocumentTextRegular} from "@fluentui/react-icons";

// import { PageFrame } from '../table';

const SIZE: "small" | "medium" = 'small';

function Component() {
    const conversations = useStore((state) => state.udps);
    // const navigate = useNavigate();
    const [ ip, setIp ] = useState<string>('');
    const columns: TableColumnDefinition<IUDPConversation>[] = [
        createTableColumn<IUDPConversation>({
            columnId: "sender",
            renderHeaderCell: () => <><DesktopMacRegular /> Address A</>,
            renderCell: (item) => {
                return (
                    <TableCellLayout>
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
            renderCell: (item) => {
                return item.packets;
            },
        }),
        createTableColumn<IUDPConversation>({
            columnId: "bytes",
            renderHeaderCell: () => <><BoxRegular /> Bytes</>,
            renderCell: (item) => {
                return format_bytes_single_unit(item.bytes);
            },
        }),
        createTableColumn<IUDPConversation>({
            columnId: "last",
            renderHeaderCell: () => <><BoxRegular /> Last Time</>,
            renderCell: (item) => {
                return Math.floor((item.last_time - item.first_time) / 1000) + 'ms';
            },
        }),
    ];
    // const onClick = (item: IUDPConversation) => {
    //     const title = `${item.sender} / ${item.receiver}`;
    //     navigate('/conversation/' + item.key, { state: { title } });
    // };
    const pageSize = conversation_size;
    const load = async (page: number) => {
        const _ip = ip === 'ANY' ? '' : ip;
        const data: ComRequest = {
            catelog: "udp",
            type: "list",
            param:{ ...compute(page, pageSize), ip: _ip },
        };
        return conversations(data);;
    }

    const breads = [
        { name: "UDP", icon: <ConversationIcon />, path: "/udp" }
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