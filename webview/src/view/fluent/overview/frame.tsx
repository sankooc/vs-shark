
import { Card } from '@fluentui/react-components';
import ReactECharts from 'echarts-for-react';
import { useStore } from '../../store';
import { useEffect, useState } from 'react';
import { ILineData } from '../../../share/gen';
import dayjs from 'dayjs';
import { format_bytes_single_unit } from '../../../share/common';


export default function Component() {
    const stat = useStore((state) => state.stat);
    const [line, setLineData] = useState<ILineData | null>(null);
    useEffect(() => {
        stat({ field: 'frame' }).then((rs) => { setLineData(rs as ILineData) });
    }, []);
    if (!line || !line.x_axis) {
        return <></>
    }
    const x_axis = line.x_axis.map(t => dayjs(Math.floor(t / 1000)).format('HH:mm:ss'));

    const padding = 70;

    const series = line.data.map((d, inx) => {
        return {
            name: line.y_axis[inx],
            smooth: true,
            type: "line",
            areaStyle: null,
            data: d
        };
    });

    const option = {
        title: {
            text: 'Network Traffic'
        },

        grid: {
            left: padding,
            right: padding,
            top: '50',
            // bottom: '50px',
            // height: '200px',
            containLabel: true,
        },
        tooltip: {
            trigger: 'axis',
            valueFormatter: (value: any) => format_bytes_single_unit(value),
            axisPointer: {
                type: 'cross',
                label: {
                    backgroundColor: '#6a7985'
                }
            }
        },
        // legend: {
        //     data: line.y_axis
        // },
        xAxis: [
            {
                type: 'category',
                boundaryGap: false,
                data: x_axis
            }
        ],
        yAxis: [
            {
                type: 'value'
            }
        ],
        series,
        dataZoom: [
            {
                type: 'slider',
                xAxisIndex: 'all',
                left: '2%',
                right: '2%',
                throttle: 120
            },
            {
                type: 'inside',
                xAxisIndex: 'all',
                throttle: 120
            }
        ]
    };
    return <Card style={{padding: 0}}>
        <ReactECharts option={option} style={{ width: '100%' }} theme="dark" className="overview-frames" />
    </Card>
}