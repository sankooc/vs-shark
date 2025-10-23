
import { Body1, Caption1, Card, CardHeader } from '@fluentui/react-components';
import ReactECharts from 'echarts-for-react';

export default function Component() {
    const hours = [];
    for(let i = 1;i<=20; i +=1){
        hours.push(`${i * 5}%`);
    }
    // prettier-ignore
    const days = [
        'tls', 'icmp', 'dns',
        'ssdp', 'http', 'tcp', 'udp'
    ];
    // prettier-ignore
    const data = [[0, 0, 1340], [1, 0, 100], [0, 1, 100], [0, 4, 1], [0, 3, 0]];
    const padding = '10px';
    //https://echarts.apache.org/examples/zh/editor.html?c=matrix-sparkline&theme=dark
    const option = {
        // backgroundColor: '#1d2021', 
        title: {
            text: 'Protocol distribution heatmap',
            left: 'center'
        },
        padding: 0,
        tooltip: {
            position: 'top'
        },
        grid: {
            left: padding,
            right: padding,
            top: '50px',
            bottom: padding,
            height: '200px',
        },
        xAxis: {
            type: 'category',
            data: hours,
            splitArea: {
                show: true
            }
        },
        yAxis: {
            type: 'category',
            data: days,
            offset: 1,
            splitArea: {
                show: true
            }
        },
        visualMap: {
            calculable: true,
            show: false,
            // inRange: {
            //     color: ['#fbf1c7', '#fb4934']
            // },
            orient: 'horizontal',
            left: 'center',
            bottom: '1%'
        },
        series: [
            {
                // name: 'frame',
                type: 'heatmap',
                data: data,
                label: {
                    show: true
                }
            }
        ]
    };
    return <Card>
        {/* <CardHeader
        // image={<ChartPerson28Filled />}
        header={
          <Body1 style={{paddingLeft: '10px'}}>
            Packet heatmap
          </Body1>
        }/> */}
        <ReactECharts option={option} style={{width: '100%'}} theme="dark" className="overview-frames"/>
    </Card>
}