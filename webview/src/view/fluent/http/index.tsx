// import React, { useEffect, useState } from "react";
import { useStore } from "../../store";
import { IVHttpConnection } from "../../../share/gen";
import { createTableColumn, TableCellLayout, TableColumnDefinition } from "@fluentui/react-components";
import { compute, ComRequest, format_bytes_single_unit } from "../../../share/common";
import indexCss from './index.module.scss';
import { useNavigate } from "react-router";
import Grid from "../grid";
import { http_size } from "../../conf";
import { HttpIcon } from "../common";
import { BorderAllRegular, ClipboardCodeRegular, DesktopSignalRegular, ImageRegular, TextWordCountRegular, TimePickerRegular } from "@fluentui/react-icons";

function Component() {
    const httpConnections = useStore((state) => state.httpConnections);
    const cachehttp = useStore((state) => state.cachehttp);
    const navigate = useNavigate();
    const columns: TableColumnDefinition<IVHttpConnection>[] = [
        createTableColumn<IVHttpConnection>({
            columnId: "status",
            renderHeaderCell: () => <><ClipboardCodeRegular /> Status</>,
            renderCell: (item) => {
                let status = 'N/A';
                if (item?.response) {
                    const ss = item.response.split(' ');
                    if (ss.length > 1) {
                        status = ss[1];
                    }
                }
                return (
                    <TableCellLayout>
                        {status}
                    </TableCellLayout>
                );
            },
        }),
        createTableColumn<IVHttpConnection>({
            columnId: "method",
            renderHeaderCell: () => <><BorderAllRegular />Method</>,
            renderCell: (item) => {
                let method = 'N/A';
                if (item?.request) {
                    const ss = item.request.split(' ');
                    if (ss.length > 1) {
                        method = ss[0];
                    }
                }
                return (
                    <TableCellLayout>
                        {method}
                    </TableCellLayout>
                );
            },
        }),
        createTableColumn<IVHttpConnection>({
            columnId: "host",
            renderHeaderCell: () => <><DesktopSignalRegular /> Host</>,
            renderCell: (item) => {
                let host = 'N/A';
                if (item?.request) {
                    const ss = item.request.split(' ');
                    if (ss.length > 1) {
                        host = ss[1];
                    }
                }
                return (
                    <TableCellLayout className={indexCss.cell}>
                        {host}
                    </TableCellLayout>
                );
            },
        }),
        createTableColumn<IVHttpConnection>({
            columnId: "length",
            renderHeaderCell: () => <><TextWordCountRegular /> Length</>,
            renderCell: (item) => {
                return format_bytes_single_unit(item.length);
            },
        }),
        createTableColumn<IVHttpConnection>({
            columnId: "content_type",
            renderHeaderCell: () => <><ImageRegular /> Content-Type</>,
            renderCell: (item) => {
                return item.content_type;
            },
        }),
        createTableColumn<IVHttpConnection>({
            columnId: "time",
            renderHeaderCell: () => <><TimePickerRegular /> Time</>,
            renderCell: (item) => {
                return item.rt;
            },
        }),
    ];
    const onClick = (item: IVHttpConnection) => {
        cachehttp(item);
        navigate('/http/detail', { state: { title: '' } });
    };
    const pageSize = http_size;
    const load = async (page: number) => {
        const data: ComRequest = {
            catelog: "http_connection",
            type: "list",
            param: { ...compute(page, pageSize) },
        };
        return httpConnections(data);
    }

    const columnSizingOptions = {
        status: {
            idealWidth: 50,
            minWidth: 50,
            defaultWidth: 50,
        },
        method: {
            idealWidth: 50,
            minWidth: 50,
            defaultWidth: 50,
        },
        host: {
            autoFitColumns: true,
            idealWidth: 1000,
            minWidth: 200,
            // defaultWidth: 200,
        },
        length: {
            autoFitColumns: true,
            idealWidth: 100,
            defaultWidth: 100,
            minWidth: 100,
        },
        content_type: {
            defaultWidth: 200,
            minWidth: 200,
            autoFitColumns: true,
        },
        time: {
            defaultWidth: 120,
            minWidth: 120,
            autoFitColumns: true,
        },
    }
    
    const breads = [
        { name: "HTTP Requests", icon: <HttpIcon/>, path: "/https" }
    ]
    return <div className={"flex flex-column h-full " + indexCss.fixframe}>
        <Grid columns={columns} onClick={onClick} pageSize={pageSize} load={load} columnSizingOptions={columnSizingOptions} breads={breads} />
    </div>
}

export default Component;