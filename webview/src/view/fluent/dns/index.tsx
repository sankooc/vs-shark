
import { usePcapStore } from "../../context";
import { IDNSResponse } from "../../../share/gen";
import { createTableColumn, TableCellLayout, TableColumnDefinition, Toolbar, ToolbarButton } from "@fluentui/react-components";
import { compute, ComRequest } from "../../../share/common";
import Grid, { SortState } from "../table";

import { ActionInfoIcon, ActionMoreIcon, DNSIcon, TimeIcon } from "../common";
import { useNavigate } from "react-router";
import { useState } from "react";
function Component() {
    const dnsList = usePcapStore((state) => state.dnsList);
    const navigate = useNavigate();
    const [sortState, setSortState] = useState<SortState>({
        sortColumn: 'latency',
        sortDirection: 'ascending'
    })
    const columns: TableColumnDefinition<IDNSResponse>[] = [
        createTableColumn<IDNSResponse>({
            columnId: "time",
            renderHeaderCell: TimeIcon,
            renderCell: (item) => {
                let content = item.ts_str;
                if (item.offset_str) {
                    content = content + ` (+${item.offset_str[0]}${item.offset_str[1]})`
                }
                return <TableCellLayout>{content}</TableCellLayout>
            },
        }),
        createTableColumn<IDNSResponse>({
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
        createTableColumn<IDNSResponse>({
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
        createTableColumn<IDNSResponse>({
            columnId: "client",
            renderHeaderCell: () => 'Client',
            renderCell: (item) => <TableCellLayout>{item.target}</TableCellLayout>,
        }),
        createTableColumn<IDNSResponse>({
            columnId: "latency",
            renderHeaderCell: () => 'Latency',
            compare: (_a, _b) => {
                return 0;
            },
            renderCell: (item) => {
                let content = 'N/A';
                if (item.latency) {
                    content = `${item.latency[0]} ${item.latency[1]}`
                }
                return <TableCellLayout>{content}</TableCellLayout>
            },
        }),
        createTableColumn<IDNSResponse>({
            columnId: "ops",
            renderHeaderCell: () => "action",
            renderCell: (_item) => {
                return <Toolbar aria-label="Default" size="small">
                    <ToolbarButton icon={ActionInfoIcon()} onClick={() => { onClick(_item) }} />
                    <ToolbarButton icon={ActionMoreIcon()} />
                </Toolbar>
            },
        }),
    ];
    const onClick = (item: IDNSResponse) => {
        const index = item.response;
        if (undefined != index) {
            navigate('/dns/' + index);
        }
    }
    const pageSize = 30;
    const load = async (page: number) => {
        const asc = sortState.sortDirection === 'ascending';
        const data: ComRequest = {
            catelog: "dns",
            type: "list",
            param: { ...compute(page, pageSize), asc },
        };
        return dnsList(data);
    }

    const breads = [
        { name: "DNS", icon: <DNSIcon />, path: "/dns" }
    ]
    const columnSizingOptions = {

        time: {
            minWidth: 300,
            idealWidth: 300,
        },
        transaction: {
            minWidth: 120,
            idealWidth: 120,
        },
        receiver: {
            minWidth: 250,
            idealWidth: 300,
        },
        client: {
            minWidth: 250,
            idealWidth: 300,
        },
        latency: {
            minWidth: 120,
            idealWidth: 120,
        }
        // opt: {
        //     minWidth: 120,
        //     idealWidth: 120,
        // }

    };
    const gridProps = {
        sortState,
        onSortChange: (_e: Event, state: SortState) => {
            setSortState(state);
        },
        columns, pageSize, load, columnSizingOptions, breads
    };
    return <Grid {...gridProps} />;
}

export default Component;