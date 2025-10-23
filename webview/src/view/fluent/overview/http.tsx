import { Card } from "@fluentui/react-components";
import ReactECharts from 'echarts-for-react';


function MethodComponent() {
    const option = {
        title: {
            text: 'HTTP',
            left: 'center'
        },
        legend: {
            left: 'center',
            top: 'bottom',
            data: [
                'rose1',
                'rose2',
                'rose3',
                'rose4',
                'rose5',
                'rose6',
                'rose7',
                'rose8'
            ]
        },
        series: [
            {
                // name: 'Area Mode',
                type: 'pie',
                radius: [20, 100],
                roseType: 'area',
                itemStyle: {
                    borderRadius: 6
                },
                center: ['16.6%', '55%'],
                data: [
                    { value: 30, name: 'GET' },
                    { value: 28, name: 'POST' },
                    { value: 26, name: 'PUT' },
                    { value: 24, name: 'DELETE' },
                ]
            },
            {
                name: 'Area Mode3',
                type: 'pie',
                radius: [20, 100],
                roseType: 'area',
                // itemStyle: {
                //     borderRadius: 5
                // },
                center: ['50%', '55%'],
                data: [
                    { value: 30, name: '????' },
                    { value: 8, name: '1XX' },
                    { value: 12, name: '2XX' },
                    { value: 34, name: '3XX' },
                    { value: 12, name: '4XX' },
                    { value: 20, name: '5XX' },
                ]
            },
            {
                name: 'Area Mode2',
                type: 'pie',
                radius: [20, 100],
                center: ['83.3%', '55%'],
                roseType: 'area',
                itemStyle: {
                    borderRadius: 5
                },
                data: [
                    { value: 30, name: 'application/javascript' },
                    { value: 28, name: 'css' },
                    { value: 26, name: 'image/png' },
                    { value: 24, name: 'image/jpeg' },
                    { value: 22, name: '???' },
                ]
            }
        ]
    };
    return <ReactECharts option={option} style={{ width: '100%' }} theme="dark" />
}




export default function Component() {
    return <Card className="trim-card" style={{minHeight: '280px'}} orientation="horizontal">
        <MethodComponent />
        {/* <MethodComponent />
        <MethodComponent /> */}
    </Card>
}