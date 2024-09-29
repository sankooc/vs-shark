
import React, { useEffect, useRef } from 'react';
import { MeterGroup } from 'primereact/metergroup';
import { Card } from 'primereact/card';
import { Button } from 'primereact/button';
import { IContextInfo } from '../../common';

class Props {
    data: IContextInfo;
  }
export default function Head(props: Props) {
    const meter = (props, attr) => <span {...attr} key={props.index} style={{ background: `linear-gradient(to right, ${props.color1}, ${props.color2})`, width: props.percentage + '%' }} />;

    const labelList = ({ values }) => (
        <>
            {values.map((item, index) => (
                <Card className="flex-1" key={index}>
                    <div className="flex justify-content-between gap-10">
                        <div className="flex flex-column gap-1">
                            <span className="text-secondary text-sm">{item.label}</span>
                            <span className="font-bold text-lg">{item.value}</span>
                        </div>
                    </div>
                </Card>
            ))}
        </>
    );

    const meta = props.data;
    const values = [
        { label: 'FileType', value: meta.file_type},
        { label: 'Time', value: meta.end_time - meta.start_time},
        { label: 'Frames', value: meta.frame_count},
        { label: 'TCP', value: meta.tcp_count},
        { label: 'DNS Record', value: meta.dns_count},
        { label: 'HTTP Conn', value: meta.http_count}
    ];

    return (
        <div className="card flex flex-nowrap justify-content-center gap-3 w-full overview-head">
            {labelList({values})}
        </div>
    )
}
        