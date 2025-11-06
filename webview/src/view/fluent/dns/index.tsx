import { useStore } from "../../store";
import { IDNSResponse } from "../../../share/gen";
import { createTableColumn, TableCellLayout, TableColumnDefinition, Toolbar, ToolbarButton } from "@fluentui/react-components";
import { compute, ComRequest } from "../../../share/common";
import Grid from "../table";

import { ActionInfoIcon, ActionMoreIcon, DNSIcon } from "../common";
import { useNavigate } from "react-router";
function Component() {
    const dnsList = useStore((state) => state.dnsList);
    const navigate = useNavigate();
    const columns: TableColumnDefinition<IDNSResponse>[] = [
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
            renderCell: (item) => {
                let content = 'N/A';
                if (item.latency){
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
              <ToolbarButton icon={ActionMoreIcon()}/>
            </Toolbar>
          },
        }),
    ];
    const onClick = (item: IDNSResponse) => {
        const index = item.index;
        if (undefined != index) {
          navigate('/dns/' + index);
        }
    }
    const pageSize = 30;
    const load = async (page: number) => {
        const data: ComRequest = {
            catelog: "dns",
            type: "list",
            param: { ...compute(page, pageSize) },
        };
        return dnsList(data);
    }

    const breads = [
        { name: "DNS", icon: <DNSIcon />, path: "/dns" }
    ]
    const columnSizingOptions = {
        transaction: {
            minWidth: 120,
            idealWidth: 120,
            autoFitColumns: true,
        },
        receiver: {
            minWidth: 250,
            idealWidth: 300,
            autoFitColumns: true,
        },
        client: {
            minWidth: 250,
            idealWidth: 300,
        },
        latency: {
            minWidth: 120,
            idealWidth: 120,
        }

    };
    const gridProps = {
        columns, pageSize, load, columnSizingOptions, breads
    };
    return <Grid {...gridProps} />;
}

export default Component;