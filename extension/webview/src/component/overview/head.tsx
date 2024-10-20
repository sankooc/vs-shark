
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
                        {/* <span className="w-2rem h-2rem border-circle inline-flex justify-content-center align-items-center text-center" style={{ backgroundColor: "#60a5fa", color: '#ffffff' }}>
                            <i className={item.icon} />
                        </span> */}
                    </div>
                </Card>
            ))}
        </>
    );

    let duration = 0;
    const meta = props.data;
    if(meta.end_time > meta.start_time){
        duration = meta.end_time - meta.start_time;
    }
    const values = [
        { label: 'FileType', value: meta.file_type, icon: 'pi pi-inbox'},
        { label: 'Time', value: new String(duration).replace(/(\d)(?=(\d\d\d)+(?!\d))/g, "$1,") + " micro", icon: 'pi pi-inbox'},
        { label: 'Frames', value: meta.frame_count, icon: 'pi pi-inbox'},
        { label: 'TCP', value: meta.tcp_count, icon: 'pi pi-inbox'},
        { label: 'DNS Record', value: meta.dns_count, icon: 'pi pi-inbox'},
        { label: 'HTTP Conn', value: meta.http_count, icon: 'pi pi-inbox'},
        { label: 'Cost', value: `${meta.cost}Ms`, icon: 'pi pi-inbox'}
    ];

    return (
        <div className="card flex flex-nowrap justify-content-center gap-3 w-full overview-head">
            {labelList({values})}
        </div>
    )
}
        