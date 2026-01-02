import type { JSXElement } from "@fluentui/react-components";
import {
    MenuList,
    MenuItem,
    MenuPopover,
    MenuTrigger,
    Menu,
    Button,
    makeStyles,
    tokens,
    MenuItemRadio,
} from "@fluentui/react-components";
import {
    bundleIcon,
    FolderOpenRegular,
    DocumentRegular,
    DocumentFilled,
    FolderOpenFilled,
    ArrowExitRegular,
    ArrowExitFilled,
    DataPieFilled,
    DataPieRegular,
    ContentViewFilled,
    ContentViewRegular,
    AppsListDetailFilled,
    AppsListDetailRegular,
    BookInformationRegular,
} from "@fluentui/react-icons";
import { usePcapStore } from "../../context";
import { PcapState } from "../../../share/common";
import { ConversationIcon, DNSIcon, FrameIcon, HttpIcon, OverviewIcon, TLSIcon, UDPTabIcon } from "../common";
import { useNavigate, useLocation } from "react-router";
import { useState } from "react";
import Property from './property';
import { StatusBar } from "../common";
import './index.scss'

const DocIcon = bundleIcon(DocumentFilled, DocumentRegular)
const OpenIcon = bundleIcon(FolderOpenFilled, FolderOpenRegular);
const CloseIcon = bundleIcon(ArrowExitFilled, ArrowExitRegular);

const ViewIcon = bundleIcon(ContentViewFilled, ContentViewRegular);

const StaIcon = bundleIcon(DataPieFilled, DataPieRegular);
const PropertyIcon = bundleIcon(AppsListDetailFilled, AppsListDetailRegular)


const useStyles = makeStyles({
    menuBar: {
        backgroundColor: tokens.colorNeutralCardBackground,
        padding: '3px 2px',
    },
});
const getRoute = (loc: string): string => {
    try {
        const route = loc;
        const tks = route.split('/');
        if(tks && tks.length > 1) {
            if(tks[1] === 'tls'){
                return '/tlslist';
            }
            return tks.slice(0, 2).join('/')
        }
        return route;
    }catch{/**/}
    return '';
};
export default function MultilineItems(): JSXElement {
    const navigate = useNavigate();
    const cus = useStyles();
    const [open, setOpen] = useState<boolean>(false)
    const info = usePcapStore((state: PcapState) => state.fileinfo);
    let canSelectFile = false;
    let debug = false;
    let version = '0.0.0';
    if (import.meta.env) {
        canSelectFile = import.meta.env.VITE_BUILD_ALL === 'true' || import.meta.env.VITE_BUILD_GUI === 'true';
        debug = !!import.meta.env.DEV;
        version = import.meta.env.VITE_APP_VERSION || '0.0.0';
    }
    const toRoute = (path: string) => {
        return () => {
            navigate(path);
        }
    }
    const closeFile = usePcapStore((state: PcapState) => state.closeFile);
    const openFile = usePcapStore((state: PcapState) => state.openFile);
    const route = getRoute(useLocation().pathname);
    if (!info) {
        if (!canSelectFile) {
            return <></>
        }
        return (
            <div className={"flex flex-row items-center " + cus.menuBar}>
                <Menu openOnHover hoverDelay={0}>
                    <MenuTrigger>
                        <Button shape="square" size="small" appearance="transparent" icon={<DocIcon />}>File</Button>
                    </MenuTrigger>
                    <MenuPopover>
                        <MenuList>
                            <MenuItem subText="Select pcap file" icon={<OpenIcon />} onClick={openFile}>
                                Open
                            </MenuItem>
                        </MenuList>
                    </MenuPopover>
                </Menu>
            </div>
        );
    }

    return (<>
        <div className={"flex flex-row items-center " + cus.menuBar}>
            {canSelectFile ? <Menu openOnHover hoverDelay={0}>
                <MenuTrigger>
                    <Button shape="square" size="small" appearance="transparent" icon={<DocIcon />}>File</Button>
                </MenuTrigger>
                <MenuPopover>
                    <MenuList>
                        <MenuItem subText="Select pcap file" icon={<OpenIcon />} onClick={openFile} disabled >
                            Open
                        </MenuItem>
                        <MenuItem subText="Close File" icon={<CloseIcon />} onClick={() => {
                            closeFile();
                            navigate('/');
                        }}>
                            Close
                        </MenuItem>
                    </MenuList>
                </MenuPopover>
            </Menu> : null
            }
            <Menu openOnHover hoverDelay={0}>
                <MenuTrigger>
                    <Button shape="square" size="small" appearance="transparent" icon={<ViewIcon />}>View</Button>
                </MenuTrigger>
                <MenuPopover>
                    <MenuList checkedValues={{ view: [route] }}>
                        <MenuItemRadio icon={<OverviewIcon />} name="view" value="/overview" onClick={toRoute('/overview')}>
                            Overview
                        </MenuItemRadio>

                        <MenuItemRadio icon={<FrameIcon />} name="view" value="/" onClick={toRoute('/')}>
                            Frames
                        </MenuItemRadio>
                    </MenuList>
                </MenuPopover>
            </Menu>
            <Menu openOnHover hoverDelay={0} >
                <MenuTrigger>
                    <Button shape="square" size="small" appearance="transparent" icon={<StaIcon />}>Statistic</Button>
                </MenuTrigger>
                <MenuPopover>
                    <MenuList checkedValues={{ view: [route] }}>
                        <MenuItemRadio icon={<ConversationIcon />} name="view" value="/conversation" onClick={toRoute('/conversation')}>
                            TCP
                        </MenuItemRadio>
                        <MenuItemRadio icon={<UDPTabIcon />} name="view" value="/udp" onClick={toRoute('/udp')}>
                            UDP
                        </MenuItemRadio>
                        <MenuItemRadio icon={<HttpIcon />} name="view" value="/https" onClick={toRoute('/https')}>
                            HTTP
                        </MenuItemRadio>
                        <MenuItemRadio icon={<TLSIcon />} name="view" value="/tlslist" onClick={toRoute('/tlslist')}>
                            TLS
                        </MenuItemRadio>
                        <MenuItemRadio icon={<DNSIcon />} name="view" value="/dns" onClick={toRoute('/dns')}>
                            DNS
                        </MenuItemRadio>
                    </MenuList>
                </MenuPopover>
            </Menu>
            <Menu openOnHover hoverDelay={0} >
                <MenuTrigger>
                    <Button shape="square" size="small" appearance="transparent" icon={<BookInformationRegular />}>About</Button>
                </MenuTrigger>
                <MenuPopover>
                    <MenuList>
                        <MenuItem icon={<PropertyIcon />} onClick={() => { setOpen(true) }}>
                            Properties
                        </MenuItem>
                        <MenuItem>
                            {version}
                        </MenuItem>
                        {
                            debug ? <MenuItem icon={<DNSIcon />} onClick={toRoute('/debug')}>
                                DEBUG
                            </MenuItem> : null
                        }
                    </MenuList>
                </MenuPopover>
            </Menu>
            <StatusBar />
            {/* <div style={{marginLeft: 'auto', alignSelf: 'center'}}>1123</div> */}
        </div>
        <Property open={open} setOpen={setOpen} />
    </>
    );
};