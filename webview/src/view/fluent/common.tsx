import {
    Breadcrumb,
    BreadcrumbItem,
    BreadcrumbDivider,
    BreadcrumbButton,
    Slot,
} from "@fluentui/react-components";
import { bundleIcon, ClipboardBulletListRtlFilled, ClipboardBulletListRtlRegular, FormSparkle20Filled, FormSparkle20Regular, GlobeColor, LockClosedKeyRegular, PlugConnected20Filled, PlugConnected20Regular, QuestionFilled, ShieldLock20Filled, ShieldLock20Regular, TextboxRotate9020Filled, TextboxRotate9020Regular, TriangleLeft20Filled, TriangleLeft20Regular, TriangleRight20Filled, TriangleRight20Regular } from "@fluentui/react-icons";
import React from "react";

import { useNavigate } from "react-router";

interface ConnectProp {
    items: {
        path?: string;
        icon?: Slot<'span'>;
        name: string;
    }[];
}

export function BreadItem(props: ConnectProp) {
    const navigate = useNavigate();
    return (<Breadcrumb aria-label="Breadcrumb" style={{padding: "5px 0px", margin: 0}}>
        {
            props.items.map((item, index) => {
                return <React.Fragment key={"bi" + index}>
                    <BreadcrumbItem >
                        {index < props.items.length - 1 ? <BreadcrumbButton icon={item.icon}  onClick={() => {
                            navigate(item.path || "/");
                        }}>{item.name}</BreadcrumbButton> : <BreadcrumbButton icon={item.icon} >{item.name}</BreadcrumbButton> }
                    </BreadcrumbItem>
                    {index < props.items.length - 1 && <BreadcrumbDivider />}
                </React.Fragment>
            })
        }
    </Breadcrumb>);
}


export const ConversationIcon = bundleIcon(FormSparkle20Filled, FormSparkle20Regular);
export const FrameIcon = bundleIcon(TextboxRotate9020Filled, TextboxRotate9020Regular);
export const HttpIcon = bundleIcon(PlugConnected20Filled, PlugConnected20Regular);
export const TLSIcon = bundleIcon(ShieldLock20Filled, ShieldLock20Regular)
export const NextIcon = bundleIcon(TriangleRight20Filled, TriangleRight20Regular);
export const PrevIcon = bundleIcon(TriangleLeft20Filled, TriangleLeft20Regular);

export const DetailIcon = bundleIcon(ClipboardBulletListRtlFilled, ClipboardBulletListRtlRegular)

// eslint-disable-next-line react-refresh/only-export-components
export function protocolText(text: string): React.ReactNode {
    if (!text){
        return <QuestionFilled />
    }
    const _text = text.trim();
    switch (_text) {
        case 'http': {
            return <div className="flex align-items-center gap-1"><GlobeColor /> http</div>
        }
        case 'tls': {
            return <div className="flex align-items-center gap-1"><LockClosedKeyRegular /> tls</div>
        }
    }
    return <>{text}</>
}