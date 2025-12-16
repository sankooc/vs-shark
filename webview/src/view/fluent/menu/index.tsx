import type { JSXElement } from "@fluentui/react-components";
import {
    MenuList,
    MenuItem,
    MenuPopover,
    MenuTrigger,
    Menu,
    Button,
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
} from "@fluentui/react-icons";
import { usePcapStore } from "../../context";
import { PcapState } from "../../../share/common";
import { ConversationIcon, DNSIcon, FrameIcon, HttpIcon, OverviewIcon, TLSIcon, UDPTabIcon } from "../common";
import { useNavigate } from "react-router";
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

export default function MultilineItems(): JSXElement {
    const navigate = useNavigate();
    const [open, setOpen] = useState<boolean>(false)
    const info = usePcapStore((state: PcapState) => state.fileinfo);
    let canSelectFile = false;
    if (import.meta.env) {
        canSelectFile = import.meta.env.VITE_BUILD_ALL === 'true' || import.meta.env.VITE_BUILD_GUI === 'true';
    }
    const toRoute = (path: string) => {
        return () => {
            navigate(path);
        }
    }
    const closeFile = usePcapStore((state: PcapState) => state.closeFile);
    const openFile = usePcapStore((state: PcapState) => state.openFile);
    
    if(!info){
        if(!canSelectFile){
            return <></>
        }
        return (
            <div className="flex flex-row items-center" style={{ borderBottom: '1px solid #FFD', padding: '3px 2px'}}>
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
        <div className="flex flex-row items-center" style={{ borderBottom: '1px solid #FFD', padding: '3px 5px'}}>
            {canSelectFile?<Menu openOnHover hoverDelay={0}>
                <MenuTrigger>
                    <Button shape="square" size="small" appearance="transparent" icon={<DocIcon />}>File</Button>
                </MenuTrigger>
                <MenuPopover>
                    <MenuList>
                        <MenuItem subText="Select pcap file" icon={<OpenIcon />} onClick={openFile} disabled >
                            Open
                        </MenuItem>
                        <MenuItem subText="Close File" icon={<CloseIcon />} onClick={closeFile}>
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
                    <MenuList>
                        <MenuItem icon={<OverviewIcon />} onClick={toRoute('/overview')}>
                            Overview
                        </MenuItem>
                        <MenuItem icon={<FrameIcon />} onClick={toRoute('/')}>
                            Frames
                        </MenuItem>
                    </MenuList>
                </MenuPopover>
            </Menu>
            <Menu openOnHover hoverDelay={0}>
                <MenuTrigger>
                    <Button shape="square" size="small" appearance="transparent" icon={<StaIcon />}>Statistic</Button>
                </MenuTrigger>
                <MenuPopover>
                    <MenuList>
                        <MenuItem icon={<PropertyIcon />} onClick={() => {setOpen(true)}}>
                            Properties
                        </MenuItem>
                        <MenuItem icon={<ConversationIcon />} onClick={toRoute('/conversations')}>
                            TCP
                        </MenuItem>
                        <MenuItem icon={<UDPTabIcon />} onClick={toRoute('/udp')}>
                            UDP
                        </MenuItem>
                        <MenuItem icon={<HttpIcon />} onClick={toRoute('/https')}>
                            HTTP
                        </MenuItem>
                        <MenuItem icon={<TLSIcon />} onClick={toRoute('/tlslist')}>
                            TLS
                        </MenuItem>
                        <MenuItem icon={<DNSIcon />} onClick={toRoute('/dns')}>
                            DNS
                        </MenuItem>
                    </MenuList>
                </MenuPopover>
            </Menu>
            <StatusBar/>
            {/* <div style={{marginLeft: 'auto', alignSelf: 'center'}}>1123</div> */}
        </div>
        <Property open={open} setOpen={setOpen}/>
        </>
    );
};