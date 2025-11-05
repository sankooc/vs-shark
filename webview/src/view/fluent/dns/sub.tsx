import { useStore } from "../../store";
import { IDNSRecord } from "../../../share/gen";
import { createTableColumn, TableCellLayout, TableColumnDefinition } from "@fluentui/react-components";
import { compute, ComRequest } from "../../../share/common";
import Grid from "../table";

import { DNSIcon } from "../common";
import { useParams } from "react-router";
function Component() {
    const request = useStore((state) => state.dnsRecords);
    const { index } = useParams();
    const columns: TableColumnDefinition<IDNSRecord>[] = [
        createTableColumn<IDNSRecord>({
            columnId: "domain",
            renderHeaderCell: () => 'Domain',
            renderCell: (item) => <TableCellLayout> {item.host} </TableCellLayout>,
        }),
        createTableColumn<IDNSRecord>({
            columnId: "rtype",
            renderHeaderCell: () => 'Type',
            renderCell: (item) => <TableCellLayout> {item.rtype} </TableCellLayout>,
        }),
        createTableColumn<IDNSRecord>({
            columnId: "class",
            renderHeaderCell: () => 'Class',
            renderCell: (item) => <TableCellLayout> {item.class} </TableCellLayout>,
        }),
        createTableColumn<IDNSRecord>({
            columnId: "info",
            renderHeaderCell: () => 'Record',
            renderCell: (item) => <TableCellLayout> {item.info || 'None'} </TableCellLayout>,
        }),
    ];
    const pageSize = 30;
    const load = async (page: number) => {
        const data: ComRequest = {
            catelog: "dns_record",
            type: "list",
            param: { ...compute(page, pageSize), index },
        };
        return request(data);
    }

    const breads = [
        { name: "DNS", icon: <DNSIcon />, path: "/dns" },
        { name: `${index}`, path: "/dns/" + index }
    ]
    const columnSizingOptions = {
        domain: {
            minWidth: 250,
            idealWidth: 400,
            autoFitColumns: true,
        },
        rtype: {
            minWidth: 100,
            idealWidth: 100,
            autoFitColumns: true,
        },
        class: {
            minWidth: 100,
            idealWidth: 100,
            autoFitColumns: true,
        },

    };
    const gridProps = {
        columns, pageSize, load, columnSizingOptions, breads
    };
    return <Grid {...gridProps} />;
}

export default Component;