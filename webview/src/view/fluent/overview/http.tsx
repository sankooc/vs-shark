import { Card } from "@fluentui/react-components";
import ReactECharts from 'echarts-for-react';
import { useStore } from "../../store";
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
    const stat = useStore((state) => state.stat);
    const [data, setData] = useState<ICounterItem[][]>([]);
    useEffect(() => {
        stat({ field: 'http_data' }).then(setData);
    }, []);
    const series = [];
    if (data && data.length > 2) {
        if (data[0].length) {
            series.push(
                {
                    ...Meta,
                    name: 'Method',
                    center: ['16.6%', '55%'],
                    data: data[0].map(item => ({ value: item.count, name: item.key }))
                });
        }
        if (data[1].length) {
            series.push(
                {
                    ...Meta,
                    name: 'Status',
                    center: ['50%', '55%'],
                    data: data[1].map(item => ({ value: item.count, name: item.key }))
                });
        }
        if (data[2].length) {
            series.push(
                {
                    ...Meta,
                    name: 'Type',
                    center: ['83.3%', '55%'],
                    data: data[2].map(item => ({ value: item.count, name: item.key }))
                });
        }
    }
    if (!series.length) {
        return <></>
    }
    const option = {
        title: {
            text: 'HTTP Analysis',
            left: 'center'
        },
        legend: {
            show: false,
            left: 'center',
            top: 'bottom',
        },
        series
    };
    return <Card className="trim-card" style={{ minHeight: '280px' }} orientation="horizontal">
        <ReactECharts option={option} style={{ width: '100%' }} theme="dark" />
    </Card>
}