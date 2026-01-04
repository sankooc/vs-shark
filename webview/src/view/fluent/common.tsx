/* eslint-disable @typescript-eslint/no-unused-expressions */
import {
    Breadcrumb,
    BreadcrumbItem,
    BreadcrumbDivider,
    BreadcrumbButton,
    Slot,
    Label,
    Select,
    Combobox,
    Option,
    Tooltip
} from "@fluentui/react-components";
import { BookGlobe20Filled, BookGlobe20Regular, bundleIcon, CallInboundRegular, CallOutboundRegular, ChartMultiple20Filled, ChartMultiple20Regular, CheckmarkSquareRegular, ClipboardBulletListRtlFilled, ClipboardBulletListRtlRegular, ClockRegular, DocumentBulletList20Filled, DocumentBulletList20Regular, DocumentGlobe20Regular, DocumentGlobeRegular, DocumentOnePageRegular, FormSparkle20Filled, FormSparkle20Regular, InfoRegular, LockClosedKey20Regular, MailTemplate20Filled, MailTemplate20Regular, MoreHorizontalFilled, PanelTopContractRegular, PanelTopExpandRegular, PlugConnected20Filled, PlugConnected20Regular, PresenceAvailableFilled, QuestionFilled, RecordStopFilled, ShieldLock20Filled, ShieldLock20Regular, ShieldQuestionRegular, SpinnerIosFilled, SpinnerIosRegular, TextboxRotate9020Filled, TextboxRotate9020Regular, TriangleLeft20Filled, TriangleLeft20Regular, TriangleRight20Filled, TriangleRight20Regular, WarningRegular } from "@fluentui/react-icons";
import React, { FormEvent, JSX, useEffect, useId, useState } from "react";

import { useNavigate } from "react-router";
import { usePcapStore } from "../context";
import { format_bytes_single_unit, PcapState } from "../../share/common";

interface ConnectProp {
    items: {
        path?: string;
        icon?: Slot<'span'>;
        name: string;
    }[];
}

export function BreadItem(props: ConnectProp) {
    const navigate = useNavigate();
    return (<Breadcrumb aria-label="Breadcrumb" style={{ padding: "1px 0px", margin: 0 }}>
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

export const SpinIcon = bundleIcon(SpinnerIosFilled, SpinnerIosRegular)

export const DetailIcon = bundleIcon(ClipboardBulletListRtlFilled, ClipboardBulletListRtlRegular);

export const HttpHeaderIcon = () => <DocumentOnePageRegular />
export const TabHttpHead = () => <DocumentGlobeRegular/>
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
            return <div className="flex align-items-center gap-1"><DocumentGlobe20Regular /> http</div>
        }
        case 'tls': {
            return <div className="flex align-items-center gap-1"><LockClosedKey20Regular /> tls</div>
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

// ...existing code...
export function IPSelector(props: SelectorProps) {
    const selectId = useId();
    const stat = usePcapStore((state) => state.stat);

    const [ip4s, setIp4s] = useState<string[]>([]);
    const [ip6s, setIp6s] = useState<string[]>([]);
    const [types, setTypes] = useState<string[]>([NoneOption]);
    const [type, setType] = useState<string>(NoneOption);

    const [query, setQuery] = useState<string>("");

    const source = type === IPV4Option ? ip4s : (type === IPV6Option ? ip6s : []);
    const [suggestions, setSuggestions] = useState<string[]>([]);
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
    useEffect(() => {
        setQuery("");
        setSuggestions(source.slice(0, 100));
    }, [type]);

    const opt1 = {
        disabled: ip4s.length === 0 && ip6s.length === 0,
    }

    const opt2 = {
        disabled: type === NoneOption,
        id: selectId,
    }

    return <>
        <Label size="small" htmlFor={selectId} style={{ paddingInlineEnd: "5px" }}>IP</Label>
        <Select size='small' {...opt1} onChange={(_: any, val: any) => { setType(val.value) }} value={type} >
            {types.map((t) => <option key={t}>{t}</option>)}
        </Select>
        <Combobox
            size="small"
            placeholder="Input IPAddress"
            {...opt2}
            value={query}
            onInput={(data: FormEvent<any>) => {
                const v = (data.target as HTMLInputElement).value;
                setQuery(v);
                if (v && v.length > 2 && type !== NoneOption) {
                    const list = source.filter(ip => ip.includes(v)).slice(0, 50);
                    setSuggestions(list.slice(0, 100));
                } else {
                    const data = source.slice(0, 100);
                    setSuggestions(data);
                }
            }}
            onOptionSelect={(_, data) => {
                const v = data.optionValue;
                setQuery(v || '');
                if (v) {
                    props.onSelect && props.onSelect(v);
                }
            }}
        >
            {suggestions.map(s => <Option key={s} value={s}>{s}</Option>)}
        </Combobox>
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
}

function getFileName(path: string) {
    if (!path) {
        return '';
    }
    const normalizedPath = path.replace(/\\/g, '/');
    const lastSlashIndex = normalizedPath.lastIndexOf('/');
    const fileName = normalizedPath.substring(lastSlashIndex + 1);
    return fileName;
}
export function StatusBar() {
    const info = usePcapStore((state: PcapState) => state.fileinfo);
    const progress = usePcapStore((state: PcapState) => state.progress);
    let filename = "";
    let filepath = '';
    let total = 0;
    let _total = '';
    let cursor = 0;
    let percent = 1;
    try {
        if (info) {
            filepath = info.name;
            // filepath = '/shue/acd/pdc.png';
            total = info.size;
            filename = getFileName(filepath);
        }
        if (progress) {
            total = Math.max(total, progress.total);
            cursor = progress.cursor;
        }
        if (total > 0) {
            _total = format_bytes_single_unit(total);
            percent = cursor / total;
        }
    } catch (e) {
        console.error(e);
    }

    const style = {
        marginLeft: 'auto',
        alignSelf: 'center',
    };
    const incomplete = percent < 0.99

    const color = incomplete ? '#689d6a' : '#458588';
    const preIcon = () => {
        if (incomplete) {
            return <RecordStopFilled style={{ color }} />
        } else {
            return <PresenceAvailableFilled style={{ color }} />
        }
    }

    return <div className="status-bar flex flex-row align-items-center page-status" style={style}>
        
        {
            incomplete ? `${Math.round(percent * 100)}%` : null
        }
        {preIcon()}
        {
            filename ? <Tooltip content={filepath} relationship="label">
                <span style={{ fontWeight: 'bold',color, padding: '0 5px', maxWidth: '400px', textWrap: 'wrap' }}> {filename}</span>
            </Tooltip> : null
        }
        {
            _total? <span>({_total})</span> : null
        }
        
    </div>
}