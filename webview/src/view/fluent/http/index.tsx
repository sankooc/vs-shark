import React, { useEffect, useState } from "react";
import { useStore } from "../../store";
import { IVHttpConnection } from "../../../share/gen";
import { Button, createTableColumn, Drawer, DrawerBody, DrawerHeader, DrawerHeaderTitle, makeStyles, Tab, TableCellLayout, TableColumnDefinition, TabList, Tree, TreeItem, TreeItemLayout } from "@fluentui/react-components";
import { compute, ComRequest, format_bytes_single_unit, HttpMessageWrap, MessageCompress } from "../../../share/common";
import { Dismiss24Regular } from "@fluentui/react-icons";
import { Fade } from "@fluentui/react-motion-components-preview";
import indexCss from './index.module.scss';
import Grid from "../grid";
import { http_size } from "../../conf";
import HexView from "./hexview";
import PlainText from './plain';

const useStyles = makeStyles({
    customTree: {
        '--spacingHorizontalXXL': '12px',
        '--fontWeightRegular': 'bold',
    },
});

class ConnectProper {
    connection!: IVHttpConnection;
}

const tabList = (hmw: HttpMessageWrap | undefined): React.ReactNode[] => {
    const list:React.ReactNode[] = [];
    if(hmw){
        if (hmw.parsed_content){
            list.push(<Tab value="plaintext">plaintext</Tab>)
        }
    }
    return list;
}


const ConnectionList = (props: ConnectProper) => {
    const httpDetail = useStore((state) => state.httpDetail);
    const [_list, setList] = useState<HttpMessageWrap[]>([]);
    const [select, setSelect] = useState<string>('');
    const [tabSelect, setTabSelect] = useState<string>('binary');
    const [hmw, setHmw] = useState<HttpMessageWrap | undefined>();
    useEffect(() => {
        httpDetail(props.connection).then((rs: MessageCompress[]) => {
            const list: HttpMessageWrap[] = rs.map((r: MessageCompress) => {
                const rt = JSON.parse(r.json);
                if (r.data.length > 0) {
                    rt.raw = r.data;
                }
                return rt;
            });
            setList(list);
        });
    }, []);
    const styles = useStyles();
    const build = (hmw: HttpMessageWrap) => {
        const it = hmw.headers;
        const head = it[0];
        const items = [];
        for (let i = 1; i < it.length; i += 1) {
            let text = it[i];
            items.push(<TreeItem itemType="leaf" key={text}>
                <TreeItemLayout onClick={() => {
                    setSelect(text);
                    setHmw(undefined);
                    // setTabSelect('binary');
                }} className={select === text ? indexCss.treeitem_select : indexCss.treeitem} >{text}</TreeItemLayout>
            </TreeItem>);
        }
        if(hmw.raw && hmw.raw.length > 0){
            const len = hmw.raw.length;
            let key = `content-${hmw.headers[0]}`;
            items.push(<TreeItem itemType="leaf" key={key}>
                <TreeItemLayout onClick={() => {
                    setSelect(key);
                    setHmw(hmw);
                    // setTabSelect('binary');
                }} className={select === key ? indexCss.treeitem_select : indexCss.treeitem} >Entity({format_bytes_single_unit(len)})</TreeItemLayout>
            </TreeItem>);
        }
        
        return <TreeItem itemType="branch" key={head}>
            <TreeItemLayout onClick={() => {
                setSelect(head);
                setHmw(undefined);
                // setTabSelect('binary');
            }} className={select === head ? indexCss.treeitem_select : indexCss.treeitem} >{head}</TreeItemLayout>
            <Tree size="small">
                {items}
            </Tree>
        </TreeItem>
    }
    const hasContent = hmw?.raw && hmw.raw.length > 0;
    
    const tabContent = (hmw: HttpMessageWrap | undefined, tabSelect: string) => {
        if (hmw && tabSelect === 'binary') {
            return <HexView data={hmw.raw || new Uint8Array()} maxLength={1024 * 1024}/>
        }
        if (hmw && tabSelect === 'plaintext') {
            return <PlainText text={hmw!.parsed_content!} mime={hmw.mime} />
        }
        return <></>
    }
    return <div className="flex flex-row h-full" style={{ overflowX: "hidden", overflowY: "auto" }}>
        <div className="flex-1" style={{ overflow: "auto", padding: "5px 10px" }}>
            <Tree aria-label="Default" size="small" className={styles.customTree}>
                {_list.map(build)}
            </Tree>
        </div>
        <Fade visible={hasContent}>
         <div className="flex-1" style={{ padding: "5px 10px", borderLeft: "1px solid #ccc" }}>
            <TabList size="small" defaultSelectedValue={tabSelect} onTabSelect={(_, {value}: any) => {setTabSelect(value)}}>
                <Tab value="binary">Raw</Tab>
                {tabList(hmw)}
            </TabList>
            <div style={{ margin: "10px 0px", padding: "5px 10px", border: "1px solid #ccc" }}>
                {tabContent(hmw, tabSelect)}
            </div>
        </div>
        </Fade>
    </div>
}

function Component() {
    const httpConnections = useStore((state) => state.httpConnections);
    const [select, setSelect] = useState<IVHttpConnection | undefined>(undefined);
    const [open, setOpen] = useState<boolean>(false);
    const columns: TableColumnDefinition<IVHttpConnection>[] = [
        createTableColumn<IVHttpConnection>({
            columnId: "status",
            renderHeaderCell: () => {
                return "Status";
            },
            renderCell: (item) => {
                let status = 'N/A';
                if (item?.response) {
                    let ss = item.response.split(' ');
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
            renderHeaderCell: () => {
                return "Method";
            },
            renderCell: (item) => {
                let method = 'N/A';
                if (item?.request) {
                    let ss = item.request.split(' ');
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
            renderHeaderCell: () => {
                return "Host";
            },
            renderCell: (item) => {
                let host = 'N/A';
                if (item?.request) {
                    let ss = item.request.split(' ');
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
            renderHeaderCell: () => {
                return "Length";
            },

            renderCell: (item) => {
                return format_bytes_single_unit(item.length);
            },
        }),
        createTableColumn<IVHttpConnection>({
            columnId: "content_type",
            renderHeaderCell: () => {
                return "Content-Type";
            },

            renderCell: (item) => {
                return item.content_type;
            },
        }),
        createTableColumn<IVHttpConnection>({
            columnId: "time",
            renderHeaderCell: () => {
                return "Time";
            },

            renderCell: (item) => {
                return item.rt;
            },
        }),
    ];
    const onClick = (item: IVHttpConnection) => {
        setOpen(true);
        setSelect(item);
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
            defaultWidth: 50,
            minWidth: 50,
        },
        content_type: {
            // idealWidth: 200,
            // minWidth: 200,
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
    return <div className="flex flex-column h-full" style={{ overflowX: "hidden", overflowY: "auto" }}>
        <Grid columns={columns} onClick={onClick} pageSize={pageSize} load={load} columnSizingOptions={columnSizingOptions} />
        <Drawer
            type="overlay"
            separator
            open={open}
            position="bottom"
            size="full"
            modalType="non-modal"
        >
            <DrawerHeader>
                <DrawerHeaderTitle
                    action={
                        <Button
                            appearance="subtle"
                            aria-label="Close"
                            icon={<Dismiss24Regular />}
                            onClick={() => setOpen(false)}
                        />
                    }
                >
                </DrawerHeaderTitle>
            </DrawerHeader>

            <DrawerBody>
                {select !== undefined && <ConnectionList connection={select} />}
            </DrawerBody>
        </Drawer>
    </div>
}

export default Component;