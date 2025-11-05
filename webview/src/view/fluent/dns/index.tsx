import { useStore } from "../../store";
import { IDNSRecord } from "../../../share/gen";
import { createTableColumn, TableCellLayout, TableColumnDefinition } from "@fluentui/react-components";
import { compute, ComRequest } from "../../../share/common";
import Grid from "../table";

import { DNSIcon } from "../common";
function Component() {
    const dnsList = useStore((state) => state.dnsList);
    const columns: TableColumnDefinition<IDNSRecord>[] = [
        createTableColumn<IDNSRecord>({
            columnId: "transaction",
            renderHeaderCell: () => 'TID',
            renderCell: (item) => {
                return (
                    <TableCellLayout>
                        {item.transaction_id}
                    </TableCellLayout>
                );
            },
        }),
        createTableColumn<IDNSRecord>({
            columnId: "receiver",
            renderHeaderCell: () => 'Server',
            renderCell: (item) => {
                return (
                    <TableCellLayout>
                        {item.source}
                    </TableCellLayout>
                );
            },
        }),
        createTableColumn<IDNSRecord>({
            columnId: "count",
            renderHeaderCell: () => 'Client',
            renderCell: (item) => <TableCellLayout>{item.target}</TableCellLayout>,
        }),
        createTableColumn<IDNSRecord>({
            columnId: "bytes",
            renderHeaderCell: () => 'Latency',
            renderCell: (item) => {
                let content = 'N/A';
                if (item.latency){
                    content = `${item.latency[0]} ${item.latency[1]}`
                }
                return <TableCellLayout>{content}</TableCellLayout>
            },
        }),
    ];
    const pageSize = 30;
    const load = async (page: number) => {
        const data: ComRequest = {
            catelog: "dns",
            type: "list",
            param: { ...compute(page, pageSize) },
        };
        return dnsList(data);;
    }

    const breads = [
        { name: "DNS", icon: <DNSIcon />, path: "/dns" }
    ]
    const columnSizingOptions = {
        sender: {
            minWidth: 250,
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
        columns, pageSize, load, columnSizingOptions, breads
    };
    return <Grid {...gridProps} />;
}

export default Component;