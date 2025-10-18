import { useEffect, useState } from "react";
import { useStore } from "../../store";
import { SelectTabData, SelectTabEvent, Tab, TabList, TabValue, Tree, TreeItem, TreeItemLayout } from "@fluentui/react-components";
import { format_bytes_single_unit, HttpMessageWrap, MessageCompress } from "../../../share/common";
import indexCss from './index.module.scss';
import { HttpIcon } from "../common";
import { DocumentGlobeRegular, PanelTopContractRegular, PanelTopExpandRegular } from "@fluentui/react-icons";
import ContentComponent from './content';
import Empty from "./content/empty";

import {PageFrame} from '../table';


// const useStyles = makeStyles({
//     customTree: {
//         '--spacingHorizontalXXL': '12px',
//         '--fontWeightRegular': 'bold',
//         'padding': '5px',
//     },
//     tab: {
//         button: {
//             color: 'red'
//         }
//     }
// });

export default function ConnectionList() {

    const httpDetail = useStore((state) => state.httpDetail);
    const getHttpCache = useStore((state) => state.getHttpCache);
    const [_list, setList] = useState<HttpMessageWrap[]>([]);
    const [select, setSelect] = useState<string>('');
    const [selectedValue, setSelectedValue] = useState<TabValue>("Header");

    const onTabSelect = (_: SelectTabEvent, data: SelectTabData) => {
        setSelectedValue(data.value);
    };

    useEffect(() => {
        const connection = getHttpCache();
        if (!connection) {
            return;
        }
        httpDetail(connection).then((rs: MessageCompress[]) => {
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
    // const styles = useStyles();
    const build = (hmw: HttpMessageWrap) => {
        const it = hmw.headers;
        const head = it[0];
        const items = [];
        for (let i = 1; i < it.length; i += 1) {
            const text = it[i];
            items.push(<TreeItem itemType="leaf" key={text}>
                <TreeItemLayout onClick={() => {
                    setSelect(text);
                    // setHmw(undefined);
                }} className={select === text ? indexCss.treeitem_select : indexCss.treeitem} >{text}</TreeItemLayout>
            </TreeItem>);
        }
        if (hmw.raw && hmw.raw.length > 0) {
            const len = hmw.raw.length;
            const key = `content-${hmw.headers[0]}`;
            items.push(<TreeItem itemType="leaf" key={key}>
                <TreeItemLayout onClick={() => {
                    setSelect(key);
                }} className={select === key ? indexCss.treeitem_select : indexCss.treeitem} >Entity({format_bytes_single_unit(len)})</TreeItemLayout>
            </TreeItem>);
        }

        return <TreeItem itemType="branch" key={head}>
            <TreeItemLayout onClick={() => {
                setSelect(head);
            }} className={select === head ? indexCss.treeitem_select : indexCss.treeitem} >{head}</TreeItemLayout>
            <Tree size="small">
                {items}
            </Tree>
        </TreeItem>
    }
    let title = "Request";
    if (_list && _list.length) {
        const hmw = _list[0];
        for (const h of hmw.headers) {
            if (h.toLowerCase().startsWith("host:")) {
                title = h.substring(5).trim();
                break;
            }
        }
    }
    const breads = [
        { name: "HTTP Requests", icon: <HttpIcon />, path: "/https" },
        { name: title, path: "/http/detail" }
    ]

    const contentRender = () => {
        switch (selectedValue) {
            case 'Header': {
                return <Tree aria-label="Default" size="small">
                    {_list.map(build)}
                </Tree>
            }
            case 'Payload': {
                if (_list && _list.length) {
                    const hmw = _list[0];
                    return <ContentComponent hmw={hmw} />
                }
                break;
            }
            case 'Response': {
                if (_list && _list.length > 1) {
                    const hmw = _list[1];
                    return <ContentComponent hmw={hmw} />
                }
                break;
            }
        }
        return <Empty/>
    }
    return (<PageFrame breads={breads}>
        <>
            <TabList selectedValue={selectedValue} onTabSelect={onTabSelect} >
                <Tab id="Header" icon={<DocumentGlobeRegular />} value="Header">
                    Header
                </Tab>
                <Tab id="Payload" icon={<PanelTopContractRegular />} value="Payload">
                    Payload
                </Tab>
                <Tab id="Response" icon={<PanelTopExpandRegular />} value="Response">
                    Response
                </Tab>
            </TabList>
            <div className="flex-1" style={{ border: '1px solid #ddd', overflow: 'auto' }} >
                {contentRender()}
            </div>
        </>
    </PageFrame>);
}
