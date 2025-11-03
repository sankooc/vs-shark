import { IHttpDetail } from "../../../../share/common";

import HexView from './hexview';
import PlainText from './plain';
import Empty from './empty';
import { SelectTabData, SelectTabEvent, Tab, TabList, TabValue } from "@fluentui/react-components";
import { bundleIcon, CalendarAgendaFilled, CalendarAgendaRegular } from "@fluentui/react-icons";
import { useState } from "react";
import ImageView from "./image";
type ContentProps = {
    hmw: IHttpDetail;
}
const CalendarAgenda = bundleIcon(CalendarAgendaFilled, CalendarAgendaRegular);


const detectText = (mime: string): boolean => {
    return mime.indexOf("json") >= 0 || mime.indexOf("javascript") >= 0 || mime.indexOf("text") >= 0 || mime.indexOf("xml") >= 0 || mime.indexOf("html") >= 0;
}
const imageMimeTypes = [
    'image/jpeg',
    'image/png',
    'image/gif',
    'image/webp',
    'image/bmp',
    'image/tiff',
    'image/svg+xml',
    'image/x-icon',
    'image/apng',
    'image/avif'
];
const detectImage = (mime: string): boolean => {
    return imageMimeTypes.includes(mime);
}

const parseMime = (headers: string[]): string => {
    if(headers && headers.length){
        for(const _header of headers){
            const header = _header.toLocaleLowerCase();
            if(header.startsWith('content-type')) {
                return header.substring(13).trim();
            }
        }
    }
    return '';
}
const renderContent = (hmw: IHttpDetail) => {
    const ds = !!hmw.plaintext ? 'preview' : 'raw';
    const [selectedValue, setSelectedValue] = useState<TabValue>(ds);
    const onTabSelect = (_: SelectTabEvent, data: SelectTabData) => {
        setSelectedValue(data.value);
    };
    const _mime = parseMime(hmw.headers);
    const inContent = () => {
        if (selectedValue === 'preview') {
            if (detectText(_mime) && hmw!.plaintext) {
                return <PlainText text={hmw!.plaintext!} mime={hmw.content_type || 'plaintext'} />
            }
            if(detectImage(_mime) && hmw.raw?.length){
                return <ImageView raw={hmw.raw} mime={_mime}/>
            }
            return <div style={{margin: '20px auto'}}> No Preview </div>
        } else {
            return <HexView data={hmw.raw || new Uint8Array()} />
        }

    }
    return <div className="flex flex-row h-full w-full">
        <TabList vertical style={{ borderRight: '1px solid #ccc' }} selectedValue={selectedValue} onTabSelect={onTabSelect}>
            <Tab id="preview" icon={<CalendarAgenda />} value="preview">
                Preview
            </Tab>
            <Tab id="raw" icon={<CalendarAgenda />} value="raw">
                Hex
            </Tab>
        </TabList>
        {inContent()}
    </div>
}


export default function Component(props: ContentProps) {
    const { hmw } = props;
    if (hmw.plaintext || hmw.raw){
        return renderContent(props.hmw);
    }
    return <Empty/>;
}