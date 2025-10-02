import { HttpMessageWrap } from "../../../../share/common";

import HexView from './hexview';
import PlainText from './plain';
import Empty from './empty';
import { SelectTabData, SelectTabEvent, Tab, TabList, TabValue } from "@fluentui/react-components";
import { bundleIcon, CalendarAgendaFilled, CalendarAgendaRegular } from "@fluentui/react-icons";
import { useState } from "react";
type ContentProps = {
    hmw: HttpMessageWrap;
}
const CalendarAgenda = bundleIcon(CalendarAgendaFilled, CalendarAgendaRegular);
const renderContent = (hmw: HttpMessageWrap) => {
    const ds = !!hmw.parsed_content ? 'preview' : 'raw';
    const [selectedValue, setSelectedValue] = useState<TabValue>(ds);
    const onTabSelect = (_: SelectTabEvent, data: SelectTabData) => {
        setSelectedValue(data.value);
    };
    const inContent = () => {
        if (selectedValue === 'preview') {
            return <PlainText text={hmw!.parsed_content!} mime={hmw.mime} />
        } else {
            return <HexView data={hmw.raw || new Uint8Array()} maxLength={1024 * 1024} />
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
    if (hmw.parsed_content || hmw.raw?.length){
        return renderContent(props.hmw);
    }
    return <Empty/>;
}