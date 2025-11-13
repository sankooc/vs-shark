// import React, { useEffect, useState } from "react";
import { useStore } from "../../store";
import { IVHttpConnection, ICounterItem } from "../../../share/gen";
import { createTableColumn, Select, TableCellLayout, TableColumnDefinition, Toolbar, ToolbarButton } from "@fluentui/react-components";
import { compute, ComRequest, format_bytes_single_unit } from "../../../share/common";
import indexCss from './index.module.scss';
import { useNavigate } from "react-router";
import Grid from "../table";
import { http_size } from "../../conf";
import { ActionInfoIcon, ActionMoreIcon, HttpIcon } from "../common";
import { BorderAllRegular, ClipboardCodeRegular, DesktopSignalRegular, ImageRegular, TextWordCountRegular, CodeBlockRegular, ContentViewRegular, WarningRegular, CheckmarkSquareRegular, ClockRegular } from "@fluentui/react-icons";


import { useId, Label } from "@fluentui/react-components";

import { useEffect, useState } from "react";
import Empty from "./content/empty";

const NoneOption = "ANY";

const SIZE: "small" | "medium" = 'small';
const docIcon = (item: IVHttpConnection) => {
    const _type = (item.content_type || '').toLocaleLowerCase();
    if (_type.indexOf('css') >= 0 || _type.indexOf('javascript') >= 0 || _type.indexOf('xml') >= 0) {
        return <CodeBlockRegular />
    }
    if (_type.indexOf('png') >= 0 || _type.indexOf('jpeg') >= 0 || _type.indexOf('gif') >= 0) {
        return <ImageRegular />
    }
    return <ContentViewRegular />
}

// const http_rt_icon = (item: IVHttpConnection) => {
//     const timeStr = item.rt;
//     if (timeStr) {
//         try {
//             if ('N/A' === timeStr) {
//                 return <ClockDismissRegular />
//             }
//             if (timeStr.indexOf('µs') >= 0) {
//                 const time: number = parseInt(timeStr.substring(0, timeStr.length - 2));
//                 if (time > 10000) {
//                     return <ClockBillRegular />
//                 }
//             }

//         } catch (e) {
//             console.error(e);
//         }
//     }
//     return <ClockRegular />;
// }

const http_connct_status = (status: string) => {
    try {
        if (parseInt(status) > 0) {
            return <CheckmarkSquareRegular />
        }
    } catch (e) {
        console.error(e);
    }
    return <WarningRegular />
}

function Component() {
    const httpConnections = useStore((state) => state.httpList);
    const stat = useStore((state) => state.stat);
    const [httpHosts, setHttpHosts] = useState<ICounterItem[]>([]);
    const [hostSelect, setHostSelect] = useState<string>(NoneOption);
    useEffect(() => {
        stat({ field: 'http_host' }).then(setHttpHosts);
    }, [])
    const navigate = useNavigate();
    const selectId = useId();

    if (httpHosts.length === 0) {
        return <Empty />
    }

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
                const media = http_connct_status(status);
                let color = '#fabd2f';
                const code = parseInt(status, 10);
                if(code < 400){
                    color = '#b8bb26';
                } else if (code >= 400){
                    color = '#fb4934'; 
                }
                return (
                    <TableCellLayout media={media} style={{ textAlign: 'center', color }}>
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
            renderHeaderCell: () => <><DesktopSignalRegular /> host</>,
            renderCell: (item) => (<TableCellLayout className={indexCss.cell}>
                        {item.hostname}
                    </TableCellLayout>),
        }),
        createTableColumn<IVHttpConnection>({
            columnId: "path",
            renderHeaderCell: () => <><DesktopSignalRegular /> path</>,
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
                let contentType = item.content_type;
                if (contentType) {
                    const inx = contentType.indexOf(';');
                    if(inx >= 0 ){
                        contentType = contentType.substring(0, inx).trim();
                    }
                    return <TableCellLayout media={docIcon(item)} style={{ textAlign: 'center' }}>
                        {contentType}
                    </TableCellLayout>
                }
                return '';
            },
        }),
        createTableColumn<IVHttpConnection>({
            columnId: "time",
            renderHeaderCell: () => <><ClockRegular /> Latency</>,
            renderCell: (item) => {
                const timeStr = item.latency;
                return <TableCellLayout >
                    {timeStr}
                </TableCellLayout>
            },
        }),
        createTableColumn<IVHttpConnection>({
            columnId: "ops",
            renderHeaderCell: () => "action",
            renderCell: (item) => {
                return <Toolbar aria-label="Default" size="small">
                    <ToolbarButton icon={ActionInfoIcon()} onClick={() => { onClick(item) }} />
                    <ToolbarButton icon={ActionMoreIcon()} />
                </Toolbar>
            },
        }),
    ];
    const onClick = (item: IVHttpConnection) => {
        // cachehttp(item);
        const index = item.index;
        const title = item.hostname || 'detail';
        if (index >=0 ){
            navigate('/http/detail/' + index, { state: { title } });
        }
    };
    const pageSize = http_size;
    const load = async (page: number, _: any) => {
        const host = hostSelect === NoneOption ? '' : hostSelect;
        const data: ComRequest = {
            catelog: "http_connection",
            type: "list",
            param: { ...compute(page, pageSize), host, asc: true },
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
            idealWidth: 200,
            minWidth: 200,
        },
        length: {
            autoFitColumns: true,
            idealWidth: 100,
            defaultWidth: 100,
            minWidth: 100,
        },
        content_type: {
            defaultWidth: 250,
            minWidth: 250,
            autoFitColumns: true,
        },
        time: {
            defaultWidth: 120,
            minWidth: 120,
            autoFitColumns: true,
        },
        ops: {
            defaultWidth: 100,
            minWidth: 100,
            autoFitColumns: true,

        }
    }

    const breads = [
        { name: "HTTP Requests", icon: <HttpIcon />, path: "/https" }
    ]

    const onChange = (_: any, val: any) => {
        setHostSelect(val.value);
    }
    const gridProps = {
        // header: <HTTPChart />,
        filterComponent: (<>
            <Label size={SIZE} htmlFor={selectId} style={{ paddingInlineEnd: "5px" }}>Hostname:</Label>
            <Select size={SIZE} id={selectId} onChange={onChange} value={hostSelect} >
                <option>{NoneOption}</option>
                {httpHosts.map((h) => {
                    return <option key={h.key}>{h.key}</option>
                })}
            </Select>
        </>),
        size: SIZE,
        columns, pageSize, load, columnSizingOptions, breads
    };

    return <Grid {...gridProps} />;
}

export default Component;