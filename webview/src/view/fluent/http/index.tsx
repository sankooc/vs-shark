// import React, { useEffect, useState } from "react";
import { useStore } from "../../store";
import { IVHttpConnection, ICounterItem } from "../../../share/gen";
import { createTableColumn, Select, TableCellLayout, TableColumnDefinition, Toolbar, ToolbarButton } from "@fluentui/react-components";
import { compute, ComRequest, format_bytes_single_unit } from "../../../share/common";
import indexCss from './index.module.scss';
import { useNavigate } from "react-router";
import Grid from "../table";
import { http_size } from "../../conf";
import { HttpIcon } from "../common";
import { BorderAllRegular, ClockBillRegular, ClockDismissRegular, ClipboardCodeRegular, DesktopSignalRegular, ImageRegular, TextWordCountRegular, CodeBlockRegular, ContentViewRegular, TextBulletListSquareColor, MoreHorizontalFilled, ClockRegular, WarningRegular, CheckmarkSquareRegular } from "@fluentui/react-icons";

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

const http_rt_icon = (item: IVHttpConnection) => {
    const timeStr = item.rt;
    if (timeStr) {
        try {
            if ('N/A' === timeStr) {
                return <ClockDismissRegular />
            }
            if (timeStr.indexOf('µs') >= 0) {
                const time: number = parseInt(timeStr.substring(0, timeStr.length - 2));
                if (time > 10000) {
                    return <ClockBillRegular />
                }
            }

        } catch (e) {

        }
    }
    return <ClockRegular />;
}

const http_connct_status = (status: string) => {
    try {
        if(parseInt(status) > 0){
            return <CheckmarkSquareRegular />
        }
    }catch(e) {}
    return <WarningRegular />
}

function Component() {
    const httpConnections = useStore((state) => state.httpConnections);
    const cachehttp = useStore((state) => state.cachehttp);
    const stat = useStore((state) => state.stat);
    const [httpHosts, setHttpHosts] = useState<ICounterItem[]>([]);
    const [hostSelect, setHostSelect] = useState<string>(NoneOption);
    useEffect(() => {
        stat({field: 'http_host'}).then(setHttpHosts);
    }, [])
    const navigate = useNavigate();
    const selectId = useId();

    if(httpHosts.length === 0){
        return <Empty/>
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
                return (
                    <TableCellLayout media={media} style={{ textAlign: 'center' }}>
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
                const contentType = item.content_type;
                if(contentType){
                    return  <TableCellLayout media={docIcon(item)} style={{ textAlign: 'center' }}>
                        {contentType}
                    </TableCellLayout>
                }
                return '';
            },
        }),
        createTableColumn<IVHttpConnection>({
            columnId: "time",
            renderHeaderCell: () => <><ClockRegular /> Time</>,
            renderCell: (item) => {
                const timeStr = item.rt;
                return <TableCellLayout media={http_rt_icon(item)}>
                    {timeStr}
                </TableCellLayout>
            },
        }),
        createTableColumn<IVHttpConnection>({
            columnId: "ops",
            renderHeaderCell: () => "ext",
            renderCell: (item) => {
                return <Toolbar aria-label="Default" size="small">
                    <ToolbarButton icon={<TextBulletListSquareColor />} onClick={() => { onClick(item) }} />
                    <ToolbarButton icon={<MoreHorizontalFilled />} />
                </Toolbar>
                // return <TableCellLayout media={<TextBulletListSquareColor />} style={{cursor: 'pointer'}}></TableCellLayout>
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