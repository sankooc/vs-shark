import { useEffect, useState } from "react";
import { useStore } from "../../store";
import { makeStyles, SelectTabData, SelectTabEvent, Tab, TabList, TabValue, Tree, TreeItem, TreeItemLayout } from "@fluentui/react-components";
import { format_bytes_single_unit, HttpMessageWrap, MessageCompress } from "../../../share/common";
// import { Fade } from "@fluentui/react-motion-components-preview";
import indexCss from './index.module.scss';
import HexView from "./hexview";
import PlainText from './plain';
import { BreadItem, HttpIcon } from "../common";
import { DocumentGlobeRegular, PanelTopContractRegular, PanelTopExpandRegular } from "@fluentui/react-icons";

const useStyles = makeStyles({
    customTree: {
        '--spacingHorizontalXXL': '12px',
        '--fontWeightRegular': 'bold',
    },
});

// const tabList = (hmw: HttpMessageWrap | undefined): React.ReactNode[] => {
//     const list: React.ReactNode[] = [];
//     if (hmw) {
//         if (hmw.parsed_content) {
//             list.push(<Tab value="plaintext">plaintext</Tab>)
//         }
//     }
//     return list;
// }
export default function ConnectionList() {

    const httpDetail = useStore((state) => state.httpDetail);
    const getHttpCache = useStore((state) => state.getHttpCache);
    const [_list, setList] = useState<HttpMessageWrap[]>([]);
    const [select, setSelect] = useState<string>('');
    const [selectedValue, setSelectedValue] = useState<TabValue>("Header");

    const onTabSelect = (_: SelectTabEvent, data: SelectTabData) => {
        setSelectedValue(data.value);
    };

    // const [tabSelect, setTabSelect] = useState<string>('binary');
    // const [hmw, setHmw] = useState<HttpMessageWrap | undefined>();
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
    const styles = useStyles();
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
                    // setHmw(hmw);
                    // setTabSelect('binary');
                }} className={select === key ? indexCss.treeitem_select : indexCss.treeitem} >Entity({format_bytes_single_unit(len)})</TreeItemLayout>
            </TreeItem>);
        }

        return <TreeItem itemType="branch" key={head}>
            <TreeItemLayout onClick={() => {
                setSelect(head);
                // setHmw(undefined);
                // setTabSelect('binary');
            }} className={select === head ? indexCss.treeitem_select : indexCss.treeitem} >{head}</TreeItemLayout>
            <Tree size="small">
                {items}
            </Tree>
        </TreeItem>
    }
    // const hasContent = hmw?.raw && hmw.raw.length > 0;

    // const tabContent = (hmw: HttpMessageWrap | undefined, tabSelect: string) => {
    //     if (hmw && tabSelect === 'binary') {
    //         return <HexView data={hmw.raw || new Uint8Array()} maxLength={1024 * 1024} />
    //     }
    //     if (hmw && tabSelect === 'plaintext') {
    //         return <PlainText text={hmw!.parsed_content!} mime={hmw.mime} />
    //     }
    //     return <></>
    // }

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
    const buildContentPreview = (hmw: HttpMessageWrap) => {
        if(hmw.parsed_content){
            return <PlainText text={hmw!.parsed_content!} mime={hmw.mime} />
        }
        if(hmw.raw?.length){
            return <HexView data={hmw.raw || new Uint8Array()} maxLength={1024 * 1024} />
        }
        return <div className="flex justify-content-center align-content-center" >Empty</div>;

    }
    const contentRender = () => {
        switch (selectedValue) {
            case 'Header': {
                return <Tree aria-label="Default" size="small" className={styles.customTree}>
                    {_list.map(build)}
                </Tree>
            }
            case 'Payload': {
                if (_list && _list.length) {
                    const hmw = _list[0];
                    return buildContentPreview(hmw);
                }
                break;
            }
            case 'Response': {
                if (_list && _list.length > 1) {
                    const hmw = _list[1];
                    return buildContentPreview(hmw);
                }
                break;
            }
        }
        return <div className="flex justify-content-center align-content-center" >Empty</div>
    }
    return (<>
        <BreadItem items={breads} ></BreadItem>
        <TabList selectedValue={selectedValue} onTabSelect={onTabSelect}>
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
        {contentRender()}
    </>);

    // return <>
    //     <BreadItem items={breads} ></BreadItem>
    //     <div className="flex flex-row h-full" style={{ overflowX: "hidden", overflowY: "auto" }}>
    //         <div className="flex-1" style={{ overflow: "auto", padding: "5px 10px" }}>
    //             <Tree aria-label="Default" size="small" className={styles.customTree}>
    //                 {_list.map(build)}
    //             </Tree>
    //         </div>
    //         <Fade visible={hasContent}>
    //             <div className="flex-1 flex flex-column" style={{ padding: "5px 10px", borderLeft: "1px solid #ccc", overflowY: "hidden" }}>
    //                 <TabList size="small" defaultSelectedValue={tabSelect} onTabSelect={(_, { value }: any) => { setTabSelect(value) }}>
    //                     <Tab value="binary">Raw</Tab>
    //                     {tabList(hmw)}
    //                 </TabList>
    //                 <div style={{ margin: "10px 0px", padding: "5px 10px", border: "1px solid #ccc", overflowY: "auto" }}>
    //                     {tabContent(hmw, tabSelect)}
    //                 </div>
    //             </div>
    //         </Fade>
    //     </div>
    // </>
}
