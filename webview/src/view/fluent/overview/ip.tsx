import { Card } from "@fluentui/react-components";
import ReactECharts from 'echarts-for-react';
import { usePcapStore } from "../../context";
import { useEffect, useState } from "react";
import { ICounterItem } from "../../../share/gen";


const Meta = {
    type: 'pie',
    radius: [20, 100],
    // roseType: 'area',
    itemStyle: {
        borderRadius: 2
    },
};


export default function Component() {
    const stat = usePcapStore((state) => state.stat);
    const [data, setData] = useState<ICounterItem[]>([]);
    useEffect(() => {
        stat({ field: 'ip_address' }).then(setData);
    }, []);
    const series = [];
    const types = [];
    const versions = [];
    for (const d of data) {
        if (d.key === 'IPv4' || d.key === 'IPv6') {
            versions.push(d);
        } else {
            types.push(d);
        }
    }
    if (types.length) {
        series.push(
            {
                ...Meta,
                name: 'Type',
                center: ['25%', '55%'],
                data: types.map(item => ({ value: item.count, name: item.key }))
            });
    }
    if (versions.length) {
        series.push(
            {
                ...Meta,
                name: 'Version',
                center: ['75%', '55%'],
                data: versions.map(item => ({ value: item.count, name: item.key }))
            });
    }
    if (!series.length) {
        return <></>
    }
    const option = {
        title: {
            text: 'IP Address',
            left: 'center'
        },
        tooltip: {
            trigger: 'item'
        },
        legend: {
            show: false,
            left: 'center',
            top: 'bottom',
        },
        series
    };
    return <Card className="trim-card" style={{ minHeight: '280px' }} orientation="vertical">
        <ReactECharts option={option} style={{ width: '100%' }} theme="dark" />
    </Card>
}