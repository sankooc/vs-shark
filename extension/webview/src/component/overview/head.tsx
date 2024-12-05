
import React from 'react';
import { Card } from 'primereact/card';
import { IContextInfo } from '../../common';

class Props {
    data: IContextInfo;
}
export default function Head(props: Props) {
    const labelList = ({ values }) => (
        <>
            {values.map((item, index) => (
                <Card className="flex-1" key={index}>
                    <div className="flex gap-3 w-full">
                        <span className="w-4rem h-4rem border-circle inline-flex justify-content-center align-items-center text-center">
                            <i className={item.icon} style={{ fontSize: "2.4rem", color: item.iconColor }} />
                        </span>
                        <div className="flex flex-column gap-1 flex-grow-1">
                            <span className="font-bold" style={{ textAlign: "right", fontSize: "1.6rem" }}>{item.value}</span>
                            <span className="text-secondary" style={{ textAlign: "right", fontSize: "1.5rem" }}>{item.label}</span>
                        </div>
                    </div>
                </Card>
            ))}
        </>
    );

    let duration = 0;
    const meta = props.data;
    if (meta.end_time > meta.start_time) {
        duration = meta.end_time - meta.start_time;
    }
    const values = [
        { label: 'FileType', value: meta.file_type, icon: 'pi pi-file-check',iconColor: "#98971a" },
        // { label: 'Time', value: new String(duration).replace(/(\d)(?=(\d\d\d)+(?!\d))/g, "$1,") + " micro", icon: 'pi pi-history' },
        { label: 'Frames', value: meta.frame_count, icon: 'pi pi-list', iconColor: "#689d6a"},
        { label: 'TCP', value: meta.tcp_count, icon: 'pi pi-arrow-right-arrow-left',iconColor: "#d79921" },
        { label: 'DNS Record', value: meta.dns_count, icon: 'pi pi-address-book',iconColor: "#fb4934" },
        { label: 'HTTP Conn', value: meta.http_count, icon: 'pi pi-globe',iconColor: "#fabd2f" },
        { label: 'Cost', value: `${meta.cost}Ms`, icon: 'pi pi-hourglass',iconColor: "#94d2bd" }
    ];

    return (
        <div className="card flex flex-nowrap justify-content-center gap-3 w-full overview-head">
            {labelList({ values })}
        </div>
    )
}
