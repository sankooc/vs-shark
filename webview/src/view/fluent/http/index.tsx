// import React, { useEffect, useState } from "react";
import { useStore } from "../../store";
import { IVHttpConnection, IHttpStatistics } from "../../../share/gen";
import { createTableColumn, Select, TableCellLayout, TableColumnDefinition } from "@fluentui/react-components";
import { compute, ComRequest, format_bytes_single_unit } from "../../../share/common";
import indexCss from './index.module.scss';
import { useNavigate } from "react-router";
import Grid from "../table";
import { http_size } from "../../conf";
import { HttpIcon } from "../common";
import { Image28Color, CodeBlock28Color,ContentView28Color, BorderAllRegular, ClipboardCodeRegular, DesktopSignalRegular, ImageRegular, TextWordCountRegular, TimePickerRegular } from "@fluentui/react-icons";

import { useId, Label } from "@fluentui/react-components";

import { useEffect, useState } from "react";

const NoneOption = "ANY";

const docIcon = (item: IVHttpConnection) => {
    // return <Image28Color />
    // return <CodeBlock28Color />
    const _type = (item.content_type || '').toLocaleLowerCase();
    if(_type.indexOf('css') >= 0 || _type.indexOf('javascript') >= 0 || _type.indexOf('xml') >= 0) {
        return <CodeBlock28Color />
    }
    if(_type.indexOf('png') >= 0 || _type.indexOf('jpeg') >= 0 || _type.indexOf('gif') >= 0) {
        return <Image28Color />
    }
    return <ContentView28Color />
}

function Component() {
    const httpConnections = useStore((state) => state.httpConnections);
    const cachehttp = useStore((state) => state.cachehttp);
    const httpStat = useStore((state) => state.httpStat);
    const [httpHosts, setHttpHosts] = useState<IHttpStatistics[]>([]);
    const [hostSelect, setHostSelect] = useState<string>(NoneOption);
    useEffect(() => {
        httpStat().then(setHttpHosts);
    }, [])
    const navigate = useNavigate();
    const columns: TableColumnDefinition<IVHttpConnection>[] = [
        createTableColumn<IVHttpConnection>({
            columnId: "icon",
            renderHeaderCell: () => <></>,
            renderCell: (item) => {
                return (
                    docIcon(item)
                );
            },
        }),
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
                    <TableCellLayout style={{textAlign: 'center'}}>
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
    const load = async (page: number, _: any) => {
        let host = hostSelect === NoneOption ? '' : hostSelect;
        const data: ComRequest = {
            catelog: "http_connection",
            type: "list",
            param: { ...compute(page, pageSize), host },
        };
        return httpConnections(data);
    }

    const columnSizingOptions = {
        icon: {
            idealWidth: 50,
            minWidth: 50,
            defaultWidth: 50,
        },
        status: {
            idealWidth: 80,
            minWidth: 80,
            defaultWidth: 80,
        },
        method: {
            idealWidth: 80,
            minWidth: 80,
            defaultWidth: 80,
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
        { name: "HTTP Requests", icon: <HttpIcon />, path: "/https" }
    ]

    const selectId = useId();
    const onChange = (_: any, val: any) => {
        setHostSelect(val.value);
    }
    const gridProps = {
        filterComponent: (<>
            <Label size="small" htmlFor={selectId} style={{ paddingInlineEnd: "5px" }}>Hostname:</Label>
            <Select size="small" id={selectId} onChange={onChange} value={hostSelect} >
                <option>{NoneOption}</option>
                {httpHosts.map((h) => {
                    return <option key={h.host}>{h.host}</option>
                })}
            </Select>
        </>),
        columns, onClick, pageSize, load, columnSizingOptions, breads
    };

    return <Grid {...gridProps} />;
}

export default Component;