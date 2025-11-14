/* eslint-disable @typescript-eslint/no-unused-expressions */
import {
    Breadcrumb,
    BreadcrumbItem,
    BreadcrumbDivider,
    BreadcrumbButton,
    Slot,
    Label,
    Select,
} from "@fluentui/react-components";
import { BookGlobe20Filled, BookGlobe20Regular, bundleIcon, CallInboundRegular, CallOutboundRegular, ChartMultiple20Filled, ChartMultiple20Regular, CheckmarkSquareRegular, ClipboardBulletListRtlFilled, ClipboardBulletListRtlRegular, ClockRegular, DocumentBulletList20Filled, DocumentBulletList20Regular, DocumentGlobeRegular, DocumentOnePageRegular, FormSparkle20Filled, FormSparkle20Regular, GlobeColor, InfoRegular, LockClosedKeyRegular, MailTemplate20Filled, MailTemplate20Regular, MoreHorizontalFilled, PanelTopContractRegular, PanelTopExpandRegular, PlugConnected20Filled, PlugConnected20Regular, QuestionFilled, ShieldLock20Filled, ShieldLock20Regular, ShieldQuestionRegular, TextboxRotate9020Filled, TextboxRotate9020Regular, TriangleLeft20Filled, TriangleLeft20Regular, TriangleRight20Filled, TriangleRight20Regular, WarningRegular } from "@fluentui/react-icons";
import React, { JSX, useEffect, useId, useState } from "react";

import { useNavigate } from "react-router";
import { usePcapStore } from "../../share/context";

interface ConnectProp {
    items: {
        path?: string;
        icon?: Slot<'span'>;
        name: string;
    }[];
}

export function BreadItem(props: ConnectProp) {
    const navigate = useNavigate();
    return (<Breadcrumb aria-label="Breadcrumb" style={{ padding: "5px 0px", margin: 0 }}>
        {
            props.items.map((item, index) => {
                return <React.Fragment key={"bi" + index}>
                    <BreadcrumbItem >
                        {index < props.items.length - 1 ? <BreadcrumbButton icon={item.icon} onClick={() => {
                            navigate(item.path || "/");
                        }}>{item.name}</BreadcrumbButton> : <BreadcrumbButton icon={item.icon} >{item.name}</BreadcrumbButton>}
                    </BreadcrumbItem>
                    {index < props.items.length - 1 && <BreadcrumbDivider />}
                </React.Fragment>
            })
        }
    </Breadcrumb>);
}


export const StatisticTabIcon = bundleIcon(DocumentBulletList20Filled, DocumentBulletList20Regular)
export const ConversationIcon = bundleIcon(FormSparkle20Filled, FormSparkle20Regular);
export const UDPTabIcon = bundleIcon(MailTemplate20Filled, MailTemplate20Regular);
export const OverviewIcon = bundleIcon(ChartMultiple20Filled, ChartMultiple20Regular);
export const FrameIcon = bundleIcon(TextboxRotate9020Filled, TextboxRotate9020Regular);
export const HttpIcon = bundleIcon(PlugConnected20Filled, PlugConnected20Regular);
export const TLSIcon = bundleIcon(ShieldLock20Filled, ShieldLock20Regular)
export const DNSIcon = bundleIcon(BookGlobe20Filled, BookGlobe20Regular)
export const NextIcon = bundleIcon(TriangleRight20Filled, TriangleRight20Regular);
export const PrevIcon = bundleIcon(TriangleLeft20Filled, TriangleLeft20Regular);

export const DetailIcon = bundleIcon(ClipboardBulletListRtlFilled, ClipboardBulletListRtlRegular);

export const HttpHeaderIcon = () => <DocumentOnePageRegular />
export const TabHttpHead = () => <DocumentGlobeRegular />
export const TabHttpReq = () => <PanelTopContractRegular />
export const TabHttpRes = () => <PanelTopExpandRegular />

export const ActionInfoIcon = () => <InfoRegular />
export const ActionMoreIcon = () => <MoreHorizontalFilled />;
export const TabHttpRequest = () => <CallInboundRegular />;
export const TabHttpResponse = () => <CallOutboundRegular />;

export const TimeIcon = () => <><ClockRegular />Time</>;


// eslint-disable-next-line react-refresh/only-export-components
export function protocolText(text: string): React.ReactNode {
    if (!text) {
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

type SelectorProps = {
    onSelect?: (value: string) => void;
}

const NoneOption = "ANY";
const IPV4Option = "ipv4";
const IPV6Option = "ipv6";

export function IPSelector(props: SelectorProps) {
    const selectId = useId();
    const stat = usePcapStore((state) => state.stat);

    const [ip4s, setIp4s] = useState<string[]>([]);
    const [ip6s, setIp6s] = useState<string[]>([]);
    const [types, setTypes] = useState<string[]>([NoneOption]);
    const [type, setType] = useState<string>(NoneOption);
    useEffect(() => {
        props.onSelect && props.onSelect(NoneOption);
    }, [type]);
    useEffect(() => {
        stat({ field: 'ip4' }).then((rs) => {
            if (rs.length) {
                if (types.indexOf(IPV4Option) < 0) {
                    types.push(IPV4Option);
                }
                setTypes(types.slice(0));
                setIp4s(rs.map((r: any) => r.key));
            }
        });
        stat({ field: 'ip6' }).then((rs) => {
            if (rs.length) {
                if (types.indexOf(IPV6Option) < 0) {
                    types.push(IPV6Option);
                }
                setTypes(types.slice(0));
                setIp6s(rs.map((r: any) => r.key));
            }
        });
    }, []);
    const opt1 = {
        disabled: ip4s.length === 0 && ip6s.length === 0,
    }

    const opt2 = {
        disabled: type === NoneOption,
        id: selectId,
    }

    let options: JSX.Element[] = [];
    switch (type) {
        case IPV6Option: {
            options = ip6s.map((h) => {
                return <option key={h}>{h}</option>
            });
            break;
        }
        case IPV4Option: {
            options = ip4s.map((h) => {
                return <option key={h}>{h}</option>
            });
            break;
        }
    }
    return <>
        <Label size="small" htmlFor={selectId} style={{ paddingInlineEnd: "5px" }}>IP</Label>
        <Select size='small' {...opt1} onChange={(_: any, val: any) => { setType(val.value) }} value={type} >
            {
                types.map((t) => {
                    return <option key={t}>{t}</option>
                })
            }
        </Select>
        <Select size="small" {...opt2} onChange={(_: any, val: any) => { props.onSelect && props.onSelect(val.value) }} >
            <option>{NoneOption}</option>
            {options}
        </Select>
    </>;
}


export function infoLevel(level: string): [string, JSX.Element] {
    switch (level) {
        case 'info':
        case 'high': {
            return ['#b8bb26', <CheckmarkSquareRegular />]
        }
        case 'error':
        case 'low': {
            return ['#fb4934', <WarningRegular />]
        }
        default: {
            return ['#fabd2f', <ShieldQuestionRegular />];
        }
    }
    // return ['', <ShieldQuestionRegular />]
}